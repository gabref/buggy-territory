use crossterm::style::{Color, Stylize};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

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
        let settings = config::Config::builder()
            .add_source(config::File::with_name("config").required(false))
            .build()?;
        settings.try_deserialize()
    }

    fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let toml_str = toml::to_string(self)?;
        std::fs::write("config.toml", toml_str)?;
        Ok(())
    }

    // Save the current configuration to a file
    pub fn save_config(&self) {
        if self.save().is_ok() {
            println!(
                "\r{}",
                "Configuration saved successfully.".with(Color::Green)
            );
        } else {
            println!("\r{}", "Failed to save configuration.".with(Color::Red));
        }
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

// Enum to represent all configurable fields in AppConfig
pub enum ConfigField {
    OutputDirectory,
    FontPathRegular,
    FontPathBold,
    FontSizeTitle,
    FontSizeSubtitle,
    LayoutWidth,
    LayoutHeight,
    LayoutMargin,
    LayoutTitleMargin,
    LayoutTextTitle,
    LayoutTextSubtitleLeft,
    LayoutTextSubtitleRight,
    MapDirectory,
    MapCropTop,
    MapCropLeft,
    MapCropBottom,
    MapCropRight,
}

// Implement FromStr for ConfigField to parse field names from strings
impl FromStr for ConfigField {
    type Err = ();

    fn from_str(input: &str) -> Result<ConfigField, Self::Err> {
        match input {
            "Output Directory" => Ok(ConfigField::OutputDirectory),
            "Font - Regular Path" => Ok(ConfigField::FontPathRegular),
            "Font - Bold Path" => Ok(ConfigField::FontPathBold),
            "Font - Title Size" => Ok(ConfigField::FontSizeTitle),
            "Font - Subtitle Size" => Ok(ConfigField::FontSizeSubtitle),
            "Layout Width" => Ok(ConfigField::LayoutWidth),
            "Layout Height" => Ok(ConfigField::LayoutHeight),
            "Layout Margin" => Ok(ConfigField::LayoutMargin),
            "Title Margin" => Ok(ConfigField::LayoutTitleMargin),
            "Text Title" => Ok(ConfigField::LayoutTextTitle),
            "Text Subtitle Left" => Ok(ConfigField::LayoutTextSubtitleLeft),
            "Text Subtitle Right" => Ok(ConfigField::LayoutTextSubtitleRight),
            "Maps Directory" => Ok(ConfigField::MapDirectory),
            "Map Crop - Top" => Ok(ConfigField::MapCropTop),
            "Map Crop - Left" => Ok(ConfigField::MapCropLeft),
            "Map Crop - Bottom" => Ok(ConfigField::MapCropBottom),
            "Map Crop - Right" => Ok(ConfigField::MapCropRight),
            _ => Err(()),
        }
    }
}

// Implement conversion to and from strings for field values
impl AppConfig {
    // Get the string value of a field
    pub fn get_field_value(&self, field: &ConfigField) -> String {
        match field {
            ConfigField::OutputDirectory => self.output_directory.clone(),
            ConfigField::FontPathRegular => self.font.path_regular.clone(),
            ConfigField::FontPathBold => self.font.path_bold.clone(),
            ConfigField::FontSizeTitle => self.font.size_title.to_string(),
            ConfigField::FontSizeSubtitle => self.font.size_subtitle.to_string(),
            ConfigField::LayoutWidth => self.layout.width.to_string(),
            ConfigField::LayoutHeight => self.layout.height.to_string(),
            ConfigField::LayoutMargin => self.layout.margin.to_string(),
            ConfigField::LayoutTitleMargin => self.layout.title_margin.to_string(),
            ConfigField::LayoutTextTitle => self.layout.text_title.clone(),
            ConfigField::LayoutTextSubtitleLeft => self.layout.text_subtitle_left.clone(),
            ConfigField::LayoutTextSubtitleRight => self.layout.text_subtitle_right.clone(),
            ConfigField::MapDirectory => self.map.maps_directory.clone(),
            ConfigField::MapCropTop => self.map.crop.top.to_string(),
            ConfigField::MapCropLeft => self.map.crop.left.to_string(),
            ConfigField::MapCropBottom => self.map.crop.bottom.to_string(),
            ConfigField::MapCropRight => self.map.crop.right.to_string(),
        }
    }

    // Set the string value of a field
    pub fn set_field_value(&mut self, field: &ConfigField, value: String) {
        match field {
            ConfigField::OutputDirectory => self.output_directory = value,
            ConfigField::FontPathRegular => self.font.path_regular = value,
            ConfigField::FontPathBold => self.font.path_bold = value,
            ConfigField::FontSizeTitle => {
                if let Ok(v) = value.parse::<f32>() {
                    self.font.size_title = v;
                }
            }
            ConfigField::FontSizeSubtitle => {
                if let Ok(v) = value.parse::<f32>() {
                    self.font.size_subtitle = v;
                }
            }
            ConfigField::LayoutWidth => {
                if let Ok(v) = value.parse::<u32>() {
                    self.layout.width = v;
                }
            }
            ConfigField::LayoutHeight => {
                if let Ok(v) = value.parse::<u32>() {
                    self.layout.height = v;
                }
            }
            ConfigField::LayoutMargin => {
                if let Ok(v) = value.parse::<u32>() {
                    self.layout.margin = v;
                }
            }
            ConfigField::LayoutTitleMargin => {
                if let Ok(v) = value.parse::<u32>() {
                    self.layout.title_margin = v;
                }
            }
            ConfigField::LayoutTextTitle => self.layout.text_title = value,
            ConfigField::LayoutTextSubtitleLeft => self.layout.text_subtitle_left = value,
            ConfigField::LayoutTextSubtitleRight => self.layout.text_subtitle_right = value,
            ConfigField::MapDirectory => self.map.maps_directory = value,
            ConfigField::MapCropTop => {
                if let Ok(v) = value.parse::<u32>() {
                    self.map.crop.top = v;
                }
            }
            ConfigField::MapCropLeft => {
                if let Ok(v) = value.parse::<u32>() {
                    self.map.crop.left = v;
                }
            }
            ConfigField::MapCropBottom => {
                if let Ok(v) = value.parse::<u32>() {
                    self.map.crop.bottom = v;
                }
            }
            ConfigField::MapCropRight => {
                if let Ok(v) = value.parse::<u32>() {
                    self.map.crop.right = v;
                }
            }
        }
    }
}
