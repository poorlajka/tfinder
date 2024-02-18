use std::path::PathBuf;
use ratatui::prelude::Rect;
use crate::path_trail::PathTrail;
use crate::file_pane::{ FilePane, StatefulList };

use ratatui_image::picker::Picker;

use crate::prompt::Prompt;
use crate::command::Command;
use crate::preview::{Preview, PreviewType};

pub struct App {
    pub focus: Component,
    pub path_trail: PathTrail,
    pub first_pane: FilePane,
    pub second_pane: FilePane,
    pub prompt: Prompt,
    pub rect: Rect,
    pub preview: Preview,
}

#[derive(PartialEq)]
pub enum Component {
    PathTrail,
    FirstPane,
    SecondPane,
}

impl App {
    //TODO this could deffo be nicer
    pub fn new(term_size: &Rect, path: &PathBuf) -> Self {

        let mut picker = Picker::from_termios().unwrap();
        picker.guess_protocol();

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
            preview: Preview {
                picker: picker,
                preview_type: PreviewType::None,
                rect: Rect::new(2 * term_size.width / 3 + 2 , term_size.height/4, term_size.width/3 - 5, term_size.height - 20),
                is_rendered: false,
            }
        };

        app.first_pane.load_path(path.to_path_buf());
        app.path_trail.load_path(&path.to_path_buf());
        app.first_pane.files.state.select(None);
        app.second_pane.files.state.select(None);

        app
    }

    //TODO THIS DOES NOT WORK PROPERLY RIGHT NOW
    pub fn resize(&mut self, new_width: u16, new_height: u16) {
        self.first_pane.rect = 
                Rect { x: 0, y: 2, 
                    width: new_width/3, 
                    height: new_height - 2 - 2, 
                }; 
        self.second_pane.rect = 
                Rect { x: new_width/3, y: 2, 
                    width: new_width/3, 
                    height: new_height - 2 - 2, 
                }; 
        self.path_trail.rect = 
                Rect::new(0,0,new_width,1);
        self.prompt.rect = 
                Rect::new(0,new_height - 1,new_width,1);
        self.preview.rect = 
                Rect::new(2 * new_width / 3 + 5 , new_height/4, new_width/3 - 10, new_height - 20);
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


