use crate::{perlin::Perlin, vec3::{Point3, Color}};

use super::Texture;

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