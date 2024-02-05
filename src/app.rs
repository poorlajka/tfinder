use crate::finder;
use crate::DirEntry;
use crate::ListState;
use crate::MouseEvent;
use std::path::PathBuf;
use ratatui::prelude::Rect;
use std::path::Path;

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

pub struct Prompt {
    is_active: bool,
    command: Command,
    pub input: String,
    pub rect: Rect,
    root: PathBuf,
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
    pub fn get_prompt_string(&self) -> String {
        match self {
            Self::Create => {
                String::from("Name of file to create: ")
            }
            _ => {
                String::from("")
            }
        }
    }
}

impl Prompt {
    pub fn get_prompt_string(&self) -> String {
        return self.command.get_prompt_string();
    }
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

    pub fn is_active(&self) -> bool{
        return self.is_active;
    }

    pub fn run_command(&mut self, root: &Path) {
        match self.command {
            Command::Create => {
                finder::create_file(root, self.input.clone());
            }
            _ => {
            }
        }
        self.input.clear();
        self.is_active = false;
    }
}


impl App {
    pub fn new(term_size: &Rect, path: &PathBuf) -> Self {

        /*
        App {
            first_pane: FilePane::new(),
            second_pane: FilePane::new(),
            path_trail: PathTrail::new(),
            prompt: Prompt::new(),
            config: config,
            rect: rect,
        }
            */

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

pub struct PathTrail {
    pub paths: Vec<(String, PathBuf)>,
    pub hovered_path: Option<usize>,
    pub rect: Rect,
}

struct Interval {
    begin: u16,
    end: u16,
}

impl PathTrail {

    fn is_mouse_on(&self, column: u16, row: u16) -> bool {
        let Rect {x, y, width, height} = self.rect;

        column >= x && column <= x + width
            && row >= y && row <= y + height
    }

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
    pub files: StatefulList<(String, usize)>,
    pub entries: Vec<DirEntry>,
    pub current_path: PathBuf,
    pub rect: Rect,
}

impl FilePane {
    fn is_mouse_on(&self, column: u16, row: u16) -> bool {
        let Rect {x, y, width, height} = self.rect;
        
        column >= x && column <= x + width
            && row >= y && row <= y + height
    }

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
