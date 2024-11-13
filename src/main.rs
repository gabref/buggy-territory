mod configuration;
mod image_processing;
mod text_processing;
mod process_images;

use configuration::AppConfig;
use process_images::process_images;

fn main() {
    let config = AppConfig::load().unwrap_or_else(|err| {
        eprintln!("Error loading configuration: {}", err);
        println!("Using default configuration");
        AppConfig::default()
    });

    process_images(&config);
}
