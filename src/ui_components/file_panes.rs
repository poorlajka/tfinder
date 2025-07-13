use crate::PathBuf;
use crate::Path;
use ratatui::prelude::Rect;
use crate::ui_components::{
    file_pane::FilePane,
    preview_pane::PreviewPane,
};

pub struct FilePanes {
    pub panes: Vec<FilePane>,
    pub preview: Option<PreviewPane>,
    pub focused: Option<usize>,
    pub hovered: Option<usize>,
    pub rect: Rect,
    pub nr_of_panes: usize,
}

impl FilePanes {
    pub fn new(path: &PathBuf, nr_of_panes: usize, rect: Rect) -> Self {
        let pane1_rect = Rect {
            x: rect.x,
            y: rect.y,
            height: rect.height,
            width: rect.width / nr_of_panes as u16,
        };
        let mut panes = vec![FilePane::new(path, pane1_rect)];
        FilePanes {
            panes,
            preview: None, 
            focused: None,
            hovered: None, 
            rect,
            nr_of_panes: nr_of_panes,
        }
    }

    pub fn resize(&mut self, rect: Rect) {
        self.rect = rect;
        for (i, mut file_pane) in &mut self.panes.iter_mut().enumerate() {
            file_pane.resize(Self::rect_from_pane_index(self.rect, i, self.nr_of_panes));
        }
    }

    pub fn show_dir(&mut self, path: &PathBuf, pane_index: usize) {
        if pane_index == self.nr_of_panes {

            for i in 0..self.nr_of_panes - 1 {

                let (left, right) = self.panes.split_at_mut(i + 1);
                let pane = &mut left[i];
                let next = &right[0];

                let prev_path = next.current_path.clone();
                pane.load_path(&prev_path);
                pane.selected = next.selected;
            }
            self.panes[self.nr_of_panes - 1].load_path(path);
        }
        else {
            if self.panes.len() < pane_index + 1 {
                self.panes.push(FilePane::new_empty(
                    Self::rect_from_pane_index(
                        self.rect, 
                        pane_index, 
                        self.nr_of_panes
                    )
                ));
            }
            self.panes[pane_index].load_path(path);
            self.close_from(pane_index+1);
        }
        self.preview = None;
    }

    pub fn show_file(&mut self, path: &PathBuf, pane_index: usize) {
        self.close_from(pane_index);
        let mut new_index = pane_index;
        if pane_index == self.nr_of_panes {

            for i in 0..self.nr_of_panes - 1 {

                let (left, right) = self.panes.split_at_mut(i + 1);
                let pane = &mut left[i];
                let next = &right[0];

                let prev_path = next.current_path.clone();
                pane.load_path(&prev_path);
                pane.selected = next.selected;
            }
            self.panes[self.nr_of_panes - 1].load_path(path);
            new_index -= 1;
        }

        let pane_width = self.rect.width / self.nr_of_panes as u16;
        let width = self.rect.width - (new_index as u16 * pane_width);
        self.preview = Some(PreviewPane::new(
            Rect::new(
                self.rect.x + pane_width * new_index as u16, 
                self.rect.y, 
                width, 
                self.rect.height
            ),
            path,
        ));
    }

    pub fn close_from(&mut self, from_index: usize) {
        self.panes.truncate(from_index);
    }

    fn rect_from_pane_index(rect: Rect, pane_index: usize, nr_of_panes: usize) -> Rect {
        let pane_width = rect.width / nr_of_panes as u16;
        Rect::new(
            rect.x + pane_width * pane_index as u16,
            rect.y,
            pane_width,
            rect.height,
        )
    }

    pub fn get_pane_index_at(&self, row: u16, col: u16) -> Option<usize> {
        let pane_index = ((col - self.rect.x) / (self.rect.width/self.nr_of_panes as u16)) as usize;
        
        if pane_index < self.panes.len() {
            Some(pane_index)
        }
        else {
            None
        }
    }
}