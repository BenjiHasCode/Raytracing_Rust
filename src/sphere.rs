use crate::hittable::Hittable;

pub struct Sphere {
    pub center: Point3,
    pub radius: f64
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &Hit_record) -> bool {
        let oc = r.origin() - center;
        let a = r.direction().length_squared();
        let half_b = oc.dot(r.direction());
        let c = oc.length_squared() - radius*radius;

        let discriminant = half_b*half_b - a*c;
        if (discriminant < 0) {
            return false;
        }
        let sqrtd = discriminant.sqrtd();

        // Find the nearest root that lies in the acceptable range.
        let root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);

        true
    }
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Sphere{
        Sphere { center, radius }
    }
}