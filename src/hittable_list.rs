use crate::hittable::Hittable;
use crate::hittable::HitRecord;
use crate::ray::Ray;
pub struct HittableList {
    list: Vec<Box<dyn Hittable>>
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        // initialize temp rec, hit anythin and closest
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        // iterate over list
        for object in &self.list {
            if object.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
            }
        }
        
        *rec = temp_rec;
        
        // return hit bool
        hit_anything
    }
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList { list: Vec::new() }
    }
    pub fn add(&mut self, value: Box<dyn Hittable>) {
        self.list.push(value)        
    }

    #[allow(dead_code)]
    pub fn clear(&mut self) {    // THIS IS NEVER USED AS FAR AS IM AWARE
        self.list.clear();
    }
}