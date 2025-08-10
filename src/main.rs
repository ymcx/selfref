use std::{env, fs};

mod count;
mod draw;

fn main() {
    let string_parts: Vec<String> = env::args().skip(1).collect();
    let string = string_parts.join("");
    let px_range = (20, 24);
    let image_dimensions = ((string.len() as f32 * 0.75) as u32, 2);

    let pixels = count::sizes_in_set(&string, px_range);
    let pixels_range = count::get_pixels_range(string.len(), &pixels);
    let results = count::brute_force(&string, px_range, pixels_range, &pixels);

    fs::remove_dir_all("images").unwrap_or_default();
    fs::create_dir("images").unwrap();
    for (iteration, (pixels, px)) in results.iter().enumerate() {
        let file = format!("images/{}.png", iteration);
        let string_final = string_parts.join(&pixels.to_string());
        draw::render(&string_final, *px as u32, &file, image_dimensions);
    }
}
