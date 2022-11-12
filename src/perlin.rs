use crate::{vec3::Point3, util::{random_int, random_double}};

pub struct Perlin {
    ranfloat: Vec<f64>,
    perm_x: Vec<u8>,
    perm_y: Vec<u8>,
    perm_z: Vec<u8>
}

impl Perlin {
    const POINT_COUNT: usize = 256;

    pub fn new() -> Self {
        let mut ranfloat = vec![0.0, Perlin::POINT_COUNT as f64];   // wtf?!
        for i in 0..Perlin::POINT_COUNT {
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

    fn perlin_generate_perm() -> Vec<u8> {
        let mut p = vec![0; Perlin::POINT_COUNT];

        for i in 0..Perlin::POINT_COUNT {
            p[i] = i as u8;
        }
                            // Vec has length no? why is this parameter even necessary?
        Perlin::permute(&mut p);
        p
    }

    fn permute(p: &mut Vec<u8>) {
        for i in (0..p.len()-1).rev() { // TODO ERROR?
            let target = random_int(0, i as u32) as usize;
            let tmp = p[i as usize];
            p[i as usize] = p[target];
            p[target] = tmp;
        }
    }
}