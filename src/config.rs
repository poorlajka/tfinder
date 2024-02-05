use std::fs;
use toml;
use serde::Deserialize;
use crate::Color;
use std::str::FromStr;


#[derive(Debug, Deserialize)]
pub struct Preconfig {
    colors: Precolors,
}

#[derive(Debug, Deserialize)]
pub struct Precolors {
    main: String,
}

pub struct Config {
    pub colors: Colors,
}
pub struct Colors {
    pub main: Color,
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

pub fn parse() -> Config {
    //Todo these two expects should probably just return a default config file instead
    let config_str = fs::read_to_string("/home/viktor/programming/term-finder/settings.toml").expect("Failed to read Cargo.toml file");
    let preconfig: Preconfig = toml::from_str(&config_str).expect("Failed to read config file!");
    let color_str = preconfig.colors.main.to_lowercase();

    let color = if color_str.starts_with('#') {
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
    };
    Config {
        colors: Colors {
            main: color
        }
    }

}
