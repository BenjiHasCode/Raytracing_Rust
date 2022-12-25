use std::{sync::Arc, f64::{NEG_INFINITY, INFINITY}};

use crate::{hittable::{Hittable, HitRecord}, material::{Material, isotropic::Isotropic}, vec3::{Color, Vec3}, texture::Texture, aabb::AABB, util::random_double};

pub struct ConstantMedium {
    boundary: Arc<dyn Hittable>,
    phase_function: Arc<dyn Material>,
    neg_inv_density: f64
}

impl ConstantMedium {
    pub fn new(b: &Arc<dyn Hittable>, d: f64, c: Color) -> Self {
        let boundary = Arc::clone(b);
        let neg_inv_density = -1.0/d;
        let phase_function = Arc::new(Isotropic::new(c));

        Self { boundary, phase_function, neg_inv_density }
    }

    pub fn new_texture(b: &Arc<dyn Hittable>, d: f64, a: &Arc<dyn Texture>) -> Self {
        let boundary = Arc::clone(b);
        let neg_inv_density = -1.0/d;
        let phase_function = Arc::new(Isotropic::new_texture(a));

        Self { boundary, phase_function, neg_inv_density }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, r: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut rec1;
        let mut rec2;

        if let Some(rec) = self.boundary.hit(r, NEG_INFINITY, INFINITY) {
            rec1 = rec;
        } else { 
            return None
        }

        if let Some(rec) = self.boundary.hit(r, rec1.t+0.0001, INFINITY) {
            rec2 = rec;
        } else {
            return None
        }
        
        if rec1.t < t_min {
            rec1.t = t_min;
        }
        if rec2.t > t_max {
            rec2.t = t_max;
        }

        if rec1.t >= rec2.t {
            return None
        }

        if rec1.t < 0.0 {
            rec1.t = 0.0;
        }

        let ray_length = r.direction().length();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * f64::log2(random_double(0.0, 1.0));   // todo is this right?
        
        if hit_distance > distance_inside_boundary {
            return None
        }

        let t = rec1.t + hit_distance / ray_length;
        let p = r.at(t);
        let normal = Vec3::new(1.0, 0.0, 0.0);   // arbitrary
        let front_face = true;                           // also arbitrary
        let mat_ptr = &self.phase_function;
        let u = 0.0;    // arbitrary???
        let v = 0.0;    // arbitrary???

        Some(HitRecord { t, p, normal, front_face, material: Arc::clone(mat_ptr), u, v})
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        self.boundary.bounding_box(time0, time1)
    }
}