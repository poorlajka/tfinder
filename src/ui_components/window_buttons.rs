use crate::Rect;

pub struct WindowButtons {
    rect: Rect,
}

impl WindowButtons {
    pub fn new(rect: Rect) -> Self {
        WindowButtons {
            rect,
        }
    }
}