use std::sync::Arc;

use crate::aabb::AABB;
use crate::hit_record::HitRecord;
use crate::ray::Ray;

use super::Hittable;


pub type HittableList = Vec<Arc<dyn Hittable>>;

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        // initialize temp rec and closest_so_far
        let mut rec = None;
        let mut closest_so_far = t_max;

        // iterate over list
        self.iter().for_each(|object| {
            if let Some(temp_rec) = object.hit(r, t_min, closest_so_far) {
                closest_so_far = temp_rec.t;
                rec = Some(temp_rec);
            }
        });

        rec
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        if self.is_empty() { return None }

        let mut bounding_box = None;

        for object in self {
            let temp_box = object.bounding_box(time0, time1);
            if temp_box.is_none() { return None }

            bounding_box = match bounding_box.is_none() {
                true => temp_box,
                false => Some(AABB::surrounding_box(bounding_box.unwrap(), temp_box.unwrap()))
            };
        }
    
        bounding_box
    }
}