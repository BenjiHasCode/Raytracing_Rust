use crate::{hittable::HitRecord, vec3::{Color}, ray::Ray};

pub mod lambertian;
pub mod metal;
pub mod dielectric;

pub trait Material : Send + Sync {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
}