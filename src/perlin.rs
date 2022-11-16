use crate::{vec3::Point3, util::{random_int, random_double}};

pub struct Perlin {
    ranfloat: [f64; Perlin::POINT_COUNT],
    perm_x: [usize; Perlin::POINT_COUNT],
    perm_y: [usize; Perlin::POINT_COUNT],
    perm_z: [usize; Perlin::POINT_COUNT]
}

impl Perlin {
    const POINT_COUNT: usize = 256;

    pub fn new() -> Self {
        let mut ranfloat = [0.0; Perlin::POINT_COUNT];
        for i in 0..ranfloat.len() {
            ranfloat[i] = random_double(0.0, 1.0);
        }

        let perm_x = Perlin::perlin_generate_perm();
        let perm_y = Perlin::perlin_generate_perm();
        let perm_z = Perlin::perlin_generate_perm();

        Perlin { ranfloat, perm_x, perm_y, perm_z }
    }

    pub fn noise(&self, p: &Point3) -> f64 {
        let i = (4.0 * p.x) as usize & 255;
        let j = (4.0 * p.y) as usize & 255;
        let k = (4.0 * p.z) as usize & 255;
        
        self.ranfloat[(self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k]) as usize]
    }

    fn perlin_generate_perm() -> [usize; Perlin::POINT_COUNT] {
        let mut p = [0; Perlin::POINT_COUNT];
        for (i, p) in p.iter_mut().enumerate() {
            *p = i;
        }

        Perlin::permute(&mut p);
        p
    }

    fn permute(p: &mut [usize; Perlin::POINT_COUNT]) {
        for i in p.len()..0 {
            let target = random_int(0, i as u32) as usize;
            p.swap(i, target);
        }
    }
}