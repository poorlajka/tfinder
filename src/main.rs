mod finder;
use viuer::{print_from_file, Config};

use std::fs::DirEntry;
use std::io::stdout;
use std::path::{Path, PathBuf};
use std::{io, time::Duration};

use crossterm::event::{self, poll, read, Event, MouseButton, MouseEvent, MouseEventKind};
use crossterm::ExecutableCommand;

fn handle_events(app: &mut App) -> io::Result<bool> {
    if poll(Duration::from_millis(100))? {
        match read()? {
            Event::FocusGained => (),
            Event::FocusLost => (),
            Event::Key(event) => match event.code {
                KeyCode::Up => app.items.previous(),
                KeyCode::Down => app.items.next(),
                KeyCode::Char('b') => match app.path.parent() {
                    Some(p_path) => {
                        app.path2 = app.path.clone();
                        app.path = p_path.to_path_buf();

                        load_folder(&mut app.items2, &mut app.files2, &app.path2);
                        load_folder(&mut app.items, &mut app.files, &app.path);
                        match app.items.state.selected() {
                            Some(index) => {
                                app.items2.state.select(Some(index));
                                app.items.unselect();
                            }
                            None => {}
                        }
                    }
                    None => {}
                },
                KeyCode::Char('q') => return Ok(true),
                _ => (),
            },
            Event::Mouse(event) => {
                let column = event.column;
                if column < app.pane_width {
                    handle_mouse_event(event, app, Pane::First);
                } else if column < app.pane_width * 2 {
                    handle_mouse_event(event, app, Pane::Second);
                }
            }
            Event::Paste(event) => (),
            Event::Paste(data) => (),
            Event::Resize(width, height) => (),
        }
    }
    Ok(false)
}

enum Pane {
    First,
    Second,
}

fn handle_mouse_event(event: MouseEvent, app: &mut App, pane: Pane) {
    let items = match pane {
        Pane::First => &mut app.items,
        Pane::Second => &mut app.items2,
    };
    let offset = items.state.offset_mut();

    match event.kind {
        MouseEventKind::ScrollDown => {
            *offset += 1;
        }
        MouseEventKind::ScrollUp => {
            if *offset > 0 {
                *offset -= 1;
            }
        }
        MouseEventKind::Down(click) => {
            if click == MouseButton::Left {
                match pane {
                    Pane::First => {
                        let index = event.row as usize + *offset;
                        if index < items.items.len() {
                            items.state.select(Some((index).into()));
                            let path = &app.files[index].path() as &Path;
                            load_folder(&mut app.items2, &mut app.files2, path);
                            app.items2.unselect();
                            app.path2 = path.to_path_buf();
                        }
                    }
                    Pane::Second => {
                        let index = event.row as usize + *offset;
                        if index < items.items.len() {
                            let path = &app.files2[index].path() as &Path;

                            if path.is_dir() {
                                match app.items2.state.selected() {
                                    Some(selected_index) => {
                                        if selected_index == index {
                                            app.path = app.path2.clone();
                                            app.path2 = path.to_path_buf();

                                            load_folder(
                                                &mut app.items2,
                                                &mut app.files2,
                                                &app.path2,
                                            );
                                            load_folder(&mut app.items, &mut app.files, &app.path);
                                            app.items.state.select(Some(index));
                                            app.items2.unselect();
                                            return;
                                        }
                                    }
                                    None => {}
                                }
                            }
                            app.items2.state.select(Some(index).into());
                            //items.state.select(Some((index).into()));
                        }
                    }
                }
            }
        }
        _ => (),
    }
}

use ratatui::{prelude::*, widgets::*};
use std::io::Stdout;

use anyhow::{Context, Result};
use crossterm::{
    event::KeyCode,
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

fn main() -> Result<()> {
    let mut terminal = setup_terminal().context("setup failed")?;
    run_app(&mut terminal, Duration::new(13, 13)).context("app loop failed")?;
    restore_terminal(&mut terminal).context("restore terminal failed")?;
    Ok(())
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>> {
    let mut stdout = stdout();
    stdout.execute(event::EnableMouseCapture);
    let mut stdout = io::stdout();
    enable_raw_mode().context("failed to enable raw mode")?;
    execute!(stdout, EnterAlternateScreen).context("unable to enter alternate screen")?;
    Terminal::new(CrosstermBackend::new(stdout)).context("creating terminal failed")
}

fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    disable_raw_mode().context("failed to disable raw mode")?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)
        .context("unable to switch to main screen")?;
    terminal.show_cursor().context("unable to show cursor")
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, tick_rate: Duration) -> Result<()> {
    //Create frame
    //Give frame to event handler
    let mut app = App {
        items: StatefulList::with_items(Vec::new()),
        items2: StatefulList::with_items(Vec::new()),
        files: Vec::new(),
        files2: Vec::new(),
        pane_width: 0,
        path: PathBuf::new(),
        path2: PathBuf::new(),
    };
    //app.items.state.select(Some(0));

    let path = Path::new("C:/Users/notso/");
    load_folder(&mut app.items, &mut app.files, &path);
    app.path = path.to_path_buf();

    loop {
        //Get list of elements to render
        //Render elements
        //let draw_fut = draw(terminal, app.clone());
        //let update_fut = update(terminal, &app);
        //join!(draw_fut, update_fut);
        let event_return = handle_events(&mut app);

        match event_return {
            Ok(exit) => {
                if exit {
                    break;
                }
            }
            Err(..) => (),
        }

        let _ = terminal.draw(|f| render_app(f, &mut app));
    }
    Ok(())
}

fn load_folder(
    items: &mut StatefulList<(String, usize)>,
    folders: &mut Vec<DirEntry>,
    path: &Path,
) {
    folders.clear();
    let _ = finder::get_folders(folders, path);

    items.items.clear();

    let mut remove = Vec::new();

    for (i, folder) in folders.iter().enumerate() {
        let mut item_name = String::new();

        match folder.file_type() {
            Ok(file_type) => {
                if file_type.is_dir() {
                    item_name += "  ";
                } else if file_type.is_file() {
                    item_name += "  ";
                } else {
                    //remove.push(i);
                    //continue;
                }
            }
            Err(..) => (),
        }
        match folder.file_name().to_str() {
            Some(name) => {
                if name.starts_with(".")
                        //Windows user settings things
                        || name.starts_with("NTUSER")
                        || name.starts_with("ntuser")
                {
                    //remove.push(i);
                    //continue;
                }
                item_name += name;
            }
            None => (),
        }
        items.items.push((item_name, i))
    }
    for i in remove {
        if i < folders.len() {
            folders.remove(i);
        }
    }
}

#[derive(Debug, Clone)]
struct StatefulList<T> {
    state: ListState,
    items: Vec<T>,
}

impl<T> StatefulList<T> {
    fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn unselect(&mut self) {
        self.state.select(None);
    }
}

struct App {
    items: StatefulList<(String, usize)>,
    items2: StatefulList<(String, usize)>,
    files: Vec<DirEntry>,
    files2: Vec<DirEntry>,
    pane_width: u16,
    path: PathBuf,
    path2: PathBuf,
}

/*
fn draw_items ()
terminal.draw(|frame| render_window(frame, &app))?;
*/

/*
* Program algorithm
* Create the window frame
* Create a thread for the event handler and give it the window frame
* loop in main thread:
*   Draw the window frame:
*       match each frame element (spawn and run in paralell)
*/

fn render_app<B: Backend>(frame: &mut Frame<B>, app: &mut App) {
    //Switch for every element in the frame

    let items: Vec<ListItem> = app
        .items
        .items
        .iter()
        .map(|i| {
            let mut lines = vec![Line::from(i.to_owned().0)];
            ListItem::new(lines).style(Style::default().fg(Color::White))
        })
        .collect();

    let items = List::new(items)
        .block(Block::default().borders(Borders::RIGHT))
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
                .bg(Color::LightBlue)
                .fg(Color::Black)
                .add_modifier(Modifier::ITALIC)
                .add_modifier(Modifier::BOLD),
        );

    let items2: Vec<ListItem> = app
        .items2
        .items
        .iter()
        .map(|i| {
            let mut lines = vec![Line::from(i.to_owned().0)];
            ListItem::new(lines).style(Style::default().fg(Color::White))
        })
        .collect();

    let items2 = List::new(items2)
        .block(Block::default().borders(Borders::RIGHT))
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
                .bg(Color::LightBlue)
                .fg(Color::Black)
                .add_modifier(Modifier::ITALIC)
                .add_modifier(Modifier::BOLD),
        );

    let size = frame.size();
    app.pane_width = size.width / 3;
    frame.render_stateful_widget(
        items,
        Rect::new(0, size.y, app.pane_width, size.height),
        &mut app.items.state,
    );

    frame.render_stateful_widget(
        items2,
        Rect::new(app.pane_width, size.y, app.pane_width, size.height),
        &mut app.items2.state,
    );

    //https://ascii-generator.site/

    let ascii_folder = "
░░░░░░░░░░░░░░░░░░░░
░▒▒▒▒▒░░░░░░░░░░░░░░
░▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒░
░▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒░
░▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒░
░▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒░
░▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒░
░▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒░
░░░░░░░░░░░░░░░░░░░░
░░░░░░░░░░░░░░░░░░░░
";

    let ascii_file = "
░░░▓▓▓▓▓▓▓▓▓▓░░░░░░░
░░░▓▒▒▒▒▒▒▒▒▓▓▓░░░░░
░░░▓▒▒▒▒▒▒▒▒▒▒▒▓░░░░
░░░▓▒▒▒▒▒▒▒▒▒▒▒▒▓░░░
░░░▓▒▒░░░░░░▒▒▒▒▓░░░
░░░▓▒▒▒▒▒▒▒▒▒▒▒▒▓░░░
░░░▓▒▒░░░░░░░░▒▒▓░░░
░░░▓▒▒▒▒▒▒▒▒▒▒▒▒▓░░░
░░░▓▒▒░░░░░░░░▒▒▓░░░
░░░▓▒▒▒▒▒▒▒▒▒▒▒▒▓░░░
";

    let pane1_state = &app.items.state;
    let pane2_state = &app.items2.state;
    match pane2_state.selected() {
        Some(index) => {
            let file = &app.files2[index];

            if let Some(file_name) = file.file_name().to_str() {
                let mut file_info = Paragraph::new("");
                if file.path().is_dir() {
                    file_info = Paragraph::new(
                        ascii_folder.to_owned() + "name: " + file_name + "\nfiletype: folder",
                    )
                    .fg(Color::LightBlue);
                } else if file.path().is_file() {
                    file_info = Paragraph::new(
                        ascii_file.to_owned() + "name: " + file_name + "\nfiletype: file",
                    )
                    .fg(Color::LightBlue);
                }

                frame.render_widget(
                    file_info,
                    Rect::new(
                        app.pane_width * 2 + app.pane_width / 3,
                        frame.size().height / 4,
                        23,
                        20,
                    ),
                );
            }
        }
        None => {
            if let Some(index) = pane1_state.selected() {
                let file = &app.files[index];
                if let Some(file_name) = file.file_name().to_str() {
                    let mut file_info = Paragraph::new("");
                    if file.path().is_dir() {
                        file_info = Paragraph::new(
                            ascii_folder.to_owned() + "name: " + file_name + "\nfiletype: folder",
                        )
                        .fg(Color::LightBlue);
                    } else if file.path().is_file() {
                        file_info = Paragraph::new(
                            ascii_file.to_owned() + "name: " + file_name + "\nfiletype: file",
                        )
                        .fg(Color::LightBlue);
                    }

                    frame.render_widget(
                        file_info,
                        Rect::new(
                            app.pane_width * 2 + app.pane_width / 3,
                            frame.size().height / 4,
                            23,
                            20,
                        ),
                    );
                }
            }
        }
    }
    /*
    let conf = Config {
        // set offset
        x: 20,
        y: 4,
        // set dimensions
        width: Some(40),
        height: Some(10),
        ..Default::default()
    };

    // starting from row 4 and column 20,
    // display `img.jpg` with dimensions 80x25 (in terminal cells)
    // note that the actual resolution in the terminal will be 80x50
    print_from_file("./src/discoball.png", &conf).expect("Image printing failed.");
    */
}
