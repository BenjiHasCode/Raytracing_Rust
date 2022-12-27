use std::{sync::Arc, f64::{INFINITY, NEG_INFINITY}};

use crate::{aabb::AABB, util::degrees_to_radians, vec3::{Point3, Vec3}, ray::Ray, hit_record::HitRecord};

use super::Hittable;

pub struct RotateY {
    ptr: Arc<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    bbox: AABB
}

impl RotateY {
    pub fn new(p: &Arc<dyn Hittable>, angle: f64) -> Self {
        let ptr = Arc::clone(p);
        let radians = degrees_to_radians(angle);
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let bbox = ptr.bounding_box(0.0, 1.0).unwrap();

        let mut min = Point3::new(INFINITY, INFINITY, INFINITY);
        let mut max = Point3::new(NEG_INFINITY, NEG_INFINITY, NEG_INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64*bbox.maximum.x + (1-i) as f64*bbox.minimum.x;
                    let y = j as f64*bbox.maximum.y + (1-j) as f64*bbox.minimum.y;
                    let z = k as f64*bbox.maximum.z + (1-k) as f64*bbox.minimum.z;

                    let newx = cos_theta*x + sin_theta*z;
                    let newz = -sin_theta*x + cos_theta*z;

                    let tester = Vec3::new(newx, y, newz);

                    for c in 0..3 {
                        // TODO refactor.... This is just sad to look at...
                        // This just looks criminal....
                        min.set_idx(c, min.idx(c).min(tester.idx(c)));
                        max.set_idx(c, max.idx(c).max(tester.idx(c)));
                    }
                }
            }
        }

        let bbox = AABB::new(min, max);

        Self { ptr, sin_theta, cos_theta, bbox }
    }
}

impl Hittable for RotateY {
    fn hit(&self, r: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut origin = r.origin();
        let mut direction = r.direction();
        
        origin.x = self.cos_theta*r.origin().x - self.sin_theta*r.origin().z;
        origin.z = self.sin_theta*r.origin().x + self.cos_theta*r.origin().z;

        direction.x = self.cos_theta*r.direction().x - self.sin_theta*r.direction().z;
        direction.z = self.sin_theta*r.direction().x + self.cos_theta*r.direction().z;

        let rotated_r = Ray::new(origin, direction, r.time());

        if let Some(rec) = self.ptr.hit(&rotated_r, t_min, t_max) {
            let mut p = rec.p;
            let mut normal = rec.normal;

            p.x = self.cos_theta*rec.p.x + self.sin_theta*rec.p.z;
            p.z = -self.sin_theta*rec.p.x + self.cos_theta*rec.p.z;

            normal.x = self.cos_theta*rec.normal.x + self.sin_theta*rec.normal.z;
            normal.z = -self.sin_theta*rec.normal.x + self.cos_theta*rec.normal.z;

            let (front_face, normal) = HitRecord::get_face_normal(&rotated_r, &normal);

            Some(HitRecord { p, normal, material: rec.material, t: rec.t, u: rec.u, v: rec.v, front_face })
        } else {
            None
        }
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        Some(AABB::new(self.bbox.minimum, self.bbox.maximum))
    }
}