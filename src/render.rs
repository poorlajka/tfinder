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
use crate::ui_components::{
    prompt,
    file_pane,
    path_breadcrumbs::PathBreadcrumbs,
    top_bar::TopBar,
    bot_bar::BotBar,
    file_panes::FilePanes,
    preview::{Preview, PreviewType},
};

use ratatui_image::StatefulImage; 


pub fn render_app(frame: &mut Frame, app: &mut app::App, render_config: &config::Config) {

    render_top_bar(
        frame, 
        &app.top_bar,
        &render_config.colors.path_trail
    );

    render_bot_bar(
        frame, 
        &app.bot_bar, 
        &render_config.colors.prompt_bar
    );

    render_file_panes (
        frame, 
        &app.file_panes, 
        &render_config.colors.file_panes
    );

    //render_pane(frame, &app.first_pane, &render_config.colors.file_panes, app.focus == app::Component::FirstPane);
    //render_pane(frame, &app.second_pane, &render_config.colors.file_panes, app.focus == app::Component::SecondPane);
    //render_prompt(frame, &app.prompt, &render_config.colors.prompt_bar);
    //render_preview(frame, &mut app.preview, );
}

fn render_top_bar(frame: &mut Frame, top_bar: &TopBar, colors: &config::PathTrailColors) {
    let trail = &top_bar.path_breadcrumbs;
    let mut pos: usize = 0;
    let area = top_bar.rect;

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
        frame.render_widget(paragraph, Rect::new(pos as u16, 0, width as u16, area.height));
        pos += width;
        frame.render_widget(Paragraph::new(" > "), Rect::new(pos as u16, 0, 3, area.height));
        pos += 3;
    }

}

fn render_bot_bar(frame: &mut Frame, bot_bar: &BotBar, colors: &config::PromptBarColors) {

    let prompt = &bot_bar.prompt;
    let area = bot_bar.rect;
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

    frame.render_widget(current_prompt, area);
}

fn render_file_panes(frame: &mut Frame, file_panes: &FilePanes, colors: &config::FilePanesColors) {

    for file_pane in &file_panes.panes {
        render_pane(frame, file_pane, colors, false)

    }
}

pub fn render_pane(frame: &mut Frame, pane: &file_pane::FilePane, colors: &config::FilePanesColors, is_focused: bool) {

    frame.render_stateful_widget(
        get_pane_list(&pane, colors, is_focused),
        pane.rect,
        &mut pane.files.state.clone(),
    );
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



/*
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




*/
