use std::sync::Arc;

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
    pub fn new() -> Self {
        Self { noise: Perlin::new(), scale: 1.0}
    }

    pub fn new_scaled(scale: f64) -> Self {
        Self { noise: Perlin::new(), scale }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: &Point3) -> Color {
        Color::new(1.0, 1.0, 1.0) * 0.5
         * (1.0 + (self.scale*p.z + 10.0*self.noise.turb(p, 7)).sin())
    }
}