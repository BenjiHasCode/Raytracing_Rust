use crate::{ray::Ray, vec3::{Point3, Vec3}};

pub struct Hit_record {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool
}

impl Hit_record {
    fn set_face_normal(&self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = r.direction().dot(outward_normal) < 0.0;
        self.normal = if self.front_face { *outward_normal } else { -*outward_normal };
    }
}

pub trait Hittable {
    fn hit(r: &Ray, t_min: f64, t_max: f64, rec: &Hit_record) -> bool;
}