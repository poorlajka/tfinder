use crate::finder;
use crate::DirEntry;
use crate::ListState;
use crate::MouseEvent;
use std::path::PathBuf;
use std::path::Path;
use crate::config;
use ratatui::prelude::Rect;

pub struct App {
    pub path_trail: PathTrail,
    pub first_pane: FilePane,
    pub second_pane: FilePane,
    pub prompt: Prompt,
    pub rect: Rect,
    pub config: config::Config,
}


pub enum Component {
    PathTrail,
    FirstPane,
    SecondPane,
}

pub struct Prompt {
    pub is_active: bool,
    pub command: Command,
    pub input: String,
    pub rect: Rect,
    pub root: PathBuf,
    pub tick: i32,
}

pub enum Command {
    Create,
    Rename,
    Move,
    Delete,
    Open,
    Help,
    Search,
    Fill,
    None,
}

impl Command {
    pub fn get_prompt(&self) -> String {
        match self {
            Self::Create => {
                "Name of file to create: ".to_string()
            }
            _ => {
                "".to_string()
            }
        }
    }
}

impl Prompt {
    pub fn begin_prompt(&mut self, command: Command) {
        self.is_active = true;
        self.command = command;
        self.input.clear();
    }

    pub fn enter_input(&mut self, input: char) {
        self.input.push(input);
    }

    pub fn delete_input(&mut self) {
        self.input.pop();
    }
    pub fn cancel(&mut self) {
        self.is_active = false;
        self.input.clear();
        self.command = Command::None;
    }

    pub fn is_active(&mut self) -> bool{
        return self.is_active;
    }

    pub fn run_command(&mut self) {
        match self.command {
            Command::Create => {
                finder::create_file(&self.root, self.input.clone());
            }
            _ => {
            }
        }
        self.input.clear();
        self.is_active = false;
    }
}


impl App {
    pub fn new(config: config::Config, term_size: &Rect, path: &PathBuf) -> Self {
        let mut app = App {
            first_pane: FilePane {
                height: 100,
                width: 40,
                files: StatefulList::with_items(Vec::new()),
                current_path: PathBuf::new(),
                entries: Vec::new(),
                rect: Rect { x: 0, y: 2, 
                    width: term_size.width/3, 
                    height: term_size.height - 2 - 2, 
                }, 
                
            },
            second_pane: FilePane {
                height: 100,
                width: 40,
                files: StatefulList::with_items(Vec::new()),
                current_path: PathBuf::new(),
                entries: Vec::new(),
                rect: Rect { x: term_size.width/3, y: 2, 
                    width: term_size.width/3, 
                    height: term_size.height - 2 - 2, 
                }, 
            },
            path_trail: PathTrail {
                height: 2,
                width: 30,
                paths: Vec::new(),
                hovered_path: None,
                rect: Rect::new(1,1,1,1),
            },
            prompt: Prompt {
                is_active: false,
                command: Command::None,
                input: String::new(),
                rect: Rect::new(0,term_size.height - 1,term_size.width,1),
                root: PathBuf::from("/home/viktor/programming/term-finder/"),
                tick: 0,
            },
            rect: *term_size,
            config: config,
        };


        app.first_pane.load_path(path.to_path_buf());
        //app.path_trail.paths = vec![("hello".to_string(), PathBuf::new())];
        app.path_trail.load_path(&path.to_path_buf());
        app.first_pane.files.state.select(None);
        app.second_pane.files.state.select(None);
        return app;
    }

    pub fn get_hovered_comp(&self, event: MouseEvent) -> Option<Component> {
        let (column, row) = (event.column, event.row);
        if row < self.path_trail.height {
            return Some(Component::PathTrail);
        }
        if column < self.first_pane.width {
            return Some(Component::FirstPane);
        }
        if column < self.first_pane.width * 2 {
            return Some(Component::SecondPane);
        }
        return None;
    }
}

pub struct PathTrail {
    pub height: u16,
    pub width: u16,
    pub paths: Vec<(String, PathBuf)>,
    pub hovered_path: Option<usize>,
    pub rect: Rect,
}

struct Interval {
    begin: u16,
    end: u16,
}

impl PathTrail {
    pub fn get_hovered_index(&self, event: MouseEvent) -> Option<usize> {
        let mut interval = Interval { begin: 0, end: 0 };

        for (i, (name, _)) in self.paths.iter().enumerate() {
            interval.begin = interval.end;
            interval.end = interval.end + name.len() as u16 + 3;

            if event.column >= interval.begin && event.column <= interval.end {
                return Some(i);
            }
        }
        None
    }

    pub fn load_path(&mut self, path: &PathBuf) {
        self.paths.clear();
        for ancestor in &mut path.ancestors() {
            self.paths.push((
                ancestor
                    .iter()
                    .last()
                    .unwrap()
                    .to_string_lossy()
                    .to_string(),
                ancestor.to_path_buf(),
            ));
        }
        self.paths.reverse();
    }
}

pub struct FilePane {
    pub height: u16,
    pub width: u16,
    pub files: StatefulList<(String, usize)>,
    pub entries: Vec<DirEntry>,
    pub current_path: PathBuf,
    pub rect: Rect,
}

impl FilePane {
    pub fn get_index(&mut self, event: MouseEvent) -> usize {
        let offset = self.files.state.offset();
        return event.row as usize - 2 + offset;
    }
    pub fn load_path(&mut self, path: PathBuf) {
        self.current_path = path.to_path_buf();

        self.entries.clear();
        let _ = finder::get_folders(&mut self.entries, &path);

        self.files.items.clear();

        let mut remove = Vec::new();

        for (i, folder) in self.entries.iter().enumerate() {
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
            self.files.items.push((item_name, i))
        }
        for i in remove {
            if i < self.entries.len() {
                self.entries.remove(i);
            }
        }
    }
    pub fn update(&mut self) {
        let path = &self.current_path;
        self.load_path(path.to_path_buf());
    }
}

#[derive(Debug, Clone)]
pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> StatefulList<T> {
    pub fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    pub fn next(&mut self) {
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

    pub fn previous(&mut self) {
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

    pub fn unselect(&mut self) {
        self.state.select(None);
    }
}
