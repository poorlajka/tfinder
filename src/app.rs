use crate::finder;
use crate::DirEntry;
use crate::ListState;
use crate::MouseEvent;
use std::path::PathBuf;
use ratatui::prelude::Rect;
use std::path::Path;
use crate::path_trail::PathTrail;
use crate::file_pane::{ FilePane, StatefulList };

use crate::prompt::Prompt;
use crate::command::Command;

pub struct App {
    pub focus: Component,
    pub path_trail: PathTrail,
    pub first_pane: FilePane,
    pub second_pane: FilePane,
    pub prompt: Prompt,
    pub rect: Rect,
}

#[derive(PartialEq)]
pub enum Component {
    PathTrail,
    FirstPane,
    SecondPane,
}

impl App {
    pub fn new(term_size: &Rect, path: &PathBuf) -> Self {
        let mut app = App {

            first_pane: FilePane {
                files: StatefulList::with_items(Vec::new()),
                current_path: PathBuf::new(),
                entries: Vec::new(),
                rect: Rect { x: 0, y: 2, 
                    width: term_size.width/3, 
                    height: term_size.height - 2 - 2, 
                }, 
                
            },

            second_pane: FilePane {
                files: StatefulList::with_items(Vec::new()),
                current_path: PathBuf::new(),
                entries: Vec::new(),
                rect: Rect { x: term_size.width/3, y: 2, 
                    width: term_size.width/3, 
                    height: term_size.height - 2 - 2, 
                }, 
            },

            path_trail: PathTrail {
                paths: Vec::new(),
                hovered_path: None,
                rect: Rect::new(0,0,term_size.width,1),
            },

            prompt: Prompt {
                is_active: false,
                command: Command::None,
                input: String::new(),
                rect: Rect::new(0,term_size.height - 1,term_size.width,1),
                root: PathBuf::from("/home/viktor/programming/term-finder/"),
                tick: 0,
            },

            focus: Component::FirstPane,

            rect: *term_size,
        };


        app.first_pane.load_path(path.to_path_buf());
        //app.path_trail.paths = vec![("hello".to_string(), PathBuf::new())];
        app.path_trail.load_path(&path.to_path_buf());
        app.first_pane.files.state.select(None);
        app.second_pane.files.state.select(None);
        return app;
    }
    pub fn resize(&mut self, new_size: &Rect) {
    }

    pub fn get_hovered_comp(&self, column: u16, row: u16) -> Option<Component> {

        if self.path_trail.is_mouse_on(column, row) {
            return Some(Component::PathTrail);
        }
        if self.first_pane.is_mouse_on(column, row) {
            return Some(Component::FirstPane);
        }
        if self.second_pane.is_mouse_on(column, row) {
            return Some(Component::SecondPane);
        }
        return None;
    }
}


