use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Alignment {
    Left,
    Center,
    Right,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub font: FontConfig,
    pub layout: LayoutConfig,
    pub map: MapConfig,
    pub output_directory: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FontConfig {
    pub path_regular: String,
    pub path_bold: String,
    pub size_title: f32,
    pub size_subtitle: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LayoutConfig {
    pub width: u32,
    pub height: u32,
    pub margin: u32,
    pub title_margin: u32,
    pub text_title: String,
    pub text_subtitle_left: String,
    pub text_subtitle_right: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MapConfig {
    pub maps_directory: String,
    pub crop: MapCrop,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct MapCrop {
    pub top: u32,
    pub left: u32,
    pub bottom: u32,
    pub right: u32,
}

impl AppConfig {
    pub fn load() -> Result<Self, config::ConfigError> {
        let settings = config::Config::builder().add_source(config::File::with_name("config").required(false)).build()?;
        settings.try_deserialize()
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let toml_str = toml::to_string(self)?;
        std::fs::write("config.toml", toml_str)?;
        Ok(())
    }

    pub fn default() -> Self {
        Self {
            font: FontConfig {
                path_regular: String::from("fonts/Roboto-Regular.ttf"),
                path_bold: String::from("fonts/Roboto-Bold.ttf"),
                size_title: 28.0,
                size_subtitle: 20.0,
            },
            layout: LayoutConfig {
                width: 1000,
                height: 707,
                margin: 30,
                title_margin: 40,
                text_title: String::from("Piantina di territorio"),
                text_subtitle_left: String::from("Congregazione **Roma** Pratolungo"),
                text_subtitle_right: String::from("**ZONA** <zone_name> **N.** <territory_number>"),
            },
            map: MapConfig {
                maps_directory: String::from("./maps"),
                crop: MapCrop {
                    top: 100,
                    left: 50,
                    bottom: 77,
                    right: 82,
                },
            },
            output_directory: String::from("layouts"),
        }
    }
}
