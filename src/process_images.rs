use crate::configuration::AppConfig;
use crate::image_processing::{add_map_image, create_layout};
use crate::text_processing::title_case;
use std::fs;
use std::path::Path;

pub fn process_images(config: &AppConfig) {
    let output_directory = Path::new(&config.output_directory);
    fs::create_dir_all(output_directory).expect("Failed to create output directory");

    let maps_directory = Path::new(&config.map.maps_directory);

    for entry in fs::read_dir(maps_directory).expect("Failed to read maps directory") {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.extension().map_or(false, |ext| ext == "png") {
                if let Some(filename) = path.file_stem().and_then(|f| f.to_str()) {
                    let parts: Vec<&str> = filename.splitn(2, '-').collect();
                    if parts.len() == 2 {
                        let territory_number = parts[0];
                        let zone_name = title_case(&parts[1].replace("-", " "));

                        let mut layout = create_layout(&config, &zone_name, territory_number);
                        add_map_image(
                            &mut layout,
                            path.to_str().unwrap(),
                            config.layout.margin,
                            config.map.crop,
                        );

                        let output_filename = format!(
                            "{}-{}.png",
                            territory_number,
                            zone_name.replace(" ", "-").to_lowercase()
                        );
                        let output_path = output_directory.join(output_filename);
                        layout
                            .save(output_path)
                            .expect("Failed to save layout image");
                    } else {
                        eprintln!("Invalid filename format: {}", filename);
                    }
                }
            }
        }
    }
}
