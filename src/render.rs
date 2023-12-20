use ratatui::widgets::Paragraph;

use crate::app;
use crate::{
    Backend,
    Block,
    Borders,
    Color,
    Frame,
    Line,
    List,
    ListItem,
    Modifier,
    Rect,
    Style,
};

pub fn render_app<B: Backend>(frame: &mut Frame<B>, app: &mut app::App) {
    let (fwidth, fheight) = (frame.size().width, frame.size().height);

    app.second_pane.width = fwidth / 3;
    app.first_pane.width = fwidth / 3;

    let trail_height = app.path_trail.height;
    let first_pane_list = get_pane_list(&app.first_pane);
    let second_pane_list = get_pane_list(&app.second_pane);

    frame.render_stateful_widget(
        first_pane_list,
        Rect::new(0, trail_height, fwidth / 3, fheight - trail_height - 2),
        &mut app.first_pane.files.state,
    );

    frame.render_stateful_widget(
        second_pane_list,
        Rect::new(
            fwidth / 3,
            trail_height,
            fwidth / 3,
            fheight - trail_height - 2,
        ),
        &mut app.second_pane.files.state,
    );

    let mut paragraph = Paragraph::new("").style(Style::default().fg(Color::Blue));
    let mut pos: usize = 0;
    for (i, (name, _)) in app.path_trail.paths.iter_mut().enumerate() {
        let mut paragraph = Paragraph::new(name.to_string());
        if let Some(index) = app.path_trail.hovered_path {
            if index == i {
                paragraph = paragraph.style(Style::default().fg(Color::LightYellow));
            } else {
                paragraph = paragraph.style(Style::default().fg(Color::White));
            }
        } else {
            paragraph = paragraph.style(Style::default().fg(Color::White));
        }
        let mut width = name.len();
        if width > 20 {
            width = 20;
        }
        frame.render_widget(paragraph, Rect::new(pos as u16, 0, width as u16, 2));
        pos += width;
        frame.render_widget(Paragraph::new(" > "), Rect::new(pos as u16, 0, 3, 2));
        pos += 3;
    }

    let commands = Paragraph::new(
        "C Create    R Rename    M Move    D Delete    O Open    H Help    / Search    F fill",
    )
    .style(Style::default().fg(Color::White).bg(Color::Black));
    frame.render_widget(commands, Rect::new(0, fheight - 1, fwidth, 1));

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

    let pane2_state = &app.second_pane.files.state;
    match pane2_state.selected() {
        Some(index) => {
            let file = &app.second_pane.entries[index];

            if let Some(file_name) = file.file_name().to_str() {
                let mut file_info = Paragraph::new("");
                if file.path().is_dir() {
                    file_info = Paragraph::new(
                        ascii_folder.to_owned() + "name: " + file_name + "\ntype: folder",
                    )
                    .style(Style::default().fg(Color::White));
                } else if file.path().is_file() {
                    file_info = Paragraph::new(
                        ascii_file.to_owned() + "name: " + file_name + "\ntype: file",
                    )
                    .style(Style::default().fg(Color::White));
                }

                frame.render_widget(
                    file_info,
                    Rect::new((fwidth / 3) * 2 + (fwidth / 4) / 3, fheight / 4, 20, 20),
                );
            }
        }
        None => {
            if let Some(index) = app.first_pane.files.state.selected() {
                let file = &app.first_pane.entries[index];
                if let Some(file_name) = file.file_name().to_str() {
                    let mut file_info = Paragraph::new("");
                    if file.path().is_dir() {
                        file_info = Paragraph::new(
                            ascii_folder.to_owned() + "name: " + file_name + "\ntype: folder",
                        )
                        .style(Style::default().fg(Color::White));
                    } else if file.path().is_file() {
                        file_info = Paragraph::new(
                            ascii_file.to_owned() + "name: " + file_name + "\ntype: file",
                        )
                        .style(Style::default().fg(Color::White));
                    }

                    frame.render_widget(
                        file_info,
                        Rect::new((fwidth / 3) * 2 + (fwidth / 4) / 3, fheight / 4, 20, 20),
                    );
                }
            }
        }
    }

    /*
    get_path_trail(&app.path_trail, &mut paragraph);

    if let Some(index) = app.path_trail.hovered_path {
        frame.render_widget(
            paragraph.style(Style::default().fg(Color::LightBlue)),
            Rect::new(0, 0, fwidth, app.path_trail.height),
        );
    } else {
        frame.render_widget(
            paragraph.style(Style::default().fg(Color::White)),
            Rect::new(0, 0, fwidth, app.path_trail.height),
        );
    }
    */
}

fn get_pane_list(file_pane: &app::FilePane) -> List<'static> {
    let first_pane_files: Vec<ListItem> = file_pane
        .files
        .items
        .iter()
        .map(|i| {
            let lines = vec![Line::from(i.to_owned().0)];
            ListItem::new(lines).style(Style::default().fg(Color::White))
        })
        .collect();

    List::new(first_pane_files)
        .block(Block::default().borders(Borders::RIGHT))
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
                .bg(Color::LightYellow)
                .fg(Color::Black)
                .add_modifier(Modifier::ITALIC)
                .add_modifier(Modifier::BOLD),
        )
}

fn get_path_trail(path_trail: &app::PathTrail, paragraph: &mut Paragraph) {
    let mut dir_trail = String::new();
    let exclude_last = 0..path_trail.paths.len() - 1;

    for (name, _) in &path_trail.paths[exclude_last] {
        dir_trail += name;
        dir_trail += " > ";
    }
    if let Some((name, _)) = path_trail.paths.last() {
        dir_trail += name;
    }

    *paragraph = Paragraph::new(dir_trail)
}
