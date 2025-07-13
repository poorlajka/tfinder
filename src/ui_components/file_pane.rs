use crate::ListState;
use crate::finder;
use crate::MouseEvent;
use crate::Rect;
use crate::PathBuf;
use crate::DirEntry;

pub struct FilePane {
    pub entries: Vec<DirEntry>,
    pub current_path: PathBuf,
    pub rect: Rect,
    pub state: ListState,
    pub scroll_offset: usize,
    pub selected: Option<usize>,
}

impl FilePane {

    pub fn new(path: &PathBuf, rect: Rect) -> Self {
        let mut file_pane = FilePane::new_empty(rect);
        file_pane.load_path(path);
        file_pane
    }

    pub fn new_empty(rect: Rect) -> Self {
        FilePane {
            entries: Vec::new(),
            current_path: PathBuf::new(),
            rect,
            state: ListState::default(),
            scroll_offset: 0,
            selected: None,
        }
    }

    pub fn resize(&mut self, rect: Rect) {
        self.rect = rect;
    }

    pub fn select(&mut self, index: Option<usize>) {
        self.selected = index;
    }

    pub fn scroll_up(&mut self) {
        if self.scroll_offset > 0 {
            self.scroll_offset -= 1;
        }
    }

    pub fn scroll_down(&mut self) {
        if self.scroll_offset + 1 < self.entries.len() 
            && self.entries.len() - self.scroll_offset >= self.rect.height as usize {
            self.scroll_offset += 1;
        }
    }

    pub fn get_file_index_at(&mut self, row: u16, col: u16) -> Option<usize> {
        let offset = self.scroll_offset;
        let file_index = (row - self.rect.y) as usize + offset;

        if file_index < self.entries.len() {
            Some(file_index)
        }
        else {
            None
        }
    }

    /*
        TODO: Refactor this function
    */
    pub fn load_path(&mut self, path: &PathBuf) {
        self.current_path = path.clone();
        self.selected = None;

        self.entries.clear();
        let _ = finder::get_folders(&mut self.entries, &path);

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
        }
        for i in remove {
            if i < self.entries.len() {
                self.entries.remove(i);
            }
        }
    }

    pub fn update(&mut self) {
        self.load_path(&self.current_path.clone());
    }
}