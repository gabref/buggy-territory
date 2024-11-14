use crate::configuration::Alignment;
use ab_glyph::{FontRef, PxScale};
use image::{Rgb, RgbImage};
use imageproc::drawing::{draw_text_mut, text_size};

pub fn title_case(text: &str) -> String {
    text.split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            chars
                .next()
                .map(|c| c.to_uppercase().collect::<String>())
                .unwrap_or_default()
                + chars.as_str()
        })
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn process_text(
    text: &str,
    variables: &[(String, String)],
    font_regular: &FontRef,
    font_bold: &FontRef,
    scale: PxScale,
    layout: &mut RgbImage,
    x: u32,
    y: u32,
    alignment: Alignment,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut processed_text = text.to_string();
    let (original_width, _) = text_size(scale, font_regular, text);

    // replace variables in text
    for (var, value) in variables {
        processed_text = processed_text.replace(&format!("<{}>", var), value);
    }

    let clean_text = processed_text.replace("**", "");
    let (clean_width, _) = text_size(scale, font_regular, &clean_text);

    let mut new_x = x;
    // recalculates the right x position based on the alignment
    match alignment {
        Alignment::Left => (),
        Alignment::Center => {
            let diff = original_width - clean_width;
            new_x = x + diff / 2;
        }
        Alignment::Right => {
            let diff = original_width - clean_width;
            new_x = x + diff;
        }
    };

    // parse text for bold
    let mut cursor_x = new_x;
    let mut is_bold = false;
    let mut current_segment = String::new();
    let chars: Vec<char> = processed_text.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        if chars[i] == '*' && i + 1 < chars.len() && chars[i + 1] == '*' {
            // flush the current segment with the current font style
            let font = if is_bold { font_bold } else { font_regular };
            let (width, _) = text_size(scale, font, &current_segment);
            draw_text_mut(
                layout,
                Rgb([0u8, 0u8, 0u8]),
                cursor_x.try_into().map_err(|_| "Failed to convert x to u32")?,
                y.try_into().map_err(|_| "Failed to convert y to u32")?,
                scale,
                font,
                &current_segment,
            );
            cursor_x += width;
            current_segment.clear();

            // toggle bold state and skip next '*'
            is_bold = !is_bold;
            i += 1;
        } else {
            current_segment.push(chars[i]);
        }
        i += 1;
    }

    if !current_segment.is_empty() {
        let font = if is_bold { font_bold } else { font_regular };
        draw_text_mut(
            layout,
            Rgb([0u8, 0u8, 0u8]),
            cursor_x.try_into().map_err(|_| "Failed to convert x to u32")?,
            y.try_into().map_err(|_| "Failed to convert y to u32")?,
            scale,
            font,
            &current_segment,
        );
    };
    Ok(())
}
