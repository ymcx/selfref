use fontdue::{
    Font, FontSettings,
    layout::{CoordinateSystem, Layout, TextStyle},
};
use image::Rgba;
use std::{error::Error, fs, path::Path};

pub fn get_font(file: &str) -> Result<Font, Box<dyn Error>> {
    let data = fs::read(file)?;
    let settings = FontSettings::default();
    let font = Font::from_bytes(data, settings)?;

    Ok(font)
}

pub fn get_font_size(font_size: &Vec<usize>) -> Result<(usize, usize), Box<dyn Error>> {
    match font_size.len() {
        1 => Ok((font_size[0], font_size[0])),
        2 => Ok((font_size[0], font_size[1])),
        _ => Err("Invalid font size".into()),
    }
}

pub fn get_color(color: &Vec<u8>) -> Result<Rgba<u8>, Box<dyn Error>> {
    match color.len() {
        3 => Ok(Rgba([color[0], color[1], color[2], 255])),
        4 => Ok(Rgba([color[0], color[1], color[2], color[3]])),
        _ => Err("Invalid color".into()),
    }
}

pub fn get_layout(text: &str, font: &Font, font_size: usize) -> Layout {
    let mut layout = Layout::new(CoordinateSystem::PositiveYDown);
    let style = TextStyle::new(text, font_size as f32, 0);
    layout.append(&[font], &style);

    layout
}

pub fn recreate_directory(directory: &str) -> Result<(), Box<dyn Error>> {
    let path = Path::new(directory);
    if path.is_dir() {
        fs::remove_dir_all(path)?;
    }
    fs::create_dir(path)?;

    Ok(())
}
