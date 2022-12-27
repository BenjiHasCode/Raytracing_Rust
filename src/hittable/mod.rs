use crate::{hit_record::HitRecord, aabb::AABB, ray::Ray};

pub mod r#box;
pub mod bvh;
pub mod constant_medium;
pub mod hittable_list;
pub mod rotate_y;
pub mod sphere;
pub mod translate;
pub mod xy_rect;
pub mod xz_rect;
pub mod yz_rect;
pub mod moving_sphere;

pub trait Hittable : Send + Sync {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB>;
}