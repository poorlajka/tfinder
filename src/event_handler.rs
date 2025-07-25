use crate::app;
use crate::Duration;
use crate::command::Command;
use crate::event::MouseButton;
use crate::Rect;

use std::path::PathBuf;

use anyhow::Result;
use crossterm::event::{
    KeyCode, poll, read, Event, MouseEvent, MouseEventKind, KeyEvent
};

pub fn handle_events(app: &mut app::App) -> Result<bool> {

    if !poll(Duration::from_millis(1000))? {
        return Ok(false);
    }

    app.top_bar.path_breadcrumbs.hovered_path = None;

    match read()? {
        Event::Key(event) => {
            return Ok(true);
        }
        Event::Mouse(event) => {
            handle_mouse_event(app, event);
        }
        Event::Resize(width, height) => {
            app.resize(width, height);
        }
        _ => {}
    }

    Ok(false)
}

fn handle_mouse_event(app: &mut app::App, event: MouseEvent) {

    if let Some(component) = app.get_component_at(event.row, event.column) {

        match component {
            app::Component::TopBar => {
                handle_mouse_event_top_bar(app, event);
            }
            app::Component::FilePanes => {
                handle_mouse_event_file_panes(app, event);
            }
            _ => {}
        }
    }
}

fn handle_mouse_event_top_bar(app: &mut app::App, event: MouseEvent) {
    let breadcrumbs = &mut app.top_bar.path_breadcrumbs;
    breadcrumbs.hovered_path = breadcrumbs.get_hovered_index(event);

    match event.kind {
        MouseEventKind::Up(MouseButton::Left) => {
            if let Some(path_index) = breadcrumbs.hovered_path {
                let path = &breadcrumbs.paths[path_index].1;
                app.file_panes.show_dir(&path, 0);
                breadcrumbs.load_path(&path.clone());
            }
        }
        _ => {}
    }
}

fn handle_mouse_event_file_panes(app: &mut app::App, event: MouseEvent) {
    let row = event.row;
    let col = event.column;

    let pane_index_option = app.file_panes.get_pane_index_at(row, col);
    if pane_index_option.is_none() {
        return;
    }
    let pane_index = pane_index_option
        .expect("I should be returning above in case of pane_index being None");
    let file_panes = &mut app.file_panes;

    match event.kind {
        MouseEventKind::Up(MouseButton::Left) => {

            file_panes.focused = Some(pane_index);
            if let Some(file_index) = file_panes.panes[pane_index].get_file_index_at(row, col) {
                file_panes.panes[pane_index].select(Some(file_index));

                let entry = &file_panes.panes[pane_index].entries[file_index];
                if let Ok(file_type) = entry.file_type() {

                    if file_type.is_dir() {
                        app.top_bar.path_breadcrumbs.load_path(&entry.path());
                        file_panes.show_dir(&entry.path(), pane_index + 1);
                    }
                    else if file_type.is_file() {
                        app.top_bar.path_breadcrumbs.load_path(
                            &file_panes.panes[pane_index].current_path
                        );
                        file_panes.show_file(&entry.path(), pane_index + 1);
                    }
                }
            }
            else {
                file_panes.close_from(pane_index + 1);
                file_panes.panes[pane_index].select(None);

                app.top_bar.path_breadcrumbs.load_path(
                    &file_panes.panes[pane_index].current_path
                );
            }
        }
        MouseEventKind::Up(MouseButton::Right) => {
        }
        MouseEventKind::Drag(MouseButton::Left) => {
            // TODO: Dragging files
        }
        MouseEventKind::ScrollUp => {
            file_panes.panes[pane_index].scroll_up();
        }
        MouseEventKind::ScrollDown => {
            file_panes.panes[pane_index].scroll_down();
        }
        _ => {
            // TODO: Hovering (do I even want this?)
        }
    }
}

/*
fn unselect_path_trail(app: &mut app::App) {
    app.path_trail.hovered_path = None;
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
        MouseEventKind::Down(_) => {

            if let Some(index) = app.path_trail.get_hovered_index(event) {
                let path = &app.path_trail.paths[index].1;
                app.first_pane.files.unselect();

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
        MouseEventKind::Down(_click) => {
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
            app.focus = app::Component::FirstPane;
            app.preview.load(&path.path());
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
        //Mouse does not display picture correctly but I should probably just take a weekend and refactor this entire file instead so lmao
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
                app.focus = app::Component::SecondPane;
            }
            app.preview.load(&app.second_pane.entries[index].path());
        }
        _ => {}
    }
}

//TODO: This is alot of boiler plate that can prolly be factored out to be cleaner
fn move_up(app: &mut app::App) {
    match app.focus {
        app::Component::FirstPane => {
            if let Some(mut state) = app.first_pane.files.state.selected() {
                if state > 0 {
                    app.first_pane.files.previous();
                    let path = &app.first_pane.entries[state-1];
                    app.second_pane.load_path(path.path());
                    app.second_pane.files.state.select(None);
                    state-=1;
                }
                app.preview.load(&app.first_pane.entries[state].path());
            }
            else {
                app.first_pane.files.state.select(Some(0));
            }
        }
        app::Component::SecondPane => {
            if let Some(mut state) = app.second_pane.files.state.selected() {
                if state > 0 {
                    app.second_pane.files.previous();
                    state-=1;
                }
                app.preview.load(&app.second_pane.entries[state].path());
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
            if let Some(mut state) = app.first_pane.files.state.selected() {
                if state < app.first_pane.entries.len()-1 {
                    app.first_pane.files.next(app.first_pane.rect.height-1);
                    let path = &app.first_pane.entries[state+1];
                    app.second_pane.load_path(path.path());
                    app.second_pane.files.state.select(None);
                    state+=1;
                }
                app.preview.load(&app.first_pane.entries[state].path());

            }
            else {
                app.first_pane.files.state.select(Some(0));
            }
        }
        app::Component::SecondPane => {
            if let Some(mut state) = app.second_pane.files.state.selected() {
                if state < app.second_pane.entries.len()-1 {
                    app.second_pane.files.next(app.first_pane.rect.height-1);
                    state+=1;
                }
                app.preview.load(&app.second_pane.entries[state].path());
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
                        app.preview.load(&app.second_pane.entries[index2].path());

                    }
                    else if app.second_pane.entries.len() > 0 {
                        app.second_pane.files.state.select(Some(0));
                        app.preview.load(&app.second_pane.entries[0].path());
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
            let path_len = app.path_trail.paths.len();
            if path_len > 1 {
                let path = &app.path_trail.paths[path_len -2].1;

                app.second_pane.load_path(app.first_pane.current_path.clone());
                app.second_pane.files.state.select(app.first_pane.files.state.selected());


                app.first_pane.load_path(path.to_path_buf());
                app.first_pane.files.state.select(Some(0));
                app.path_trail.load_path(&path.to_path_buf());

            }
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
                app.prompt.begin_prompt(Command::Create);
            }
            KeyCode::Char('r') => {
                //app.prompt.begin_prompt(Command::Rename);
            }
            KeyCode::Char('d') => {
                //app.prompt.begin_prompt(Command::Move);
            }
            KeyCode::Char('m') => {
                //app.prompt.begin_prompt(Command::Delete);
            }
            KeyCode::Char('o') => {
                //app.prompt.begin_prompt(Command::Open);
            }
            KeyCode::Char('/') => {
                //app.prompt.begin_prompt(Command::Search);
            }
            KeyCode::Char('f') => {
            }
            
            KeyCode::Char('g') => {
            }
            KeyCode::Esc => {
                return false;
            }
            _ => (),
        }
    }
    true

}
*/
