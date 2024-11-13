mod configuration;
mod image_processing;
mod process_images;
mod text_processing;

use configuration::{AppConfig, ConfigField};
use crossterm::{
    event::{self, Event, KeyCode},
    style::{Color, Stylize},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use process_images::process_images;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?; // Enable raw mode to capture key events
    let mut config = AppConfig::load().unwrap_or_else(|err| {
        eprintln!("\rError loading configuration: {}", err);
        println!("\rUsing default configuration");
        AppConfig::default()
    });

    let mut selected_option = 0;
    let menu_options = [
        "Process images and create layouts",
        "View current configurations",
        "Edit configurations",
        "Save configurations",
        "Exit",
    ];

    loop {
        clear_terminal();
        display_header();
        display_menu(&menu_options, selected_option);

        if let Event::Key(key_event) = event::read().unwrap() {
            match key_event.code {
                KeyCode::Up => {
                    if selected_option > 0 {
                        selected_option -= 1;
                    }
                }
                KeyCode::Down => {
                    if selected_option < menu_options.len() - 1 {
                        selected_option += 1;
                    }
                }
                KeyCode::Enter => match selected_option {
                    0 => {
                        clear_terminal();
                        display_header();
                        println!("\r{}", "Processing images...\n".with(Color::Yellow));
                        process_images(&config);
                        pause_after_action(
                            "Images processed.\n\rPress Enter to return to the menu...",
                        );
                    }
                    1 => {
                        clear_terminal();
                        display_header();
                        display_config(&config);
                        pause_after_action("Press Enter to return to the menu...");
                    }
                    2 => {
                        clear_terminal();
                        display_header();
                        edit_config(&mut config);
                    }
                    3 => {
                        clear_terminal();
                        display_header();
                        save_config(&config);
                        pause_after_action(
                            "Configuration saved. Press Enter to return to the menu...",
                        );
                    }
                    4 => {
                        display_goodbye();
                        disable_raw_mode()?; // Restore terminal mode
                        return Ok(());
                    }
                    _ => {}
                },
                KeyCode::Esc | KeyCode::Char('q') => {
                    display_goodbye();
                    disable_raw_mode()?; // Restore terminal mode
                    return Ok(());
                }
                _ => {}
            }
        }
    }
}

// Clears the terminal screen
fn clear_terminal() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}

// Display menu options with arrow key selection
fn display_menu(menu_options: &[&str], selected_option: usize) {
    println!("\rPlease select an option:\n");
    for (index, option) in menu_options.iter().enumerate() {
        if index == selected_option {
            println!(
                "\r\t{}",
                format!("> {} <", option).on(Color::Cyan).with(Color::Black)
            ); // Highlight selected option
        } else {
            println!("\r\t  {}", option.with(Color::White));
        }
    }
    println!(
        "\n\r{}",
        "Use ↑ ↓ arrows to navigate and Enter to select. Press ESC or q to exit"
            .with(Color::DarkGrey)
    );
}

// Get user input from the terminal

fn get_user_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().to_string()
}

// Display current configurations
fn display_config(config: &AppConfig) {
    println!("\n\rCurrent Configurations:");
    println!("\r{}", format!("{:#?}", config).with(Color::Green));
}

fn edit_config(config: &mut AppConfig) {
    // Mapping field display labels to ConfigField variants for dynamic editing
    let config_fields = [
        ("Output Directory", ConfigField::OutputDirectory),
        ("Maps Directory", ConfigField::MapDirectory),
        ("Font - Regular Path", ConfigField::FontPathRegular),
        ("Font - Bold Path", ConfigField::FontPathBold),
        ("Font - Title Size", ConfigField::FontSizeTitle),
        ("Font - Subtitle Size", ConfigField::FontSizeSubtitle),
        ("Layout Width", ConfigField::LayoutWidth),
        ("Layout Height", ConfigField::LayoutHeight),
        ("Layout Margin", ConfigField::LayoutMargin),
        ("Title Margin", ConfigField::LayoutTitleMargin),
        ("Text Title", ConfigField::LayoutTextTitle),
        ("Text Subtitle Left", ConfigField::LayoutTextSubtitleLeft),
        ("Text Subtitle Right", ConfigField::LayoutTextSubtitleRight),
        ("Map Crop - Top", ConfigField::MapCropTop),
        ("Map Crop - Left", ConfigField::MapCropLeft),
        ("Map Crop - Bottom", ConfigField::MapCropBottom),
        ("Map Crop - Right", ConfigField::MapCropRight),
    ];

    let mut selected_option = 0;

    loop {
        clear_terminal();
        display_edit_config_header();
        println!(
            "\r    Edit Configurations (Use ↑ ↓ to navigate, Enter to edit, Esc to go back)\n"
        );

        // Display each configuration option with current values
        for (index, (label, field)) in config_fields.iter().enumerate() {
            let value = config.get_field_value(field); // Get the current value as a string
            if index == selected_option {
                println!(
                    "\r\t{}",
                    format!("> {}: {} <", label, value)
                        .on(Color::Cyan)
                        .with(Color::Black)
                );
            } else {
                println!(
                    "\r\t{}",
                    format!("  {}: {}", label, value).with(Color::White)
                );
            }
        }

        // Handle key events
        if let Event::Key(key) = event::read().expect("Failed to read event") {
            match key.code {
                KeyCode::Up => {
                    if selected_option > 0 {
                        selected_option -= 1;
                    }
                }
                KeyCode::Down => {
                    if selected_option < config_fields.len() - 1 {
                        selected_option += 1;
                    }
                }
                KeyCode::Enter => {
                    // Temporarily disable raw mode for text input
                    disable_raw_mode().expect("Failed to disable raw mode");

                    // Prompt user for new value
                    let clean_label = config_fields[selected_option].0;
                    let new_value = get_new_value(clean_label);

                    // Update the selected configuration value in `config`
                    let field = &config_fields[selected_option].1;
                    config.set_field_value(field, new_value);
                    // Re-enable raw mode
                    enable_raw_mode().expect("Failed to re-enable raw mode");
                }
                KeyCode::Esc | KeyCode::Char('q') => break,
                _ => {}
            }
        }
    }
}

// Helper function to get a new configuration value from the user
fn get_new_value(prompt: &str) -> String {
    print!(
        "\rt\t{}",
        format!("Enter new value for {}: ", prompt).with(Color::Yellow)
    );
    io::stdout().flush().unwrap();
    get_user_input()
}

// Save the current configuration to a file
fn save_config(config: &AppConfig) {
    if config.save().is_ok() {
        println!(
            "\r{}",
            "Configuration saved successfully.".with(Color::Green)
        );
    } else {
        println!("\r{}", "Failed to save configuration.".with(Color::Red));
    }
}

// Pauses the application until the user presses Enter
fn pause_after_action(message: &str) {
    println!("\r{}", message.with(Color::Yellow));
    loop {
        if let Event::Key(key) = event::read().expect("Failed to read key") {
            if key.code == KeyCode::Enter
                || key.code == KeyCode::Esc
                || key.code == KeyCode::Char('q')
            {
                break;
            }
        }
    }
}

// Display header with ASCII art
fn display_header() {
    println!(
        "{}",
        "\n\n\
\r    ██████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████\n\
\r    █▌                                                                                                                          ▐█\n\
\r    █▌  ██████╗ ██╗   ██╗ ██████╗  ██████╗██╗   ██╗    ████████╗███████╗██████╗ ██████╗ ██╗████████╗ ██████╗ ██████╗ ██╗   ██╗  ▐█\n\
\r    █▌  ██╔══██╗██║   ██║██╔════╝ ██╔════╝╚██╗ ██╔╝    ╚══██╔══╝██╔════╝██╔══██╗██╔══██╗██║╚══██╔══╝██╔═══██╗██╔══██╗╚██╗ ██╔╝  ▐█\n\
\r    █▌  ██████╔╝██║   ██║██║  ███╗██║  ███╗╚████╔╝        ██║   █████╗  ██████╔╝██████╔╝██║   ██║   ██║   ██║██████╔╝ ╚████╔╝   ▐█\n\
\r    █▌  ██╔══██╗██║   ██║██║   ██║██║   ██║ ╚██╔╝         ██║   ██╔══╝  ██╔══██╗██╔══██╗██║   ██║   ██║   ██║██╔══██╗  ╚██╔╝    ▐█\n\
\r    █▌  ██████╔╝╚██████╔╝╚██████╔╝╚██████╔╝  ██║          ██║   ███████╗██║  ██║██║  ██║██║   ██║   ╚██████╔╝██║  ██║   ██║     ▐█\n\
\r    █▌  ╚═════╝  ╚═════╝  ╚═════╝  ╚═════╝   ╚═╝          ╚═╝   ╚══════╝╚═╝  ╚═╝╚═╝  ╚═╝╚═╝   ╚═╝    ╚═════╝ ╚═╝  ╚═╝   ╚═╝     ▐█\n\
\r    █▌                                                                                                                          ▐█\n\
\r    ██████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████\n\n"
            .with(Color::Cyan)
    );
}

fn display_edit_config_header() {
    println!(
        "{}",
        "\n\n\
\r    ███████████████████████████████████████████████████████████████████████████████████████████████\n\
\r    █▌                                                                                           ▐█\n\
\r    █▌  ███████╗██████╗ ██╗████████╗     ██████╗ ██████╗ ███╗   ██╗███████╗██╗ ██████╗ ███████╗  ▐█\n\
\r    █▌  ██╔════╝██╔══██╗██║╚══██╔══╝    ██╔════╝██╔═══██╗████╗  ██║██╔════╝██║██╔════╝ ██╔════╝  ▐█\n\
\r    █▌  █████╗  ██║  ██║██║   ██║       ██║     ██║   ██║██╔██╗ ██║█████╗  ██║██║  ███╗███████╗  ▐█\n\
\r    █▌  ██╔══╝  ██║  ██║██║   ██║       ██║     ██║   ██║██║╚██╗██║██╔══╝  ██║██║   ██║╚════██║  ▐█\n\
\r    █▌  ███████╗██████╔╝██║   ██║       ╚██████╗╚██████╔╝██║ ╚████║██║     ██║╚██████╔╝███████║  ▐█\n\
\r    █▌  ╚══════╝╚═════╝ ╚═╝   ╚═╝        ╚═════╝ ╚═════╝ ╚═╝  ╚═══╝╚═╝     ╚═╝ ╚═════╝ ╚══════╝  ▐█\n\
\r    █▌                                                                                           ▐█\n\
\r    ███████████████████████████████████████████████████████████████████████████████████████████████\n\n"
            .with(Color::Cyan)
    );
}

// Display goodbye screen with ASCII art and pause before exiting
fn display_goodbye() {
    clear_terminal();

    println!(
        "{}",
        "\n\n\
\r    ████████████████████████████████████████████████████████████████████████\n\
\r    █▌                                                                    ▐█\n\
\r    █▌   ██████╗  ██████╗  ██████╗ ██████╗     ██████╗ ██╗   ██╗███████╗  ▐█\n\
\r    █▌  ██╔════╝ ██╔═══██╗██╔═══██╗██╔══██╗    ██╔══██╗╚██╗ ██╔╝██╔════╝  ▐█\n\
\r    █▌  ██║  ███╗██║   ██║██║   ██║██║  ██║    ██████╔╝ ╚████╔╝ █████╗    ▐█\n\
\r    █▌  ██║   ██║██║   ██║██║   ██║██║  ██║    ██╔══██╗  ╚██╔╝  ██╔══╝    ▐█\n\
\r    █▌  ╚██████╔╝╚██████╔╝╚██████╔╝██████╔╝    ██████╔╝   ██║   ███████╗  ▐█\n\
\r    █▌   ╚═════╝  ╚═════╝  ╚═════╝ ╚═════╝     ╚═════╝    ╚═╝   ╚══════╝  ▐█\n\
\r    █▌                                                                    ▐█\n\
\r    ████████████████████████████████████████████████████████████████████████\n"
            .with(Color::Cyan)
    );
    thread::sleep(Duration::from_secs(2));
}
