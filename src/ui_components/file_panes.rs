use crate::PathBuf;
use ratatui::prelude::Rect;
use crate::ui_components::{
    file_pane::FilePane,
    preview_pane::PreviewPane,
};

pub struct FilePanes {
    pub panes: Vec<FilePane>,
    pub preview: Option<PreviewPane>,
    pub focused: Option<u32>,
    pub hovered: Option<u32>,
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
        let pane_width = self.rect.width / 4;
        for (i, mut file_pane) in &mut self.panes.iter_mut().enumerate() {
            file_pane.resize(Rect::new(
                self.rect.x + pane_width * i as u16,
                self.rect.y,
                pane_width,
                self.rect.height,
            ));
        }
    }
}