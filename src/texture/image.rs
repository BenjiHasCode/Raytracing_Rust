use image::RgbImage;

use crate::vec3::{Point3, Color};

use super::Texture;

pub struct ImageTexture {
    img: RgbImage
}

impl ImageTexture {
    pub fn new(filename: &str) -> Self {
        let img = image::open(filename).unwrap();
        let img = img.to_rgb8();

        Self { img }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _p: &Point3) -> Color {
        // Clamp input texture coordinates to [0,1] x [1,0]
        let u = u.clamp( 0.0, 1.0);
        let v = 1.0 - v.clamp(0.0, 1.0);

        let width = self.img.width();
        let height = self.img.height();

        let i = (u * width as f64) as u32;
        let j = (v * height as f64) as u32;

        // Clamp integer mapping, since actual coordinates should be less than 1.0
        let i = i.min(width - 1);
        let j = j.min(height - 1);

        const COLOR_SCALE: f64 = 1.0 / 255.0;

        let pixel = self.img.get_pixel(i, j);
        let r = pixel[0] as f64 * COLOR_SCALE;
        let g = pixel[1] as f64 * COLOR_SCALE;
        let b = pixel[2] as f64 * COLOR_SCALE;

        Color::new(r, g, b)
    }
}