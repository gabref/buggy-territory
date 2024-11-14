use crate::configuration::AppConfig;
use crate::image_processing::{add_map_image, create_layout};
use crate::text_processing::title_case;
use crossterm::style::{Color, Stylize};
use indicatif::{ProgressBar, ProgressStyle};
use std::fs;
use std::path::Path;
use std::time::Instant;

pub fn process_images(config: &AppConfig) -> Result<(), Box<dyn std::error::Error>> {
    let output_directory = Path::new(&config.output_directory);
    fs::create_dir_all(output_directory)
        .map_err(|e| format!("Failed to create output directory: {}", e))?;

    let maps_directory = Path::new(&config.map.maps_directory);

    // Gather all entries to determine the total count
    let entries: Vec<_> = fs::read_dir(maps_directory)?.collect();
    let total_images = entries.len();

    // Initialize the progress bar
    let progress_bar = ProgressBar::new(total_images as u64);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .progress_chars("=>-")
            .template("\r{msg} [{bar:40.cyan/blue}] {pos}/{len} ({percent}%)")
            .map_err(|e| format!("Failed to create progress bar: {}", e))?,
    );
    progress_bar.set_message("\rProcessing images...");

    let mut success_count = 0;
    let mut failure_count = 0;
    let start_time = Instant::now();

    for entry in
        fs::read_dir(maps_directory).map_err(|e| format!("Failed to read directory: {}", e))?
    {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.extension().map_or(false, |ext| ext == "png") {
                if let Some(filename) = path.file_stem().and_then(|f| f.to_str()) {
                    let parts: Vec<&str> = filename.splitn(2, '-').collect();
                    if parts.len() == 2 {
                        let territory_number = parts[0];
                        let zone_name = title_case(&parts[1].replace("-", " "));

                        let result = (|| {
                            let mut layout = create_layout(&config, &zone_name, territory_number)?;
                            add_map_image(
                                &mut layout,
                                path.to_str().unwrap(),
                                config.layout.margin,
                                config.map.crop,
                            )?;

                            let output_filename = format!(
                                "{}-{}.png",
                                territory_number,
                                zone_name.replace(" ", "-").to_lowercase()
                            );
                            let output_path = output_directory.join(output_filename);
                            layout.save(output_path)?;
                            Ok::<(), Box<dyn std::error::Error>>(())
                        })();
                        match result {
                            Ok(_) => success_count += 1,
                            Err(e) => {
                                eprintln!(
                                    "\r{}",
                                    format!("Failed to process {}: {}", filename, e)
                                        .with(Color::Red)
                                );
                                failure_count += 1;
                            }
                        }
                    } else {
                        eprintln!(
                            "\r{}",
                            format!("Invalid filename format: {}", filename).with(Color::Red)
                        );
                    }
                }
            }
        }
        progress_bar.inc(1);
    }
    let elapsed = start_time.elapsed();

    progress_bar.finish_with_message("Processing complete");

    // Display summary
    println!("\n\n\r\t SUMMARY:");
    println!("\r\t Success: {}", success_count);
    println!("\r\t Failures: {}", failure_count);
    println!("\r\t Time taken: {:.2?}\n", elapsed);

    Ok(())
}
