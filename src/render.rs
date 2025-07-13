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
    Color,
    BorderType,
    Alignment,
    Constraint,
    Layout,
    Direction,
};
use crate::config;
use crate::ui_components::{
    prompt,
    file_pane,
    path_breadcrumbs::PathBreadcrumbs,
    top_bar::TopBar,
    bot_bar::BotBar,
    file_panes::FilePanes,
    fav_pane::FavPane,
    preview::{Preview, PreviewType},
    preview_pane::PreviewPane,
};

use ratatui_image::StatefulImage; 


pub fn render_app(frame: &mut Frame, app: &app::App, render_config: &config::Config) {

    render_top_bar(
        frame, 
        &app.top_bar,
        &render_config.colors.path_trail,
    );

    render_bot_bar(
        frame, 
        &app.bot_bar, 
        &render_config.colors.prompt_bar,
    );

    render_fav_pane(
        frame,
        &app.fav_pane,
        &render_config.colors.file_panes,
    );

    render_file_panes(
        frame, 
        &app.file_panes, 
        &render_config.colors.file_panes,
    );

    if let Some(preview) = &app.file_panes.preview {
        render_preview_pane(
            frame, 
            preview, 
            &render_config.colors.file_panes,
        );
    }


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

fn render_fav_pane(frame: &mut Frame, fav_pane: &FavPane, colors: &config::FilePanesColors) {
    let list = List::new(Vec::<ListItem>::new())
        .block(Block::default().borders(Borders::RIGHT))
        .style(Style::default().fg(colors.border))
        .highlight_style(
            Style::default()
                .bg(colors.background)
                .fg(colors.text_selected)
                .add_modifier(Modifier::ITALIC)
                .add_modifier(Modifier::BOLD),
        );
    frame.render_widget(
        list,
        fav_pane.rect,
    )
}

fn render_file_panes(frame: &mut Frame, file_panes: &FilePanes, colors: &config::FilePanesColors) {

    for (i, file_pane) in file_panes.panes.iter().enumerate() {
        render_pane(frame, file_pane, colors, file_panes.focused == Some(i));
    }
}

pub fn render_pane(frame: &mut Frame, pane: &file_pane::FilePane, colors: &config::FilePanesColors, is_focused: bool) {

    frame.render_widget(
        get_pane_list(&pane, colors, is_focused),
        pane.rect,
    );
}

fn trim_filename(name: &String) -> String {
    if name.len() < 16 {
        name.to_string()
    }
    else {
        let start = name
            .chars()
            .take(8)
            .collect::<String>();
        let end = name
            .chars()
            .rev()
            .take(8)
            .collect::<String>()
            .chars()
            .rev()
            .collect::<String>();

        format!("{}{}{}", start, "...", end)
    }
}

fn get_pane_list(file_pane: &file_pane::FilePane, colors: &config::FilePanesColors, is_focused: bool) -> List<'static> {
    let selected_bg = if is_focused {
        colors.selected_focus
    }
    else {
        colors.selected_no_focus
    };

    let first_pane_files: Vec<ListItem> = file_pane
        .entries
        .iter()
        .enumerate()
        .map(|(i, f)| {
            let lines = vec![Line::from(trim_filename(
                &f.to_owned()
                .file_name()
                .into_string()
                .unwrap_or(String::from(""))
            ))];
            if file_pane.selected == Some(i) {
                ListItem::new(lines).style(Style::default().fg(colors.text_selected).bg(selected_bg))
            } 
            else {
                ListItem::new(lines).style(Style::default().fg(colors.text_default))
            }
        })
        .skip(file_pane.scroll_offset)
        .collect();

    List::new(first_pane_files)
        .block(Block::default().borders(Borders::RIGHT))
        .style(Style::default().fg(colors.border))
        .highlight_style(
            Style::default()
                .bg(selected_bg)
                .fg(colors.text_selected)
                .add_modifier(Modifier::ITALIC)
                .add_modifier(Modifier::BOLD),
        )
}

fn render_preview_pane(frame: &mut Frame, preview_pane: &PreviewPane, colors: &config::FilePanesColors) {

    let vertical_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(40), // Top space
            Constraint::Percentage(20), // Widget space
            Constraint::Percentage(40), // Bottom space
        ])
        .split(preview_pane.rect);

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(40), // left part
            Constraint::Percentage(20), // middle band (to simulate center offset)
            Constraint::Percentage(40), // right part
        ])
        .split(vertical_chunks[1]);

    let p = Paragraph::new(format!("{}\n{}", preview_pane.file_name, preview_pane.size))
    .style(Style::default().fg(Color::Yellow))
    .block(
        Block::default()
    )
    .alignment(Alignment::Left);

    frame.render_widget(p, chunks[1]);
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
