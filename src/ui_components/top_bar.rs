use crate::PathBuf;
use crate::Rect;
use crate::ui_components::{
    path_breadcrumbs::PathBreadcrumbs,
    window_buttons::WindowButtons,
    nav_arrows::NavArrows,
};

const NAV_ARROWS_WIDTH: u32 = 3;
const WINDOW_BUTTON_WIDTH: u32 = 5;

pub struct TopBar {
    pub path_breadcrumbs: PathBreadcrumbs,
    pub nav_arrows: NavArrows,
    pub window_buttons: WindowButtons,
    pub rect: Rect,
}

impl TopBar {
    pub fn new(path: &PathBuf, rect: Rect) -> Self {
        TopBar {
            path_breadcrumbs: PathBreadcrumbs::new(path),
            nav_arrows: NavArrows::new(rect),
            window_buttons: WindowButtons::new(rect),
            rect,
        }
    }

    pub fn resize(&mut self, rect: Rect) {
        self.rect = rect;
    }
}

