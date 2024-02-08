use crate::app;
use crate::{
    Backend,
    Block,
    Borders,
    Color,
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

pub fn render_app<B: Backend>(frame: &mut Frame<B>, app: &app::App, render_config: &config::Config) {

    render_trail(frame, &app.path_trail);
    render_pane(frame, &app.first_pane, render_config, app.focus == app::Component::FirstPane);
    render_pane(frame, &app.second_pane, render_config, app.focus == app::Component::SecondPane);
    render_prompt(frame, &app.prompt);

    //TODO REDO THE THIRD PANE (Commented block below) RENDERING THIS IS UGLY AF
    let ascii_folder = "
░░░░░░░░░░░░░░░░░░░░
░▒▒▒▒▒░░░░░░░░░░░░░░
░▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒░
░▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒░
░▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒░
░▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒░
░▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒░
░▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒░
░░░░░░░░░░░░░░░░░░░░
░░░░░░░░░░░░░░░░░░░░
";

    let ascii_file = "
░░░▓▓▓▓▓▓▓▓▓▓░░░░░░░
░░░▓▒▒▒▒▒▒▒▒▓▓▓░░░░░
░░░▓▒▒▒▒▒▒▒▒▒▒▒▓░░░░
░░░▓▒▒▒▒▒▒▒▒▒▒▒▒▓░░░
░░░▓▒▒░░░░░░▒▒▒▒▓░░░
░░░▓▒▒▒▒▒▒▒▒▒▒▒▒▓░░░
░░░▓▒▒░░░░░░░░▒▒▓░░░
░░░▓▒▒▒▒▒▒▒▒▒▒▒▒▓░░░
░░░▓▒▒░░░░░░░░▒▒▓░░░
░░░▓▒▒▒▒▒▒▒▒▒▒▒▒▓░░░
";

    /*
    let (fwidth, fheight) = (frame.size().width, frame.size().height);

    app.second_pane.width = fwidth / 3;
    app.first_pane.width = fwidth / 3;
    let pane2_state = &app.second_pane.files.state;
    match pane2_state.selected() {
        Some(index) => {
            let file = &app.second_pane.entries[index];

            if let Some(file_name) = file.file_name().to_str() {
                let mut file_info = Paragraph::new("");
                if file.path().is_dir() {
                    file_info = Paragraph::new(
                        ascii_folder.to_owned() + "name: " + file_name + "\ntype: folder",
                    )
                    .style(Style::default().fg(Color::White));
                } else if file.path().is_file() {
                    file_info = Paragraph::new(
                        ascii_file.to_owned() + "name: " + file_name + "\ntype: file",
                    )
                    .style(Style::default().fg(Color::White));
                }

                frame.render_widget(
                    file_info,
                    Rect::new((fwidth / 3) * 2 + (fwidth / 4) / 3, fheight / 4, 20, 20),
                );
            }
        }
        None => {
            if let Some(index) = app.first_pane.files.state.selected() {
                let file = &app.first_pane.entries[index];
                if let Some(file_name) = file.file_name().to_str() {
                    let mut file_info = Paragraph::new("");
                    if file.path().is_dir() {
                        file_info = Paragraph::new(
                            ascii_folder.to_owned() + "name: " + file_name + "\ntype: folder",
                        )
                        .style(Style::default().fg(Color::White));
                    } else if file.path().is_file() {
                        file_info = Paragraph::new(
                            ascii_file.to_owned() + "name: " + file_name + "\ntype: file",
                        )
                        .style(Style::default().fg(Color::White));
                    }

                    frame.render_widget(
                        file_info,
                        Rect::new((fwidth / 3) * 2 + (fwidth / 4) / 3, fheight / 4, 20, 20),
                    );
                }
            }
        }
    }
    */
}

fn get_pane_list(file_pane: &file_pane::FilePane, config: &config::Config, is_focused: bool) -> List<'static> {
    let color = if is_focused {
        config.colors.file_panes.selected_focus
    }
    else {
        config.colors.file_panes.selected_no_focus
    };

    let first_pane_files: Vec<ListItem> = file_pane
        .files
        .items
        .iter()
        .map(|i| {
            let lines = vec![Line::from(i.to_owned().0)];
            ListItem::new(lines).style(Style::default().fg(config.colors.file_panes.text_default))
        })
        .collect();

    List::new(first_pane_files)
        .block(Block::default().borders(Borders::RIGHT))
        .style(Style::default().fg(config.colors.file_panes.border))
        .highlight_style(
            Style::default()
                .bg(color)
                .fg(config.colors.file_panes.text_selected)
                .add_modifier(Modifier::ITALIC)
                .add_modifier(Modifier::BOLD),
        )
}
pub fn render_pane<B: Backend>(frame: &mut Frame<B>, pane: &file_pane::FilePane, config: &config::Config, is_focused: bool) {

    frame.render_stateful_widget(
        get_pane_list(&pane, config, is_focused),
        pane.rect,
        &mut pane.files.state.clone(),
    );
}

pub fn render_trail<B: Backend>(frame: &mut Frame<B>, trail: &path_trail::PathTrail) {
    let mut pos: usize = 0;
    for (i, (name, _)) in trail.paths.iter().enumerate() {
        let mut paragraph = Paragraph::new(name.to_string());
        if let Some(index) = trail.hovered_path {
            if index == i {
                paragraph = paragraph.style(Style::default().fg(Color::LightMagenta));
            } else {
                paragraph = paragraph.style(Style::default().fg(Color::White));
            }
        } else {
            paragraph = paragraph.style(Style::default().fg(Color::White));
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

pub fn render_prompt<B: Backend>(frame: &mut Frame<B>, prompt: &prompt::Prompt) {

    let current_prompt = if prompt.is_active() {

        if prompt.tick < 5 {
            Paragraph::new(
                prompt.get_prompt_string() + &prompt.input.clone() + "_",
            )
            .style(Style::default().fg(Color::White).bg(Color::Black))
        }
        else {

            Paragraph::new(
                prompt.get_prompt_string() + &prompt.input.clone() + " ",
            )
            .style(Style::default().fg(Color::White).bg(Color::Black))
        }
    }
    else {

        Paragraph::new(
            "C Create    R Rename    M Move    D Delete    O Open    H Help    / Search    F fill",
        )
        .style(Style::default().fg(Color::White).bg(Color::Black))
    };

    frame.render_widget(current_prompt, prompt.rect);
}

