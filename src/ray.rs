use crate::vec3::{Point3, Vec3};

#[derive(Default)]
pub struct Ray {
    orig: Point3,
    dir: Vec3,
    tm: f64
}

impl Ray {
    pub fn new(orig: Point3, dir: Vec3, tm: f64) -> Ray {
        Ray { orig, dir, tm }
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.orig + t*self.dir
    }

    pub fn origin(&self) -> Point3 {
        self.orig
    }

    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    pub fn time(&self) -> f64 {
        self.tm
    }
}