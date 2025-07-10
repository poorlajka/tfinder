use crate::Rect;

pub struct FavPane {
    pub rect: Rect,
}

impl FavPane {
    pub fn new(rect: Rect) -> Self {
        FavPane {
            rect,
        }
    }
}
