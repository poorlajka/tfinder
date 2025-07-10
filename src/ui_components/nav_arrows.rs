use crate::Rect;


pub struct NavArrows {
    rect: Rect,
}

impl NavArrows {
    pub fn new(rect: Rect) -> Self {
        NavArrows {
            rect,
        }
    }
}

