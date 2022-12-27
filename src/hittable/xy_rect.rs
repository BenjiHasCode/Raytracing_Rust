use std::sync::Arc;

use crate::{vec3::{Point3, Vec3}, ray::Ray, material::Material, aabb::AABB, hit_record::HitRecord};

use super::Hittable;

pub struct XYRect {
    pub material: Arc<dyn Material>,
    pub x0: f64,
    pub x1: f64,
    pub y0: f64,
    pub y1: f64,
    pub k: f64
}

impl XYRect {
    pub fn new(x0: f64, x1: f64, y0: f64, y1: f64, k: f64, material: &Arc<dyn Material>) -> Self {
        Self { material: Arc::clone(material), x0, x1, y0, y1, k }
    }
}

impl Hittable for XYRect {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - r.origin().z) / r.direction().z;
        
        // TODO can it be simplified?
        if t < t_min || t > t_max { return None }

        let x = r.origin().x + t*r.direction().x;
        let y = r.origin().y + t*r.direction().y;

        // TODO can it be simplified
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 { return None }

        let u = (x-self.x0)/(self.x1-self.x0);
        let v = (y-self.y0)/(self.y1-self.y0);
        let outward_normal = Vec3::new(0.0, 0.0, 1.0);
        let (front_face, normal) = HitRecord::get_face_normal(r, &outward_normal);
        let p = r.at(t);

        Some(HitRecord {t, p, u, v, material: Arc::clone(&self.material), normal, front_face})
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        // The bounding box must have non-zero width in each dimension, so pad the Z
        // dimension a small amount.
        Some(AABB::new(
                Point3::new(self.x0, self.y0, self.k-0.0001),
                Point3::new(self.x1, self.y1, self.k+0.0001)
            ))
    }
}