use std::sync::Arc;

use crate::{vec3::{Color, Vec3}, hittable::HitRecord, ray::Ray, texture::{Texture, SolidColor}};

use super::Material;

pub struct Lambertian {
    albedo: Arc<dyn Texture>
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();
        
        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        
        let scattered = Ray::new(rec.p, scatter_direction, r_in.time());
        let attunation = self.albedo.value(rec.u, rec.v, &rec.p);
        
        Some((attunation, scattered))
    }
}

impl Lambertian {
    pub fn new_color(a: Color) -> Lambertian {
        Self { albedo: Arc::new(SolidColor::new(a.x, a.y, a.z)) }
    }
    pub fn new_texture(albedo: &Arc<dyn Texture>) -> Lambertian {
        Lambertian { albedo: Arc::clone(albedo) }
    }
}