use std::{sync::Arc, f64::consts::PI};

use crate::{vec3::{Point3, Vec3}, material::Material, ray::Ray, aabb::AABB, hit_record::HitRecord};

use super::Hittable;

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub material: Arc<dyn Material>
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: &Arc<dyn Material>) -> Sphere {
        Sphere { center, radius, material: Arc::clone(material) }
    }

    pub fn get_sphere_uv(p: &Point3) -> (f64, f64) {
        // p: a given point on the sphere of radius one, centered at the origin
        // u: returned value [0, 1] of angle around the Y axis from X=-1
        // v: returned value [0, 1] of angle from Y=-1 to Y=+1
        //  <1 0 0> yields <0.50 0.50>      <-1 0 0> yields <0.00 0.50>
        //  <0 1 0> yields <0.50 1.00>      <0 -1 0> yields <0.50 0.00>
        //  <0 0 1> yields <0.25 0.50>      <0 0 -1> yields <0.75 0.50>

        let theta = (-p.y).acos();
        let phi = (-p.z).atan2(p.x) + PI;

        let u = phi / (2.0*PI);
        let v = theta / PI;
        (u, v)
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = Vec3::dot(&oc, &r.direction());
        let c = oc.length_squared() - self.radius*self.radius;

        let discriminant = half_b*half_b - a*c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let t = root;
        let p = r.at(t);
        let outward_normal = (p - self.center) / self.radius;
        let (front_face, normal) = HitRecord::get_face_normal(r, &outward_normal);
        let (u, v) = Sphere::get_sphere_uv(&normal);

        Some(HitRecord { t, p, u, v, material: Arc::clone(&self.material), normal, front_face })
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        Some(AABB::new(
                self.center - Vec3::new(self.radius, self.radius, self.radius),
                self.center + Vec3::new(self.radius, self.radius, self.radius)
            ))
    }
}