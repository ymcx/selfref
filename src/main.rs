use fontdue::{
    Font, FontSettings,
    layout::{CoordinateSystem, Layout, TextStyle},
};
use image::{GrayImage, Luma};
use std::env;
use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn pixels_in_character(character: char, px: usize) -> usize {
    let data: &[u8] = include_bytes!("/usr/share/fonts/open-sans/OpenSans-Regular.ttf");
    let settings = FontSettings::default();
    let font = Font::from_bytes(data, settings).unwrap();

    let (_, bitmap) = font.rasterize(character, px as f32);
    let pixels = bitmap.iter().filter(|&&pixel| pixel > 127).count();

    pixels
}

fn sizes_in_set(string: &str, px_range: (usize, usize)) -> HashMap<char, Vec<usize>> {
    let string = format!("{}0123456789", string);
    let mut sizes: HashMap<char, Vec<usize>> = HashMap::new();
    for character in string.chars().collect::<HashSet<_>>() {
        let mut sizes_character: Vec<usize> = Vec::new();
        for px in px_range.0..px_range.1 {
            let sizes_character_px = pixels_in_character(character, px);
            sizes_character.push(sizes_character_px);
        }
        sizes.insert(character, sizes_character);
    }

    sizes
}

fn render(string: &str, px: u32, file: &str, dimensions: (u32, u32)) {
    let mut image = GrayImage::new(dimensions.0 * px, dimensions.1 * px);

    let data = include_bytes!("/usr/share/fonts/open-sans/OpenSans-Regular.ttf") as &[u8];
    let settings = FontSettings::default();
    let font = Font::from_bytes(data, settings).unwrap();

    let mut layout = Layout::new(CoordinateSystem::PositiveYDown);
    let style = TextStyle::new(string, px as f32, 0);
    layout.append(&[font.clone()], &style);

    let pixel_white = Luma([255]);
    for glyph in layout.glyphs() {
        let (metrics, bitmap) = font.rasterize_indexed(glyph.key.glyph_index, glyph.key.px);
        for y in 0..metrics.height {
            for x in 0..metrics.width {
                let pixel = bitmap[y * metrics.width + x];
                if pixel < 128 {
                    continue;
                }

                image.put_pixel(
                    x as u32 + glyph.x as u32,
                    y as u32 + glyph.y as u32,
                    pixel_white,
                );
            }
        }
    }

    image.save(file).unwrap();
}

fn brute_force(
    string: &str,
    px_range: (usize, usize),
    pixels_range: (usize, usize),
    sizes: &HashMap<char, Vec<usize>>,
) -> Vec<(usize, usize)> {
    let mut results: Vec<(usize, usize)> = Vec::new();
    for pixels in pixels_range.0..pixels_range.1 {
        for (iteration, px) in (px_range.0..px_range.1).enumerate() {
            let string_numbers = format!("{}{}", string, pixels);
            let pixels_actual = string_numbers
                .chars()
                .map(|character| sizes.get(&character).unwrap()[iteration])
                .sum();
            if pixels == pixels_actual {
                results.push((pixels, px));
            }
        }
    }

    results
}

fn get_pixels_range(string_length: usize, sizes: &HashMap<char, Vec<usize>>) -> (usize, usize) {
    let max = string_length * sizes.values().map(|s| s.last().unwrap()).max().unwrap();

    (0, max)
}

fn main() {
    let string_parts: Vec<String> = env::args().skip(1).collect();
    let string = string_parts.join("");
    let px_range = (20, 24);
    let image_dimensions = ((string.len() as f32 * 0.75) as u32, 2);

    let pixels = sizes_in_set(&string, px_range);
    let pixels_range = get_pixels_range(string.len(), &pixels);
    let results = brute_force(&string, px_range, pixels_range, &pixels);

    fs::remove_dir_all("images").unwrap_or_default();
    fs::create_dir("images").unwrap();
    for (iteration, (pixels, px)) in results.iter().enumerate() {
        let file = format!("images/{}.png", iteration);
        let string_final = string_parts.join(&pixels.to_string());
        render(&string_final, *px as u32, &file, image_dimensions);
    }
}
