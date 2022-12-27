use std::{sync::Arc, cmp::Ordering};

use crate::{aabb::AABB, util::random_int, vec3::Vec3, hit_record::HitRecord, ray::Ray};

use super::{hittable_list::HittableList, Hittable};


pub struct BvhNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    bounding_box: AABB
}

impl BvhNode {
    pub fn new(list: &mut HittableList, start: usize, end: usize, time0: f64, time1: f64) -> Self {
        let objects = list; // Create a modifiable array of the source scene objects
        // is above necessary?

        let axis = random_int(0, 2);
        let comparator = match axis {   // fn to compare?????
            0 => Self::box_x_compare,
            1 => Self::box_y_compare,
            _ => Self::box_z_compare
        };

        let object_span = end - start;
        let left;
        let right;

        if object_span == 1 {
            left = Arc::clone(&objects[start]); // ey bro wtf
            right = Arc::clone(&objects[start]);

        } else if object_span == 2 {
            if comparator(&*objects[start], &*objects[start+1]).is_le() {   // is less or is greater?
                left = Arc::clone(&objects[start]);
                right = Arc::clone(&objects[start+1]);
            } else {
                left = Arc::clone(&objects[start+1]);
                right = Arc::clone(&objects[start]);
            }
        } else {
            //wtf
            //std::sort(objects.begin() + start, objects.begin() + end, comparator);
            let slice = &mut objects[start..end];
            slice.sort_by(|a, b| comparator(&**a, &**b)); // yo wtf

            let mid = start + object_span / 2;
            left = Arc::new(Self::new(objects, start, mid, time0, time1));
            right = Arc::new(Self::new(objects, mid, end, time0, time1));
        }
    
        let box_left = left.bounding_box(time0, time1);
        let box_right = right.bounding_box(time0, time1);

        if box_left.is_none() || box_right.is_none() {
            println!("No bounding box in bvh_node constructor");    // todo error message? panic?
        }

        let bounding_box = AABB::surrounding_box(box_left.unwrap(), box_right.unwrap());
        BvhNode { left, right, bounding_box }
    }

    // why not just call this function directly and pass the parameters along????
    fn box_compare(a: &dyn Hittable, b: &dyn Hittable, axis: u8) -> Ordering {
        let box_a = a.bounding_box(0.0, 0.0);
        let box_b = b.bounding_box(0.0, 0.0);

        if box_a.is_none() || box_b.is_none() {
            println!("No bounding box in bvh_node constructor");
        }

        let axis = axis as usize;
        box_a.unwrap().minimum.idx(axis).partial_cmp(&box_b.unwrap().minimum.idx(axis)).unwrap()
        // box_a.unwrap().minimum.idx(axis) < box_b.unwrap().minimum.idx(axis)
    }

    // redundant no?
    fn box_x_compare(a: &dyn Hittable, b: &dyn Hittable) -> Ordering {
        Self::box_compare(a, b, 0)
    }
    // redundant no?
    fn box_y_compare(a: &dyn Hittable, b: &dyn Hittable) -> Ordering {
        Self::box_compare(a, b, 1)
    }
    // redundant no?
    fn box_z_compare(a: &dyn Hittable, b: &dyn Hittable) -> Ordering {
        Self::box_compare(a, b, 2)
    }
}

impl Hittable for BvhNode { // TODO THIS IS PROBABLY VERY ERROR PRONE!!!!!
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if !self.bounding_box.hit(r, t_min, t_max) { return None }
        
        let hit_left = self.left.hit(r, t_min, t_max);
        let hit_right = self.right.hit(r, t_min, t_max);


        // Feel like this is very error prone, check here if it does not work.
        if hit_left.is_some() {
            return hit_left;
        } else if hit_right.is_some() {
            return hit_right;
        } else {
            return None;
        }

        /*
        if (!box.hit(r, t_min, t_max))
        return false;

        bool hit_left = left->hit(r, t_min, t_max, rec);
        bool hit_right = right->hit(r, t_min, hit_left ? rec.t : t_max, rec);

        return hit_left || hit_right;
        */
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        //output_box = box;
        // return true;

//        Some(self.bounding_box)
        // TEMP SOLUTION FIND BETTER WAY
        // PERHAPS A BETTER SOLUTION
        // WOULD BE RETURNING OPTION WITH A REFERENCE?
        Some(AABB::new(
            Vec3::new(
                self.bounding_box.minimum.x,
                self.bounding_box.minimum.y,
                self.bounding_box.minimum.z
            ),
            Vec3::new(
                self.bounding_box.maximum.x,
                self.bounding_box.maximum.y,
                self.bounding_box.maximum.z
            )
        ))
    }
}