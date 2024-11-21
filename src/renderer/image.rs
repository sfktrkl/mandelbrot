use image::{Rgb, RgbImage};

pub fn render(width: u32, height: u32, grid: &[u8]) -> RgbImage {
    let mut img = RgbImage::new(width, height);
    for (i, &value) in grid.iter().enumerate() {
        let x = (i as u32) % width;
        let y = (i as u32) / width;
        img.put_pixel(x, y, Rgb([value, value, value]));
    }
    img
}
