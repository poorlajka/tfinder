mod app;
mod event_handler;
mod finder;
mod render;

use std::fs::DirEntry;
use std::path::{Path, PathBuf};
use std::io::{stdout, Stdout};
use std::time::Duration;
use anyhow::{Context, Result};

use ratatui::{prelude::*, widgets::*};
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
    event::{self, MouseEvent},
};


fn main() -> Result<()> {
    let mut terminal = setup_terminal()
        .context("setup failed")?;

    run_app(&mut terminal, Duration::new(13, 13))
        .context("app loop failed")?;

    restore_terminal(&mut terminal)
        .context("restore terminal failed")?;

    Ok(())
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>> {
    let mut stdout = stdout();
    let _ = stdout.execute(event::EnableMouseCapture);

    enable_raw_mode()
        .context("failed to enable raw mode")?;

    execute!(stdout, EnterAlternateScreen)
        .context("unable to enter alternate screen")?;

    Terminal::new(CrosstermBackend::new(stdout))
        .context("creating terminal failed")
}

fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    disable_raw_mode()
        .context("failed to disable raw mode")?;

    execute!(terminal.backend_mut(), LeaveAlternateScreen)
        .context("unable to switch to main screen")?;

    terminal.show_cursor()
        .context("unable to show cursor")
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, tick_rate: Duration) -> Result<()> {
    let path = Path::new("C:/Users/notso/");

    let mut app = app::App {
        first_pane: app::FilePane {
            height: 100,
            width: 40,
            files: app::StatefulList::with_items(Vec::new()),
            current_path: PathBuf::new(),
            entries: Vec::new(),
        },
        second_pane: app::FilePane {
            height: 100,
            width: 40,
            files: app::StatefulList::with_items(Vec::new()),
            current_path: PathBuf::new(),
            entries: Vec::new(),
        },
        path_trail: app::PathTrail {
            height: 2,
            width: 30,
            paths: Vec::new(),
            hovered_path: None,
        },
    };


    app.first_pane.load_path(path.to_path_buf());
    //app.path_trail.paths = vec![("hello".to_string(), PathBuf::new())];
    app.path_trail.load_path(&path.to_path_buf());
    app.first_pane.files.state.select(None);
    app.second_pane.files.state.select(None);

    loop {
        let _ = terminal.draw(|frame| render::render_app(frame, &mut app));

        let event_return = event_handler::handle_events(&mut app);

        match event_return {
            Ok(exit) => {
                if exit {
                    break;
                }
            }
            Err(..) => (),
        }
    }
    Ok(())
}
