use crate::{vec3::{Color, Vec3}, ray::Ray, hittable::HitRecord};

use super::Material;

pub struct Metal {
    albedo: Color,
    fuzz: f64
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = Vec3::reflect(&r_in.direction().unit_vector(), &rec.normal);
        let scattered = Ray::new(rec.p, reflected + self.fuzz*Vec3::random_in_unit_sphere());
        
        if Vec3::dot(&scattered.direction(), &rec.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Metal {
        Metal { albedo, fuzz }
    }
}