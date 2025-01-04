use std::fs;

use serde::{Deserialize, Serialize};
use tui::style::Color;

#[derive(Debug, Serialize, Deserialize)]
struct App {
    app_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Theme {
    foreground: Option<String>,
    background: Option<String>,
    h_foreground: Option<String>,
    h_background: Option<String>,
    border: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub app_name: String,
    pub foreground: Color,
    pub background: Color,
    pub h_foreground: Color,
    pub h_background: Color,
    pub border: Color,
}

#[derive(Debug, Serialize, Deserialize)]
struct ConfToml {
    app: Option<App>,
    theme: Option<Theme>,
}

// Implement singleton pattern for Config

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

impl Config {
    pub fn new() -> Self {
        let conf_path = [home::home_dir()
            .unwrap()
            .as_path()
            .join(".config/rustify/config/config.toml")
        ];

        let mut content: String = "".to_owned();

        for config in conf_path.iter() {
            if config.exists() {
                content = fs::read_to_string(config).unwrap();
            }
        }

        let conf_toml: ConfToml = toml::from_str(&content).unwrap_or_else(|_| {
            eprintln!("Error: Could not read the config file. Using default values.");
            ConfToml {
                app: None,
                theme: None,
            }
        });

        let app_name = match conf_toml.app {
            Some(app) => app.app_name.unwrap_or("Rustify".to_string()),
            None => "Rustify".to_string(),
        };

        let (foreground, background, h_foreground, h_background, border) = match conf_toml.theme {
            Some(theme) => {
                // item, if error
                let map = |i: Option<String>, s: String| {
                    let rgb = i.clone();
                    match i.unwrap_or(s).to_ascii_lowercase().as_ref() {
                        "black" => Color::Black,
                        "blue" => Color::Blue,
                        "green" => Color::Green,
                        "red" => Color::Red,
                        "yellow" => Color::Yellow,
                        "magenta" => Color::Magenta,
                        "cyan" => Color::Cyan,
                        "gray" => Color::Gray,
                        "dark gray" => Color::DarkGray,
                        "light red" => Color::LightRed,
                        "light green" => Color::LightGreen,
                        "light yellow" => Color::LightYellow,
                        "light blue" => Color::LightBlue,
                        "light magenta" => Color::LightMagenta,
                        "light cyan" => Color::LightCyan,
                        "white" => Color::White,
                        _ => {
                            let colors: Vec<u8> = rgb.unwrap()
                            .split(|i| i == ',')
                            .map(|i| i.to_string().trim().parse().expect("Couldn't read RGB Values. Make sure each value is between 0 & 255"))
                            .collect();

                            if colors.len() == 3 {
                                Color::Rgb(colors[0], colors[1], colors[2])
                            } else {
                                eprintln!("Couldn't read RGB Values. Make sure each value is comma seperated");
                                Color::Black
                            }
                        }
                    }
                };

                let foreground = map(theme.foreground, "Light Cyan".to_string());
                let background = map(theme.background, "Black".to_string());
                let hfg = map(theme.h_foreground, "Black".to_string());
                let hbg = map(theme.h_background, "Light Green".to_string());
                let border = map(theme.border, "Light Green".to_string());

                (foreground, background, hfg, hbg, border)
            }

            None => (
                Color::LightCyan,
                Color::Black,
                Color::Black,
                Color::LightGreen,
                Color::LightGreen,
            ),
        };

        Self { 
            app_name,
            foreground,
            background,
            h_foreground,
            h_background,
            border,
        }
    }

    pub fn app_name(&self) -> String {
        self.app_name.clone()
    }

    pub fn foreground(&self) -> Color {
        self.foreground
    }

    pub fn background(&self) -> Color {
        self.background
    }

    pub fn h_foreground(&self) -> Color {
        self.h_foreground
    }

    pub fn h_background(&self) -> Color {
        self.h_background
    }

    pub fn border(&self) -> Color {
        self.border
    }
}