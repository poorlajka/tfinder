use crate::app;
use crate::Duration;

use std::path::PathBuf;

use anyhow::Result;
use crossterm::event::{
    KeyCode, poll, read, Event, MouseEvent, MouseEventKind, KeyEvent
};

//Main event handler entry point
pub fn handle_events(app: &mut app::App) -> Result<bool> {

    if app.prompt.tick > 10 {
        app.prompt.tick = 0;
    }
    else {
        app.prompt.tick+=1;
    }

    if !poll(Duration::from_millis(100))? {
        return Ok(false);
    }

    match read()? {
        Event::Key(event) => {
            //This is how I quit right now kinda ugly though
            if !handle_key_event(event, app) {
                return Ok(true);
            }
        }
        Event::Mouse(event) => {
            handle_mouse_event(event, app);
        }
        _ => (),
    }

    app.first_pane.update();
    app.second_pane.update();


    Ok(false)
}

fn handle_mouse_event(event: MouseEvent, app: &mut app::App) {

    if let Some(component) = app.get_hovered_comp(event.column, event.row) {

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

//TODO: This is alot of boiler plate that can prolly be factored out to be cleaner
fn move_up(app: &mut app::App) {
    match app.focus {
        app::Component::FirstPane => {
            if let Some(state) = app.first_pane.files.state.selected() {
                if state > 0 {
                    app.first_pane.files.state.select(Some(state-1));
                    let path = &app.first_pane.entries[state-1];
                    app.second_pane.load_path(path.path());
                    app.second_pane.files.state.select(None);
                }
            }
            else {
                app.first_pane.files.state.select(Some(0));
            }
        }
        app::Component::SecondPane => {
            if let Some(state) = app.second_pane.files.state.selected() {
                if state > 0 {
                    app.second_pane.files.state.select(Some(state-1));
                }
            }
            else if app.second_pane.entries.len() > 0 {
                app.second_pane.files.state.select(Some(0));
            }
        }
        _ => {
        }
    }
}

fn move_down(app: &mut app::App) {
    match app.focus {
        app::Component::FirstPane => {
            if let Some(state) = app.first_pane.files.state.selected() {
                if state < app.first_pane.entries.len()-1 {
                    app.first_pane.files.state.select(Some(state+1));
                    let path = &app.first_pane.entries[state+1];
                    app.second_pane.load_path(path.path());
                    app.second_pane.files.state.select(None);
                }

            }
            else {
                app.first_pane.files.state.select(Some(0));
            }
        }
        app::Component::SecondPane => {
            if let Some(state) = app.second_pane.files.state.selected() {
                if state < app.second_pane.entries.len()-1 {
                    app.second_pane.files.state.select(Some(state+1));
                }
            }
            else if app.second_pane.entries.len() > 0 {
                app.second_pane.files.state.select(Some(0));
            }
        }
        _ => {
        }
    }
}


//TODO THIS IS SO FUCKING MESSY CLEAN LATER!!!!
fn move_right(app: &mut app::App) {
    match app.focus {
        app::Component::FirstPane => {
            if let Some(index) = app.first_pane.files.state.selected() {
                let path = &mut app.first_pane.entries[index];
                if path.path().is_dir() {
                    app.second_pane.load_path(path.path());
                    app.focus = app::Component::SecondPane;

                    if let Some(index2) = app.second_pane.files.state.selected() {
                        app.second_pane.files.state.select(Some(index2));

                    }
                    else if app.second_pane.entries.len() > 0 {
                        app.second_pane.files.state.select(Some(0));
                    }
                }
            }
        }
        app::Component::SecondPane => {
            if let Some(index) = app.second_pane.files.state.selected() {
                let path = &app.second_pane.entries[index];
                if path.path().is_dir() {
                    app.first_pane.load_path(app.second_pane.current_path.clone());
                    app.first_pane.files.state.select(Some(index));
                    app.second_pane.load_path(path.path());
                    app.second_pane.files.state.select(None);
                    app.focus = app::Component::FirstPane;
                }
            }
        }
        _ => {
        }
    }
}
fn move_left(app: &mut app::App) {
    match app.focus {
        app::Component::FirstPane => {
        }
        app::Component::SecondPane => {
            app.focus = app::Component::FirstPane;
        }
        _ => {
        }
    }
}


fn handle_key_event(event: KeyEvent, app: &mut app::App) -> bool {

    if app.prompt.is_active() {
        match event.code {
            KeyCode::Enter => {
                let path = match app.focus {
                    app::Component::FirstPane => {
                        &app.first_pane.current_path
                    }
                    app::Component::SecondPane => {
                        &app.second_pane.current_path
                    }
                    _ => {
                        &app.first_pane.current_path
                    }

                };
                app.prompt.run_command(path);
            }
            KeyCode::Backspace => {
                app.prompt.delete_input();
            }
            KeyCode::Esc => {
                app.prompt.cancel();
            }
            KeyCode::Char(char) => {
                app.prompt.enter_input(char);
            }
            _ => {
            }
        }
    }
    else {
        match event.code {
            KeyCode::Up | KeyCode::Char('k') => {
                move_up(app);
            }
            KeyCode::Down | KeyCode::Char('j') => {
                move_down(app);
            }
            KeyCode::Left | KeyCode::Char('h') => {
                move_left(app);
            }
            KeyCode::Right | KeyCode::Char('l') => {
                move_right(app);
            }
            KeyCode::Char('c') => {
                app.prompt.begin_prompt(app::Command::Create);
            }
            KeyCode::Char('r') => {
                //app.prompt.begin_prompt(app::Command::Rename);
            }
            KeyCode::Char('d') => {
                //app.prompt.begin_prompt(app::Command::Move);
            }
            KeyCode::Char('m') => {
                //app.prompt.begin_prompt(app::Command::Delete);
            }
            KeyCode::Char('o') => {
                //app.prompt.begin_prompt(app::Command::Open);
            }
            KeyCode::Char('/') => {
                //app.prompt.begin_prompt(app::Command::Search);
            }
            KeyCode::Char('f') => {
                //app.prompt.begin_prompt(app::Command::Fill);
            }
            KeyCode::Char('q') => {
                return false;
            }
            _ => (),
        }
    }
    true

}
