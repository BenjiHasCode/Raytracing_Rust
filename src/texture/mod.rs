pub mod noise;
pub mod image;
pub mod checker;
pub mod solid_color;

use crate::vec3::{Color, Point3};

pub trait Texture: Send + Sync {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color;
}