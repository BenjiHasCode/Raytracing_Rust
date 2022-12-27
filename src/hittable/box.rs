use std::sync::Arc;

use crate::{vec3::Point3, material::Material, aabb::AABB, hit_record::HitRecord};

use super::{hittable_list::HittableList, xy_rect::XYRect, xz_rect::XZRect, yz_rect::YZRect, Hittable};

pub struct Box {
    box_min: Point3,
    box_max: Point3,
    sides: HittableList
}

impl Box {
    pub fn new(p0: &Point3, p1: &Point3, material: &Arc<dyn Material>) -> Self {
        let mut sides = HittableList::new();

        sides.push(Arc::new(XYRect::new(p0.x, p1.x, p0.y, p1.y, p1.z, material)));
        sides.push(Arc::new(XYRect::new(p0.x, p1.x, p0.y, p1.y, p0.z, material)));

        sides.push(Arc::new(XZRect::new(p0.x, p1.x, p0.z, p1.z, p1.y, material)));
        sides.push(Arc::new(XZRect::new(p0.x, p1.x, p0.z, p1.z, p0.y, material)));

        sides.push(Arc::new(YZRect::new(p0.y, p1.y, p0.z, p1.z, p1.x, material)));
        sides.push(Arc::new(YZRect::new(p0.y, p1.y, p0.z, p1.z, p0.x, material)));

        Box { box_min: *p0, box_max: *p1, sides }
    }
}

impl Hittable for Box {
    fn hit(&self, r: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.sides.hit(r, t_min, t_max)
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        Some(AABB::new(self.box_min, self.box_max))
    }
}