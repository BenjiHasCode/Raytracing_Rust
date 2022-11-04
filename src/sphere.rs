use std::{sync::Arc};

use crate::{hittable::{Hittable, HitRecord}, vec3::{Point3, Vec3}, ray::Ray, material::Material};

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub material: Arc<Material>
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = Vec3::dot(&oc, &r.direction());
        let c = oc.length_squared() - self.radius*self.radius;

        let discriminant = half_b*half_b - a*c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, &outward_normal);
        rec.material = Arc::clone(&self.material);

        true
    }
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: &Arc<Material>) -> Sphere {
        Sphere { center, radius, material: Arc::clone(material) }
    }
}