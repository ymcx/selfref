use crate::{calculate, misc};
use fontdue::Font;
use image::{Rgba, RgbaImage};
use std::{collections::HashMap, error::Error};

fn find_matching_values(
    text: &str,
    font_size: (usize, usize),
    pixels: &HashMap<char, Vec<usize>>,
) -> Vec<(usize, usize)> {
    let mut results = Vec::new();
    let placeholder_count = text.matches("%s").count();
    let text = text.replace("%s", "");

    for (i, font_size) in (font_size.0..=font_size.1).enumerate() {
        let text_pixel_count: usize = text
            .chars()
            .map(|character| pixels.get(&character).unwrap()[i])
            .sum();
        for number in 0..100000 {
            let number_pixel_count: usize = number
                .to_string()
                .chars()
                .map(|character| pixels.get(&character).unwrap()[i])
                .sum();
            if number == number_pixel_count * placeholder_count + text_pixel_count {
                let result = (number, font_size);
                results.push(result);
            }
        }
    }

    results
}

fn render_image(
    text: &str,
    font: &Font,
    font_size: usize,
    fg_color: &Rgba<u8>,
    bg_color: &Rgba<u8>,
) -> Result<RgbaImage, Box<dyn Error>> {
    let (width, height) = calculate::image_dimensions(text, font, font_size);
    let mut image = RgbaImage::from_pixel(width as u32, height as u32, *bg_color);
    let layout = misc::get_layout(text, font, font_size);

    for glyph in layout.glyphs() {
        let (metrics, bitmap) = font.rasterize_indexed(glyph.key.glyph_index, glyph.key.px);
        for (i, intensity) in bitmap.into_iter().enumerate() {
            if intensity > u8::MAX / 2 {
                let x = (i % metrics.width) as u32 + glyph.x as u32;
                let y = (i / metrics.width) as u32 + glyph.y as u32;
                image.put_pixel(x, y, *fg_color);
            }
        }
    }

    Ok(image)
}

pub fn generate_image(
    text: &str,
    font: &str,
    font_size: &Vec<usize>,
    fg_color: &Vec<u8>,
    bg_color: &Vec<u8>,
) -> Result<(), Box<dyn Error>> {
    let font = misc::get_font(font)?;
    let font_size = misc::get_font_size(font_size)?;
    let fg_color = misc::get_color(fg_color)?;
    let bg_color = misc::get_color(bg_color)?;

    let pixels = calculate::pixels_in_characters(text, &font, font_size);
    let results = find_matching_values(text, font_size, &pixels);

    misc::recreate_directory("images")?;
    for (pixels, font_size) in results {
        let file = format!("images/{}.png", pixels);
        let text_final = text.replace("%s", &pixels.to_string());
        let image = render_image(&text_final, &font, font_size, &fg_color, &bg_color)?;
        image.save(file)?;
    }

    Ok(())
}
