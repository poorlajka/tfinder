use crate::app;
use crate::{
    Block,
    Borders,
    Frame,
    Line,
    List,
    ListItem,
    Modifier,
    Rect,
    Style,
    Paragraph,
};
use crate::config;
use crate::prompt;
use crate::file_pane;
use crate::path_trail;

use ratatui_image::StatefulImage; 
use crate::preview::{Preview, PreviewType};


pub fn render_app(frame: &mut Frame, app: &mut app::App, render_config: &config::Config) {

    render_trail(frame, &app.path_trail, &render_config.colors.path_trail);
    //Dealing with focus like this is hacky and does not spark joy but it works
    render_pane(frame, &app.first_pane, &render_config.colors.file_panes, app.focus == app::Component::FirstPane);
    render_pane(frame, &app.second_pane, &render_config.colors.file_panes, app.focus == app::Component::SecondPane);
    render_prompt(frame, &app.prompt, &render_config.colors.prompt_bar);

    render_preview(frame, &mut app.preview, );
}

fn render_preview(frame: &mut Frame, preview: &mut Preview) {
    match &preview.preview_type {
        PreviewType::Image(dyn_img) => {
            let image = StatefulImage::new(None);
            frame.render_stateful_widget(
                image,
                preview.rect,
                &mut dyn_img.image.clone(),
            );
        }
        PreviewType::File => {
        }
        PreviewType::Folder => {
        }
        PreviewType::None => {
        }
    }
}

fn get_pane_list(file_pane: &file_pane::FilePane, colors: &config::FilePanesColors, is_focused: bool) -> List<'static> {
    let color = if is_focused {
        colors.selected_focus
    }
    else {
        colors.selected_no_focus
    };

    let first_pane_files: Vec<ListItem> = file_pane
        .files
        .items
        .iter()
        .map(|i| {
            let lines = vec![Line::from(i.to_owned().0)];
            ListItem::new(lines).style(Style::default().fg(colors.text_default))
        })
        .collect();

    List::new(first_pane_files)
        .block(Block::default().borders(Borders::RIGHT))
        .style(Style::default().fg(colors.border))
        .highlight_style(
            Style::default()
                .bg(color)
                .fg(colors.text_selected)
                .add_modifier(Modifier::ITALIC)
                .add_modifier(Modifier::BOLD),
        )
}
pub fn render_pane(frame: &mut Frame, pane: &file_pane::FilePane, colors: &config::FilePanesColors, is_focused: bool) {

    frame.render_stateful_widget(
        get_pane_list(&pane, colors, is_focused),
        pane.rect,
        &mut pane.files.state.clone(),
    );
}

pub fn render_trail(frame: &mut Frame, trail: &path_trail::PathTrail, colors: &config::PathTrailColors) {
    let mut pos: usize = 0;
    for (i, (name, _)) in trail.paths.iter().enumerate() {
        let mut paragraph = Paragraph::new(name.to_string());
        if let Some(index) = trail.hovered_path {
            if index == i {
                paragraph = paragraph.style(Style::default().fg(colors.text_hovered));
            } else {
                paragraph = paragraph.style(Style::default().fg(colors.text_default));
            }
        } else {
            paragraph = paragraph.style(Style::default().fg(colors.text_default));
        }
        let mut width = name.len();
        if width > 20 {
            width = 20;
        }
        frame.render_widget(paragraph, Rect::new(pos as u16, 0, width as u16, 2));
        pos += width;
        frame.render_widget(Paragraph::new(" > "), Rect::new(pos as u16, 0, 3, 2));
        pos += 3;
    }

}


pub fn render_prompt(frame: &mut Frame, prompt: &prompt::Prompt, colors: &config::PromptBarColors) {

    let current_prompt = if prompt.is_active() {

        Paragraph::new(
            prompt.get_prompt_string() + &prompt.input.clone() + "_",
        )
        .style(Style::default().fg(colors.text_prompt).bg(colors.background))
    }
    else {

        Paragraph::new(
            "C Create    R Rename    M Move    D Delete    O Open    H Help    / Search    F fill",
        )
        .style(Style::default().fg(colors.text_default).bg(colors.background))
    };

    frame.render_widget(current_prompt, prompt.rect);
}

