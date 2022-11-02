use std::ptr::null;

use crate::{hittable::HitRecord, vec3::{Color, Vec3}, ray::Ray};

use self::{metal::Metal, lambertian::Lambertian, dielectric::Dielectric};

pub mod lambertian;
pub mod metal;
pub mod dielectric;

pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric)
}

impl Material {
    pub fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        match self {
            Material::Lambertian(lambertian) => lambertian.scatter(r_in, rec, attenuation, scattered),
            Material::Metal(metal) => metal.scatter(r_in, rec, attenuation, scattered),
            Material::Dielectric(dielectric) => dielectric.scatter(r_in, rec, attenuation, scattered)
        }
    }
}

impl Default for Material {
    fn default() -> Self {

        Material::Lambertian(Lambertian::new(Vec3::default()))
    }
}