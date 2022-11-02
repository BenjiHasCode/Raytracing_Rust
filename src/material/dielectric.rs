use crate::{hittable::HitRecord, ray::Ray, vec3::{Color, Vec3}};

pub struct Dielectric { // TODO
    
}

impl Dielectric {   // TODO
    pub fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();
        
        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        
        *scattered = Ray::new(rec.p, scatter_direction);
   //     *attenuation = self.albedo;
        true
    }
}

impl Dielectric {   // TODO
    pub fn new() -> Dielectric {
        Dielectric {  } 
    }
}