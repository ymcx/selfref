use image::{GenericImageView, Rgba};

pub fn white_pixel_count(file: &str) -> usize {
    let image = image::open(file).unwrap();
    let white = Rgba([255, 255, 255, 255]);
    let count = image.pixels().filter(|pixel| pixel.2 == white).count();

    count
}
