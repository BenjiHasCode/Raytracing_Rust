use crate::{vec3::Point3, ray::Ray};

pub struct AABB {
    pub minimum: Point3,
    pub maximum: Point3
}

impl AABB {
    pub fn new(minimum: Point3, maximum: Point3) -> Self {
        AABB { minimum, maximum }   // use pointers(references here?!?!?!)
    }

    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> bool {
        for a in 0..3 {
      //  (0..3).for_each(|a| {
            let inv_d = 1.0 / r.direction().idx(a);  // TEMP SOLUTION FIND WAY TO ACCESS IDX DIRECTLY INSTEAD OF MATCHING
            let mut t0 = (self.minimum.idx(a) - r.origin().idx(a)) * inv_d;
            let mut t1 = (self.maximum.idx(a) - r.origin().idx(a)) * inv_d;

            if inv_d < 0.0 {
                let temp = t0;
                t0 = t1;
                t1 = temp;  // any cool functions for swapping values?
            }

            // Is fine to shadow fn parameters?
            let t_min = if t0 > t_min { t0 } else { t_min };
            let t_max = if t1 < t_max { t1 } else { t_max };

            if t_max <= t_min {
                return false;
            }
        }

        true
    }

    pub fn surrounding_box(box0: AABB, box1: AABB) -> Self {
        let small = Point3::new(
            f64::min(box0.minimum.x, box1.minimum.x),
            f64::min(box0.minimum.y, box1.minimum.y),
            f64::min(box0.minimum.z, box1.minimum.z)
        );

        let big = Point3::new(
            f64::max(box0.maximum.x, box1.maximum.x),
            f64::max(box0.maximum.y, box1.maximum.y),
            f64::max(box0.maximum.z, box1.maximum.z)
        );

        AABB::new(small, big)   
    }
}
