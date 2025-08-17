use crate::misc;
use fontdue::Font;
use image::GenericImageView;
use std::{
    collections::{HashMap, HashSet},
    error::Error,
};

pub fn pixels_in_image(file: &str, fg_color: &Vec<u8>) -> Result<usize, Box<dyn Error>> {
    let target_color = misc::get_color(fg_color)?;
    let pixels = image::open(file)?
        .pixels()
        .filter(|&(_, _, color)| color == target_color)
        .count();

    Ok(pixels)
}

fn pixels_in_character(character: char, font: &Font, font_size: usize) -> usize {
    let (_, bitmap) = font.rasterize(character, font_size as f32);
    let pixels = bitmap
        .into_iter()
        .filter(|&intensity| intensity > u8::MAX / 2)
        .count();

    pixels
}

pub fn pixels_in_characters(
    text: &str,
    font: &Font,
    font_size: (usize, usize),
) -> HashMap<char, Vec<usize>> {
    let characters: HashSet<char> = text.chars().chain("0123456789".chars()).collect();
    let pixels = characters
        .into_iter()
        .map(|character| {
            let pixels = (font_size.0..=font_size.1)
                .map(|font_size| pixels_in_character(character, font, font_size))
                .collect();
            (character, pixels)
        })
        .collect();

    pixels
}

pub fn image_dimensions(text: &str, font: &Font, font_size: usize) -> (usize, usize) {
    let layout = misc::get_layout(text, font, font_size);
    let dimensions = (
        layout
            .glyphs()
            .into_iter()
            .map(|glyph| glyph.x as usize + glyph.width)
            .max()
            .unwrap_or_default(),
        layout
            .glyphs()
            .into_iter()
            .map(|glyph| glyph.y as usize + glyph.height)
            .max()
            .unwrap_or_default(),
    );

    dimensions
}
