#[derive(Debug, PartialEq, Clone, Copy)] //what is partialeq?
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 {x, y, z}
    }

    // length
    pub fn length(&self) -> f64 {
        f64::sqrt(self.length_squared())
    }
    // length_squared
    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    // dot
    pub fn dot(&self, other: &Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    // cross
    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x
        }
    }
    // unit_vector
    pub fn unit_vector(&self) -> Vec3 {
        *self / self.length() // Todo is this right
    }
}


impl std::ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z
        }
    }
}

impl std::ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }
}

impl std::ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, _rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x * _rhs.x,
            y: self.y * _rhs.y,
            z: self.z * _rhs.z
        }
    }
}

impl std::ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, _rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x * _rhs,
            y: self.y * _rhs,
            z: self.z * _rhs
        }
    }
}

impl std::ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, _rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self * _rhs.x,
            y: self * _rhs.y,
            z: self * _rhs.z
        }
    }
}

impl std::ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, _rhs: f64) -> Self::Output {
        self * (1.0/_rhs)
    }
}

impl std::ops::AddAssign for Vec3 {
    fn add_assign(&mut self, _rhs: Vec3) {
        self.x += _rhs.x;
        self.y += _rhs.y;
        self.z += _rhs.z;
    }
}

impl std::ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, _rhs: f64) {
        self.x *= _rhs;
        self.y *= _rhs;
        self.z *= _rhs;
    }
}

impl std::ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, _rhs: f64) {
        let i = 1.0/_rhs;
        self.x *= i;
        self.y *= i;
        self.z *= i;
    }
}



// Type aliases for vec3
pub type Point3 = Vec3;   // 3d point
pub type Color = Vec3;    // rgb color

impl Color {
    pub fn translate(&self) -> Vec3 {    // give better name
        Vec3 {
            x: 255.999 * self.x,
            y: 255.999 * self.y,
            z: 255.999 * self.z
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Vec3;
    
    #[test]
    fn add() {
        let vec1 = Vec3::new(1.0, 1.0, 1.0);
        let vec2 = Vec3::new(2.0, 2.0, 2.0);

        let vec3 = vec1 + vec2;

        assert_eq!(vec3, Vec3::new(3.0, 3.0, 3.0))
    }

    #[test]
    fn sub() {
        let vec1 = Vec3::new(1.0, 1.0, 1.0);
        let vec2 = Vec3::new(2.0, 2.0, 2.0);

        let vec3 = vec1 - vec2;

        assert_eq!(vec3, Vec3::new(-1.0, -1.0, -1.0));
    }

    #[test]
    fn neg() {
        let vec1 = Vec3::new(1.0, 1.0, 1.0);

        let vec1 = -vec1;

        assert_eq!(vec1, Vec3::new(-1.0, -1.0, -1.0));

        // I'm kinda breaking the triple a pattern here
        let vec1 = -vec1;
        assert_eq!(vec1, Vec3::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn mul_vec3() {
        let vec1 = Vec3::new(2.0, 2.0, 2.0);
        let vec2 = Vec3::new(3.0, 3.0, 3.0);

        let vec3 = vec1 * vec2;

        assert_eq!(vec3, Vec3::new(6.0, 6.0, 6.0));
    }

    #[test]
    fn mul_f64() {
        let vec1 = Vec3::new(2.0, 2.0, 2.0);
        let scale = 3.0;

        let vec2 = vec1 * scale;

        assert_eq!(vec2, Vec3::new(6.0, 6.0, 6.0));
    }

    #[test]
    fn div() {
        let vec1 = Vec3::new(3.0, 3.0, 3.0);
        let div = 3.0;

        let vec2 = vec1 / div;

        assert_eq!(vec2, Vec3::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn add_assign() {
        let mut vec1 = Vec3::new(3.0, 3.0, 3.0);
        let vec2 = Vec3::new(3.0, 3.0, 3.0);

        vec1 += vec2;

        assert_eq!(vec1, Vec3::new(6.0, 6.0, 6.0));
    }

    #[test]
    fn mul_assign() {
        let mut vec1 = Vec3::new(3.0, 3.0, 3.0);
        let scale = 3.0;

        vec1 *= scale;

        assert_eq!(vec1, Vec3::new(9.0, 9.0, 9.0));
    }

    #[test]
    fn div_assign() {
        let mut vec1 = Vec3::new(6.0, 6.0, 6.0);
        let div = 3.0;

        vec1 /= div;

        assert_eq!(vec1, Vec3::new(2.0, 2.0, 2.0));
    }
}