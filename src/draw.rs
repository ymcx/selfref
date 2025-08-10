use fontdue::{
    Font, FontSettings,
    layout::{CoordinateSystem, Layout, TextStyle},
};
use image::{GrayImage, Luma};

pub fn render(string: &str, px: u32, file: &str, dimensions: (u32, u32)) {
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
