use crate::app;
use crate::Duration;

use std::path::PathBuf;

use anyhow::Result;
use crossterm::event::{
    KeyCode, poll, read, Event, MouseEvent, MouseEventKind, KeyEvent
};

//Main event handler entry point
pub fn handle_events(app: &mut app::App) -> Result<bool> {

    if !poll(Duration::from_millis(100))? {
        return Ok(false);
    }

    match read()? {
        Event::Key(event) => {
            let handle = handle_key_event(event, app);
            if handle {
                return Ok(false);
            }
            return Ok(true);
        }
        Event::Mouse(event) => {
            handle_mouse_event(event, app);
        }
        _ => (),
    }

    Ok(false)
}

fn handle_mouse_event(event: MouseEvent, app: &mut app::App) {

    if let Some(component) = app.get_hovered_comp(event) {

        match component {
            app::Component::PathTrail => handle_path_trail_me(event, app),
            app::Component::FirstPane => {
                handle_first_pane_me(event, app);
                unselect_path_trail(app);
            }
            app::Component::SecondPane => {
                handle_second_pane_me(event, app);
                unselect_path_trail(app);
            }
        }
    }
}

fn unselect_path_trail(app: &mut app::App) {
    app.path_trail.hovered_path = None;
}

fn handle_path_trail_me(event: MouseEvent, app: &mut app::App) {

    if let Some(index) = app.path_trail.get_hovered_index(event) {
        app.path_trail.hovered_path = Some(index);
    }

    match event.kind {
        MouseEventKind::Down(click) => {

            if let Some(index) = app.path_trail.get_hovered_index(event) {
                let path = &app.path_trail.paths[index].1;

                app.first_pane.load_path(path.to_path_buf());
                app.path_trail.load_path(&path.to_path_buf());

                app.second_pane.load_path(PathBuf::new());
                app.second_pane.files.unselect();
            }
        }
        _ => {}
    }
}

fn handle_first_pane_me(event: MouseEvent, app: &mut app::App) {
    let pane = &mut app.first_pane;
    let offset = pane.files.state.offset_mut();

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
            let index = pane.get_index(event);
            if index >= pane.files.items.len() {
                pane.files.unselect();
                return;
            }

            pane.files.state.select(Some(index));

            let path = &mut pane.entries[index];
            app.second_pane.load_path(path.path());
            app.path_trail.load_path(&path.path().to_path_buf());

            app.second_pane.files.unselect();
        }
        _ => {}
    }
}

fn handle_second_pane_me(event: MouseEvent, app: &mut app::App) {
    let pane = &mut app.second_pane;
    let offset = pane.files.state.offset_mut();

    match event.kind {
        MouseEventKind::ScrollDown => {
            *offset += 1;
        }
        MouseEventKind::ScrollUp => {
            if *offset > 0 {
                *offset -= 1;
            }
        }
        MouseEventKind::Down(_click) => {
            let index = pane.get_index(event);
            if index >= pane.files.items.len() {
                pane.files.unselect();
                return;
            }

            let path = &mut pane.entries[index].path();
            if let Some(selected_index) = pane.files.state.selected() {

                if path.is_dir() && selected_index == index {

                    app.first_pane.load_path(pane.current_path.to_path_buf());
                    app.first_pane.files.state.select(Some(index));

                    pane.load_path(path.to_path_buf());
                    pane.files.unselect();

                    app.path_trail.load_path(&path.to_path_buf());
                } 
                else {
                    pane.files.state.select(Some(index));
                }
            } 
            else {
                pane.files.state.select(Some(index));
            }
        }
        _ => {}
    }
}

fn handle_key_event(event: KeyEvent, app: &mut app::App) -> bool {
    match event.code {
        KeyCode::Up => (),
        KeyCode::Down => (),
        KeyCode::Char('b') => (),
        KeyCode::Char('q') => {
            return false;
        }
        _ => (),
    }
    true
}
