use std::sync::Arc;

use crate::{ray::Ray, vec3::{Point3, Vec3}, material::Material};

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub material: Arc<dyn Material>,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = Vec3::dot(&r.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face { *outward_normal } else { -*outward_normal };
    }

    pub fn get_face_normal(r: &Ray, outward_normal: &Vec3) -> (bool, Vec3) {
        let front_face = Vec3::dot(&r.direction(), outward_normal) < 0.0;
        let normal = if front_face { *outward_normal } else { -*outward_normal };

        (front_face, normal)
    }
}