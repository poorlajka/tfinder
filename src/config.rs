use toml;
use serde::Deserialize;
use crate::Color;
use std::fs;


/* 
* TODO: surely there's a better way to do this but can't find it in the crate documentation rn and
* I'm lazy as fuck so this works for now ig
*/

#[derive(Debug, Deserialize)]
struct RawConfig {
    pub colors: RawColors,
}

#[derive(Debug, Deserialize)]
struct RawColors {
    pub file_panes: RawFilePanesColors,
    pub path_trail: RawPathTrailColors,
    pub prompt_bar: RawPromptBarColors,
}

#[derive(Debug, Deserialize)]
struct RawFilePanesColors {
    pub background: String,
    pub border: String,
    pub hover: String,
    pub selected_focus: String,
    pub selected_no_focus: String,
    pub text_default: String,
    pub text_selected: String,
}

#[derive(Debug, Deserialize)]
struct RawPathTrailColors {
    pub background: String,
    pub text_default: String,
    pub text_hovered: String,
}

#[derive(Debug, Deserialize)]
struct RawPromptBarColors {
    pub background: String,
    pub text_default: String,
    pub text_hovered: String,
    pub text_prompt: String,
}

pub struct Config {
    pub colors: Colors,
}

pub struct Colors {
    pub file_panes: FilePanesColors,
    pub path_trail: PathTrailColors,
    pub prompt_bar: PromptBarColors,
}

pub struct FilePanesColors {
    pub background: Color,
    pub border: Color,
    pub hover: Color,
    pub selected_focus: Color,
    pub selected_no_focus: Color,
    pub text_default: Color,
    pub text_selected: Color,
}

pub struct PathTrailColors {
    pub background: Color,
    pub text_default: Color,
    pub text_hovered: Color,
}

pub struct PromptBarColors {
    pub background: Color,
    pub text_default: Color,
    pub text_hovered: Color,
    pub text_prompt: Color,
}

pub fn parse() -> Result<Config, std::io::Error> {

    let config_str = fs::read_to_string("/home/viktor/programming/term-finder/settings.toml")?;
    let raw_config: RawConfig = toml::from_str(&config_str)?;

    Ok(Config::from_raw(&raw_config))
}

impl Config {
    fn from_raw(raw_config: &RawConfig) -> Self {
        Config {
            colors: Colors::from_raw(&raw_config.colors),
        }
    }
}

impl Colors {
    fn from_raw(raw_colors: &RawColors) -> Self {
        Colors {
            file_panes: FilePanesColors::from_raw(&raw_colors.file_panes),
            path_trail: PathTrailColors::from_raw(&raw_colors.path_trail),
            prompt_bar: PromptBarColors::from_raw(&raw_colors.prompt_bar),

        }
    }
}

impl FilePanesColors {
    fn from_raw(raw_colors: &RawFilePanesColors) -> Self {
        FilePanesColors {
            background: parse_color(&raw_colors.background),
            border: parse_color(&raw_colors.border),
            hover: parse_color(&raw_colors.hover),
            selected_focus: parse_color(&raw_colors.selected_focus),
            selected_no_focus: parse_color(&raw_colors.selected_no_focus),
            text_default: parse_color(&raw_colors.text_default),
            text_selected: parse_color(&raw_colors.text_selected),
        }
    }
}

impl PathTrailColors {
    fn from_raw(raw_colors: &RawPathTrailColors) -> Self {
        PathTrailColors {
            background: parse_color(&raw_colors.background),
            text_default: parse_color(&raw_colors.text_default),
            text_hovered: parse_color(&raw_colors.text_hovered),
        }
    }
}

impl PromptBarColors {
    fn from_raw(raw_colors: &RawPromptBarColors) -> Self {
        PromptBarColors {
            background: parse_color(&raw_colors.background),
            text_default: parse_color(&raw_colors.text_default),
            text_hovered: parse_color(&raw_colors.text_hovered),
            text_prompt: parse_color(&raw_colors.text_prompt),
        }
    }
}

pub fn parse_color (color_str: &str) -> Color {
    let color_str = color_str.to_lowercase();

    if color_str.starts_with('#') {
        match parse_rbg(&color_str) {
            Ok(color) => color,
            Err(_) => Color::Red,
        }
    }

    else {
        match color_str.as_str() {
            "black" => Color::Black,
            "red" => Color::Red,
            "green" => Color::Green,
            "yellow" => Color::Yellow,
            "blue" => Color::Blue,
            "magenta" => Color::Magenta,
            "cyan" => Color::Cyan,
            "gray" => Color::Gray,
            "darkgray" => Color::DarkGray,
            "lightred" => Color::LightRed,
            "lightyellow" => Color::LightYellow,
            "lightblue" => Color::LightBlue,
            "lightmagenta" => Color::LightMagenta,
            "lightcyan" => Color::LightCyan,
            "white" => Color::White,
            _ => Color::Red,
        }
    }
}

fn parse_rbg(hex_code: &str) -> Result<Color, std::num::ParseIntError> {
    if hex_code.chars().count() != 7 {
        return Ok(Color::Red);
    }

    let r: u8 = u8::from_str_radix(&hex_code[1..3], 16)?;
    let g: u8 = u8::from_str_radix(&hex_code[3..5], 16)?;
    let b: u8 = u8::from_str_radix(&hex_code[5..7], 16)?;

    Ok(Color::Rgb(r, g, b))
}
