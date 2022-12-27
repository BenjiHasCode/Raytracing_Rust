use crate::{ray::Ray, vec3::{Color, Vec3}, util::random_double, hit_record::HitRecord};

use super::Material;

pub struct Dielectric {
    ir: f64 // Index of refraction
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let refraction_ratio = if rec.front_face { 
            1.0/self.ir 
        } else { 
            self.ir 
        };

        let unit_direction = r_in.direction().unit_vector();
        let cos_theta = f64::min(Vec3::dot(&-unit_direction, &rec.normal), 1.0);
        let sin_theta = f64::sqrt(1.0 - cos_theta*cos_theta);

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let should_reflect = Self::reflectance(cos_theta, refraction_ratio) > random_double(0.0, 1.0);
        
        let direction = if cannot_refract || should_reflect {
            Vec3::reflect(&unit_direction, &rec.normal)
        } 
        else {
            Vec3::refract(&unit_direction, &rec.normal, refraction_ratio)
        };

        let scattered = Ray::new(rec.p, direction, r_in.time());
        
        Some((Color::new(1.0, 1.0, 1.0), scattered))
    }
}

impl Dielectric {
    pub fn new(ir: f64) -> Dielectric {
        Dielectric { ir } 
    }

    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        // Use Schlick's approximation for reflectance.
        let mut r0 = (1.0-ref_idx) / (1.0+ref_idx);
        r0 = r0*r0;

        r0 + (1.0-r0)*f64::powi(1.0 - cosine, 5)
    }
}