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
        }
    }

    pub fn resize(&mut self, rect: Rect) {
        self.rect = rect;
        for (i, mut file_pane) in &mut self.panes.iter_mut().enumerate() {
            file_pane.resize(Self::rect_from_pane_index(self.rect, i));
        }
    }

    pub fn open(&mut self, path: &PathBuf, pane_index: usize) {
        if self.panes.len() < pane_index + 1 {
            self.panes.push(FilePane::new_empty(Self::rect_from_pane_index(self.rect, pane_index)));
        }
        self.panes[pane_index].load_path(path);
        for pane in &mut self.panes.iter_mut().skip(pane_index) {

        }

    }

    fn rect_from_pane_index(rect: Rect, pane_index: usize) -> Rect {
        let pane_width = rect.width / 4;
        Rect::new(
            rect.x + pane_width * pane_index as u16,
            rect.y,
            pane_width,
            rect.height,
        )
    }

    pub fn get_pane_index_at(&self, row: u16, col: u16) -> usize {
        return 0;
    }
}