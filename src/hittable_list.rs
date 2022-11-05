use crate::hittable::Hittable;
use crate::hittable::HitRecord;
use crate::ray::Ray;

pub type HittableList = Vec<Box<dyn Hittable>>;

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        // initialize temp rec, hit anythin and closest
        let mut rec = None;
        let mut closest_so_far = t_max;

        // iterate over list
        for object in self.iter() {
            if let Some(temp_rec) = object.hit(r, t_min, closest_so_far) {
                closest_so_far = temp_rec.t;
                rec = Some(temp_rec);
            }
        }

        rec
    }
}