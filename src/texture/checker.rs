use std::sync::Arc;

use crate::vec3::{Color, Point3};

use super::{Texture, solid_color::SolidColor};

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