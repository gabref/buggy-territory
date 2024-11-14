use crate::configuration::{AppConfig, ConfigField};
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    style::{Color, Stylize},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

pub fn clear_terminal() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}

pub fn display_menu(menu_options: &[&str], selected_option: usize) {
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

pub fn display_config(config: &AppConfig) {
    println!("\n\rCurrent Configurations:\n");

    // Format the config struct as a pretty-printed string
    let formatted_config = format!("{:#?}", config);

    // Process each line to prepend '\r' and apply color formatting
    for line in formatted_config.lines() {
        println!("\r  {}", line.with(Color::Green));
    }
}

pub fn pause_after_action(message: &str) {
    println!("\r{}", message.with(Color::Yellow));
    loop {
        if let Event::Key(key) = event::read().expect("Failed to read key") {
            if key.kind != KeyEventKind::Press {
                continue;
            }
            if key.code == KeyCode::Enter
                || key.code == KeyCode::Esc
                || key.code == KeyCode::Char('q')
            {
                break;
            }
        }
    }
}

pub fn edit_config(config: &mut AppConfig) {
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
            if key.kind != KeyEventKind::Press {
                continue;
            }
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

// Get user input from the terminal
fn get_user_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().to_string()
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

pub fn display_header() {
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

pub fn display_edit_config_header() {
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

pub fn display_goodbye() {
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
