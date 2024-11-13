use ab_glyph::{FontRef, PxScale};
use image::{ImageBuffer, Rgb, RgbImage};
use imageproc::drawing::{draw_text_mut, text_size};
use std::fs;
use std::path::Path;

struct Config {
    font_path_regular: String,
    font_path_bold: String,
    font_size_title: f32,
    font_size_subtitle: f32,
    layout_width: u32,
    layout_height: u32,
    output_path: String,
    image_path: String,
}

impl Config {
    fn default() -> Self {
        Self {
            font_path_regular: String::from("fonts/Roboto-Regular.ttf"),
            font_path_bold: String::from("fonts/Roboto-Bold.ttf"),
            font_size_title: 24.0,
            font_size_subtitle: 18.0,
            layout_width: 1000,
            layout_height: 707,
            output_path: String::from("layout.png"),
            image_path: String::from("./images/201.png"),
        }
    }
}

fn load_font_data(path: &str) -> Vec<u8> {
    fs::read(path).expect("Error reading font file")
}

fn create_font_ref<'a>(font_data: &'a [u8]) -> FontRef<'a> {
    FontRef::try_from_slice(font_data).expect("Error constructing FontRef")
}

fn create_layout(config: &Config) -> RgbImage {
    let font_data = load_font_data(&config.font_path_regular);
    let font_regular = create_font_ref(&font_data);
    let font_data = load_font_data(&config.font_path_bold);
    let font_bold = create_font_ref(&font_data);

    let title_scale = PxScale::from(config.font_size_title);
    let subtitle_scale = PxScale::from(config.font_size_subtitle);

    let text_title = "Piantina di territorio";
    let text_subtitle_left = "Congregazione Roma Pratolungo";
    // let subtitle_right = format!("ZONA {} N. {}", territory_name, territory_number);
    let text_subtitle_right = "ZONA Casal Monastero N. 200";

    let mut layout = ImageBuffer::from_pixel(
        config.layout_width,
        config.layout_height,
        Rgb([255u8, 255u8, 255u8]),
    );

    let (title_w, _) = text_size(title_scale, &font_bold, text_title);
    let (subtitle_right_w, _) = text_size(subtitle_scale, &font_regular, &text_subtitle_right);

    let title_x = ((layout.width() - title_w) / 2).try_into().unwrap();
    let subtitle_right_x = (layout.width() - subtitle_right_w - 20).try_into().unwrap();

    draw_text_mut(
        &mut layout,
        Rgb([0u8, 0u8, 0u8]),
        title_x,
        20,
        title_scale,
        &font_bold,
        text_title,
    );
    draw_text_mut(
        &mut layout,
        Rgb([0u8, 0u8, 0u8]),
        20,
        60,
        subtitle_scale,
        &font_bold,
        text_subtitle_left,
    );
    draw_text_mut(
        &mut layout,
        Rgb([0u8, 0u8, 0u8]),
        subtitle_right_x,
        60,
        subtitle_scale,
        &font_bold,
        text_subtitle_right,
    );
    layout
}

fn add_map_image(layout: &mut RgbImage, map_image_path: &str) {
    let map_image = image::open(map_image_path).expect("Error opening map image");
    let map_image = map_image.to_rgb8();

    let (width, height) = map_image.dimensions();
    let (top, left, bottom, right) = (100, 50, 77, 82);
    let crop_x = left;
    let crop_y = top;
    let crop_width = width - right - left;
    let crop_height = height - bottom - top;
    let margin = 20;

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

fn main() {
    let config = Config::default();

    let path = Path::new(&config.output_path);

    let mut layout = create_layout(&config);
    add_map_image(&mut layout, &config.image_path);

    layout.save(path).unwrap();
}
