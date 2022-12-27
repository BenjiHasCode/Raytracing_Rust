use std::sync::Arc;

use crate::{texture::{Texture, solid_color::SolidColor}, vec3::{Color, Vec3}, ray::Ray, hit_record::HitRecord};

use super::Material;

pub struct Isotropic {
    albedo: Arc<dyn Texture>
}

impl Isotropic {
    pub fn new(c: Color) -> Self {
        Self { albedo: Arc::new(SolidColor::new(c.x, c.y, c.z)) }
    }

    pub fn new_texture(a: &Arc<dyn Texture>) -> Self {
        Self { albedo: Arc::clone(a) }
    }
}

impl Material for Isotropic {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let scattered = Ray::new(rec.p, Vec3::random_in_unit_sphere(), r_in.time());
        let attenuation = self.albedo.value(rec.u, rec.v, &rec.p);

        Some((attenuation, scattered))
    }
}