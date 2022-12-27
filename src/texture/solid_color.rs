use crate::vec3::{Color, Point3};

use super::Texture;

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