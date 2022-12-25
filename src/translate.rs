use std::sync::Arc;

use crate::{vec3::Vec3, hittable::{Hittable, HitRecord}, aabb::AABB, ray::Ray};

pub struct Translate {
    ptr: Arc<dyn Hittable>,
    offset: Vec3
}

impl Translate {
    pub fn new(p: &Arc<dyn Hittable>, offset: &Vec3) -> Self {
        Self { ptr: Arc::clone(p), offset: *offset }
    }
}

impl Hittable for Translate {
    fn hit(&self, r: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let moved_r = Ray::new(r.origin() - self.offset, r.direction(), r.time());
        
        if let Some(mut rec) = self.ptr.hit(&moved_r, t_min, t_max) {
            rec.p += self.offset;
            let normal = rec.normal;
            rec.set_face_normal(&moved_r, &normal);

            Some(rec)
        } else {
            None
        }
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        if let Some(aabb) = self.ptr.bounding_box(time0, time1) {
            Some(AABB::new(
                aabb.minimum + self.offset,
                aabb.maximum + self.offset
            ))
        } else {
            None
        }
    }
}

