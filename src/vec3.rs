#[derive(Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 {x, y, z}
    }
}


// Type aliases for vec3
pub type Point3 = Vec3;   // 3d point
pub type Color = Vec3;    // rgb color