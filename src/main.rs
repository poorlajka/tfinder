mod app;
mod event_handler;
mod finder;
mod render;
mod config;


use std::fs::DirEntry;
use std::env;
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
    execute!(terminal.backend_mut(), LeaveAlternateScreen)
        .context("unable to switch to main screen")?;

    let mut stdout = stdout();
    let _ = stdout.execute(event::DisableMouseCapture);

    terminal.show_cursor()
        .context("unable to show cursor")?;

    disable_raw_mode()
        .context("failed to disable raw mode")
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, tick_rate: Duration) -> Result<()> {

    let render_config = config::parse();
    let mut app = app::App::new(&terminal.size()?, &env::current_dir()?);

    loop {
        let _ = terminal.draw(|frame| render::render_app(frame, &app, &render_config));

        let event_return = event_handler::handle_events(&mut app);

        match event_return {
            Ok(exit) => {
                if exit {
                    break;
                }
            }
            //THIS Is not so Vry good me no likey fucking hell change later
            Err(..) => {
                break;
            },
        }
    }
    Ok(())
}
