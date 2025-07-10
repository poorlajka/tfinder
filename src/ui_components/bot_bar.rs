use crate::ui_components::prompt::Prompt;
use crate::PathBuf;
use crate::Rect;

pub struct BotBar {
    pub prompt: Prompt,
    pub rect: Rect,
}

impl BotBar {
    pub fn new(path: &PathBuf, rect: Rect) -> Self {
        BotBar {
            prompt: Prompt::new(path),
            rect,
        }
    }

    pub fn resize(&mut self, rect: Rect) {
        self.rect = rect;
    }

    pub fn contains_pos(&self, row: u16, _column: u16) -> bool {
        return row >= self.rect.y - self.rect.height;
    }
}
