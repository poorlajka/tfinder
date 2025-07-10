use crate::ListState;
use crate::finder;
use crate::MouseEvent;
use crate::Rect;
use crate::PathBuf;
use crate::DirEntry;


pub struct FilePane {
    pub files: StatefulList<(String, usize)>,
    pub entries: Vec<DirEntry>,
    pub current_path: Option<PathBuf>,
    pub rect: Rect,
}

impl FilePane {

    pub fn new(path: &PathBuf, rect: Rect) -> Self {
        let mut file_pane = FilePane::new_empty(rect);
        file_pane.load_path(path);
        file_pane
    }

    pub fn new_empty(rect: Rect) -> Self {
        FilePane {
            files: StatefulList::with_items(Vec::new()),
            entries: Vec::new(),
            current_path: None,
            rect,
        }
    }

    pub fn resize(&mut self, rect: Rect) {
        self.rect = rect;
    }

    pub fn get_index(&mut self, event: MouseEvent) -> usize {
        let offset = self.files.state.offset();
        return event.row as usize - 2 + offset;
    }

    pub fn load_path(&mut self, path: &PathBuf) {
        self.current_path = Some(path.clone());

        self.entries.clear();
        let _ = finder::get_folders(&mut self.entries, &path);

        self.files.items.clear();

        let remove = Vec::new();

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
        if let Some(path) = &self.current_path {
            self.load_path(&path.clone());
        }
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

    pub fn next(&mut self, height: u16) {
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

        if i > (height as usize + self.state.offset()) {
            *self.state.offset_mut() += 1;
        }
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

        if i < (self.state.offset()) {
            *self.state.offset_mut() -= 1;
        }
    }

    pub fn unselect(&mut self) {
        self.state.select(None);
    }
}
