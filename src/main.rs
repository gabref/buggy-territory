mod configuration;
mod image_processing;
mod process_images;
mod text_processing;
mod ui;

use configuration::AppConfig;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    style::{Color, Stylize},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use process_images::process_images;
use ui::{
    clear_terminal, display_config, display_goodbye, display_header, display_menu, edit_config,
    pause_after_action,
};

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
            if key_event.kind != KeyEventKind::Press {
                continue;
            }
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
                        match process_images(&config) {
                            Ok(_) => {
                                pause_after_action(
                                    "Images processed.\n\rPress Enter to return to the menu...",
                                );
                            }
                            Err(e) => {
                                pause_after_action(&format!(
                                    "{}\n\r{}\n\n\r{:#?}",
                                    "An error occurred processing images :(.",
                                    "Press Enter to return to the menu...",
                                    e
                                ));
                            }
                        };
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
                        config.save_config();
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
