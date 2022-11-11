use std::f64::consts::PI;
use rand::Rng;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

// returns random f64 [min, max)
pub fn random_double(min: f64, max: f64) -> f64 {
    rand::thread_rng().gen_range(min..max)
}

// returns a random u32 in [min, max]
pub fn random_int(min: u32, max: u32) -> u32{
    rand::thread_rng().gen_range(min..max+1)
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    x
}