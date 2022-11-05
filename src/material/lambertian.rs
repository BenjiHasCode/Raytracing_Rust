use crate::{vec3::{Color, Vec3}, hittable::HitRecord, ray::Ray};

use super::Material;

pub struct Lambertian {
    albedo: Color
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();
        
        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        
        let scattered = Ray::new(rec.p, scatter_direction, r_in.time());
        
        Some((self.albedo, scattered))
    }
}

impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian { albedo }
    }
}