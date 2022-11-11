use std::{sync::Arc, f64::consts::PI};

use crate::{hittable::{Hittable, HitRecord}, vec3::{Point3, Vec3}, ray::Ray, material::Material, aabb::AABB};

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

        let theta = f64::acos(-p.y);
        let phi = f64::atan2(-p.z, p.x) + PI;

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


        let mut rec = HitRecord {
            t: root,
            p: r.at(root),
            u: 0.0, // resrtucture
            v: 0.0,
            material: Arc::clone(&self.material),
            normal: Vec3::new(0.0, 0.0, 0.0),
            front_face: false
        };

        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, &outward_normal);

        let (u, v) = Sphere::get_sphere_uv(&outward_normal);
        rec.u = u;
        rec.v = v;

        Some(rec)
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        Some(AABB::new(
                self.center - Vec3::new(self.radius, self.radius, self.radius),
                self.center + Vec3::new(self.radius, self.radius, self.radius)
            ))
    }
}




pub struct MovingSphere {
    pub center0: Point3,
    pub center1: Point3,
    pub time0: f64,
    pub time1: f64,
    pub radius: f64,
    pub material: Arc<dyn Material>
}

impl MovingSphere {
    pub fn new(
        center0: Point3,
        center1: Point3,
        time0: f64,
        time1: f64,
        radius: f64,
        material: &Arc<dyn Material>
    ) -> Self {
        MovingSphere {
            center0,
            center1,
            time0,
            time1,
            radius,
            material: Arc::clone(material)  // is this clone redundant?
        }
    }

    pub fn center(&self, time: f64) -> Point3 {
        self.center0 + ((time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0)
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin() - self.center(r.time());
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

        let mut rec = HitRecord {
            t: root,
            p: r.at(root),
            u: 0.0, // TODO
            v: 0.0, // TODO
            material: Arc::clone(&self.material),
            normal: Vec3::new(0.0, 0.0, 0.0),
            front_face: false
        };

        let outward_normal = (rec.p - self.center(r.time())) / self.radius;
        rec.set_face_normal(r, &outward_normal);

        Some(rec)
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        let box0 = AABB::new(
            self.center(time0) - Vec3::new(self.radius, self.radius, self.radius),
            self.center(time0) + Vec3::new(self.radius, self.radius, self.radius)
        );
        let box1 = AABB::new(
            self.center(time1) - Vec3::new(self.radius, self.radius, self.radius),
            self.center(time1) - Vec3::new(self.radius, self.radius, self.radius)
        );

        Some(AABB::surrounding_box(box0, box1))
    }
}