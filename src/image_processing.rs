use crate::configuration::{AppConfig, MapCrop, Alignment};
use crate::text_processing::process_text;
use ab_glyph::{FontRef, PxScale};
use image::{ImageBuffer, Rgb, RgbImage};
use imageproc::drawing::text_size;
use std::fs;

fn load_font_data(path: &str) -> Vec<u8> {
    fs::read(path).expect("Error reading font file")
}

fn create_font_ref<'a>(font_data: &'a [u8]) -> FontRef<'a> {
    FontRef::try_from_slice(font_data).expect("Error constructing FontRef")
}

pub fn create_layout(config: &AppConfig, name: &str, number: &str) -> RgbImage {
    let font_data = load_font_data(&config.font.path_regular);
    let font_regular = create_font_ref(&font_data);
    let font_data = load_font_data(&config.font.path_bold);
    let font_bold = create_font_ref(&font_data);

    let title_scale = PxScale::from(config.font.size_title);
    let subtitle_scale = PxScale::from(config.font.size_subtitle);

    let text_title = &config.layout.text_title;
    let text_subtitle_left = &config.layout.text_subtitle_left;
    let text_subtitle_right = &config.layout.text_subtitle_right;

    let mut layout = ImageBuffer::from_pixel(
        config.layout.width,
        config.layout.height,
        Rgb([255u8, 255u8, 255u8]),
    );

    let (title_w, _) = text_size(title_scale, &font_bold, text_title);
    let (subtitle_right_w, _) = text_size(subtitle_scale, &font_regular, &text_subtitle_right);

    let title_x = (layout.width() - title_w) / 2;
    let subtitle_left_x = config.layout.margin;
    let subtitle_right_x = layout.width() - subtitle_right_w - config.layout.margin;

    let title_y = config.layout.margin;
    let subtitle_y = title_y + config.layout.title_margin;

    let variables = vec![
        ("zone_name".to_string(), name.to_string()),
        ("territory_number".to_string(), number.to_string()),
    ];

    process_text(
        text_title,
        &variables,
        &font_regular,
        &font_bold,
        title_scale,
        &mut layout,
        title_x,
        title_y,
        Alignment::Center,
    );
    process_text(
        text_subtitle_left,
        &variables,
        &font_regular,
        &font_bold,
        subtitle_scale,
        &mut layout,
        subtitle_left_x,
        subtitle_y,
        Alignment::Left,
    );
    process_text(
        &text_subtitle_right,
        &variables,
        &font_regular,
        &font_bold,
        subtitle_scale,
        &mut layout,
        subtitle_right_x,
        subtitle_y,
        Alignment::Right,
    );
    layout
}

pub fn add_map_image(layout: &mut RgbImage, map_image_path: &str, margin: u32, map_crop: MapCrop) {
    let map_image = image::open(map_image_path).expect("Error opening map image");
    let map_image = map_image.to_rgb8();

    let (width, height) = map_image.dimensions();
    let (top, left, bottom, right) = (map_crop.top, map_crop.left, map_crop.bottom, map_crop.right);
    let crop_x = left;
    let crop_y = top;
    let crop_width = width - right - left;
    let crop_height = height - bottom - top;

    if crop_width <= 0 || crop_height <= 0 {
        panic!("Invalid crop dimensions, resulting width or height is zero or negative.");
    }

    let cropped_map =
        image::imageops::crop_imm(&map_image, crop_x, crop_y, crop_width, crop_height).to_image();

    let (target_w, target_h) = (layout.width() - 2 * margin, layout.height() - 2 * margin);
    // Calculate the scaling factor to maintain aspect ratio
    let scale_factor = f32::min(
        target_w as f32 / cropped_map.width() as f32,
        target_h as f32 / cropped_map.height() as f32,
    );

    // Compute the new dimensions that preserve the aspect ratio
    let new_w = (cropped_map.width() as f32 * scale_factor).round() as u32;
    let new_h = (cropped_map.height() as f32 * scale_factor).round() as u32;

    let resized_map = image::imageops::resize(
        &cropped_map,
        new_w,
        new_h,
        image::imageops::FilterType::Lanczos3,
    );

    let (overlay_x, overlay_y) = (
        (margin + (target_w - new_w) / 2).into(),
        (layout.height() - margin - new_h).into(),
    );

    image::imageops::overlay(layout, &resized_map, overlay_x, overlay_y);
}
