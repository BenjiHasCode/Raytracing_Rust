use std::sync::Arc;

use crate::{texture::{Texture, solid_color::SolidColor}, vec3::{Point3, Color}, ray::Ray, hit_record::HitRecord};

use super::Material;

pub struct DiffuseLight {
    emit: Arc<dyn Texture>
}

impl Material for DiffuseLight {
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord) -> Option<(Color, Ray)> {
        None
    }

    fn emitted(&self, u: f64, v: f64, p: &Point3) -> Color {
        self.emit.value(u, v, p)
    }
}

impl DiffuseLight {
    pub fn new_color(a: Color) -> Self {    // why not just pass reference?
        Self { emit: Arc::new(SolidColor::new(a.x, a.y, a.z)) }
    }
    pub fn new_texture(albedo: &Arc<dyn Texture>) -> Self {
        Self { emit: Arc::clone(albedo) }
    }
}