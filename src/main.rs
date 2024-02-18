mod app;
mod event_handler;
mod finder;
mod render;
mod command;
mod config;
mod prompt;
mod file_pane;
mod path_trail;
mod preview;

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

    //Let errors that would crash bubble to the top so the terminal can be reset properly
    //Errors which should't crash should never reach this point
    let e = if let Err(e) = run_app(&mut terminal, Duration::new(13, 13)) {
        Some(e)
    } 
    else {
        None
    };

    restore_terminal(&mut terminal)
        .context("restore terminal failed")?;

    if let Some(err) = e {
        println!("Error: {:?}", err);
    }
    

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

fn run_app<B: Backend>(terminal: &mut Terminal<B>, _tick_rate: Duration) -> Result<()> {

    let render_config = config::parse()?;
    let mut app = app::App::new(&terminal.size()?, &env::current_dir()?);

    loop {
        let _ = terminal.draw(|frame| render::render_app(frame, &mut app, &render_config));

        let quit = event_handler::handle_events(&mut app)?;

        if quit {
            break;
        }

    }
    Ok(())
}
