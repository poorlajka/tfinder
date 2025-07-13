use std::path::PathBuf;
use ratatui::prelude::Rect;

use crate::ui_components::{
    top_bar::TopBar,
    bot_bar::BotBar,
    fav_pane::FavPane,
    file_panes::FilePanes,
    preview_pane::PreviewPane,
};

#[derive(PartialEq)]
pub enum Component {
    TopBar,
    BotBar,
    FavPane,
    FilePanes,
}

struct AppLayout {
    pub top_bar: Rect,
    pub bot_bar: Rect,
    pub fav_pane: Rect,
    pub file_panes: Rect,
}

impl AppLayout {
    fn new(width: u16, height: u16) -> Self {
        let top_bar_height = 2;
        let bot_bar_height = 1;
        let fav_pane_width = 15;

        AppLayout {
            top_bar: Rect {
                x: 0,
                y: 0,
                width: width,
                height: top_bar_height,
            },
            bot_bar: Rect {
                x: 0,
                y: height - bot_bar_height,
                width: width,
                height: bot_bar_height,
            },
            fav_pane: Rect {
                x: 0,
                y: top_bar_height,
                width: fav_pane_width,
                height: height 
                    -  (top_bar_height + bot_bar_height),
            },
            file_panes: Rect {
                x: fav_pane_width,
                y: top_bar_height,
                width: width - fav_pane_width,
                height: height 
                    -  (top_bar_height + bot_bar_height),
            },
        }
    }
}

pub struct App {
    pub top_bar: TopBar,
    pub bot_bar: BotBar,
    pub fav_pane: FavPane,
    pub file_panes: FilePanes,
}

impl App {
    pub fn new(size: Rect, initial_path: &PathBuf) -> Self {
        let layout = AppLayout::new(size.width, size.height);

        App {
            top_bar: TopBar::new(initial_path, layout.top_bar),
            bot_bar: BotBar::new(initial_path, layout.bot_bar),
            fav_pane: FavPane::new(layout.fav_pane),
            file_panes: FilePanes::new(initial_path, 4, layout.file_panes),
        }
    }

    pub fn resize(&mut self, width: u16, height: u16) {
        let new_layout = AppLayout::new(width, height);

        self.top_bar.resize(new_layout.top_bar);
        self.bot_bar.resize(new_layout.bot_bar);
    }

    pub fn get_component_at(&self, row: u16, column: u16) -> Option<Component> {
        if row < self.top_bar.rect.height {
            return Some(Component::TopBar);
        }
        if row > self.bot_bar.rect.y {
            return Some(Component::BotBar);
        }

        if column <= self.fav_pane.rect.width {
            return Some(Component::FavPane);
        }

        if column <= self.fav_pane.rect.width + self.file_panes.rect.width {
            return Some(Component::FilePanes);
        }
        None
    }
}