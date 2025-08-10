use fontdue::{Font, FontSettings};
use std::collections::{HashMap, HashSet};

fn pixels_in_character(character: char, px: usize) -> usize {
    let data: &[u8] = include_bytes!("/usr/share/fonts/open-sans/OpenSans-Regular.ttf");
    let settings = FontSettings::default();
    let font = Font::from_bytes(data, settings).unwrap();

    let (_, bitmap) = font.rasterize(character, px as f32);
    let pixels = bitmap.iter().filter(|&&pixel| pixel > 127).count();

    pixels
}

pub fn sizes_in_set(string: &str, px_range: (usize, usize)) -> HashMap<char, Vec<usize>> {
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

pub fn brute_force(
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

pub fn get_pixels_range(string_length: usize, sizes: &HashMap<char, Vec<usize>>) -> (usize, usize) {
    let max = string_length * sizes.values().map(|s| s.last().unwrap()).max().unwrap();

    (0, max)
}
