use crate::{vec3::{Color, Vec3}, ray::Ray, hittable::HitRecord};

pub struct Metal {
    albedo: Color,
    fuzz: f64
}

impl Metal {
    pub fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let reflected = Vec3::reflect(&r_in.direction().unit_vector(), &rec.normal);
        *scattered = Ray::new(rec.p, reflected + self.fuzz*Vec3::random_in_unit_sphere());
        *attenuation = self.albedo;
        
        Vec3::dot(&scattered.direction(), &rec.normal) > 0.0
    }
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Metal {
        Metal { albedo, fuzz }
    }
}