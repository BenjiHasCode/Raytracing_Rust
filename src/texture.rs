use std::sync::Arc;

use image::RgbImage;

use crate::{vec3::{Color, Point3}, perlin::Perlin};

pub trait Texture: Send + Sync {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color;
}

//------- SOLIDCOLOR ----------
pub struct SolidColor {
    color_value: Color
}

impl SolidColor {
    pub fn new(r: f64, g: f64, b:f64) -> Self {
        SolidColor { color_value: Color::new(r, g, b) }
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: &Point3) -> Color {
        self.color_value
    }
}
//----------- CHECKER ----------------
pub struct CheckerTexture {
    pub odd: Arc<dyn Texture>,
    pub even: Arc<dyn Texture>
}

impl CheckerTexture {
    pub fn new(even: Color, odd: Color) -> Self {
        CheckerTexture {
            odd: Arc::new(SolidColor::new(odd.x, odd.y, odd.z)),
            even: Arc::new(SolidColor::new(even.x, even.y, even.z))
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        let sines = f64::sin(10.0 * p.x) * f64::sin(10.0 * p.y) * f64::sin(10.0 * p.z);
        if sines < 0.0 {
            return self.odd.value(u, v, p);
        } else {
            return self.even.value(u, v, p);
        }
    }
}


//----------- NOISE --------------
pub struct NoiseTexture {
    noise: Perlin,
    scale: f64
}

impl NoiseTexture {
    pub fn new(scale: f64) -> Self {
        Self { noise: Perlin::new(), scale }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: &Point3) -> Color {
        Color::new(1.0, 1.0, 1.0) * 0.5
         * (1.0 + (self.scale*p.z + 10.0*self.noise.turb(p, 7)).sin())
    }
}




//----------- IMAGE --------------
pub struct ImageTexture {
    img: RgbImage
}

impl ImageTexture {
    pub fn new(filename: &str) -> Self {
        // load image
        let img = image::open(filename).unwrap().to_rgb8();
        // TODO error checking

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