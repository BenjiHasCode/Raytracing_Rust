use std::sync::Arc;

use crate::{vec3::{Point3, Vec3}, ray::Ray, material::Material, aabb::AABB, hit_record::HitRecord};

use super::Hittable;

pub struct YZRect {
    pub material: Arc<dyn Material>,
    pub y0: f64,
    pub y1: f64,
    pub z0: f64,
    pub z1: f64,
    pub k: f64
}

impl YZRect {
    pub fn new(y0: f64, y1: f64, z0: f64, z1: f64, k: f64, material: &Arc<dyn Material>) -> Self {
        Self { material: Arc::clone(material), y0, y1, z0, z1, k }
    }
}

impl Hittable for YZRect {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - r.origin().x) / r.direction().x;
        
        // TODO can it be simplified?
        if t < t_min || t > t_max { return None }

        let y = r.origin().y + t*r.direction().y;
        let z = r.origin().z + t*r.direction().z;

        // TODO can it be simplified
        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 { return None }

        let u = (y-self.y0)/(self.y1-self.y0);
        let v = (z-self.z0)/(self.z1-self.z0);
        let outward_normal = Vec3::new(1.0, 0.0, 0.0);
        let (front_face, normal) = HitRecord::get_face_normal(r, &outward_normal);
        let p = r.at(t);

        Some(HitRecord {t, p, u, v, material: Arc::clone(&self.material), normal, front_face})
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        // The bounding box must have non-zero width in each dimension, so pad the X
        // dimension a small amount.
        Some(AABB::new(
                Point3::new(self.k-0.0001, self.y0, self.z0),
                Point3::new(self.k+0.0001, self.y1, self.z1)
            ))
    }
}