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
        let mut ranfloat = [0.0; Perlin::POINT_COUNT]; //todo don't need to initialize
        for i in 0..Perlin::POINT_COUNT {
            ranfloat[i] = random_double(0.0, 1.0);
        }

        let perm_x = Perlin::perlin_generate_perm();
        let perm_y = Perlin::perlin_generate_perm();
        let perm_z = Perlin::perlin_generate_perm();

        Perlin { ranfloat, perm_x, perm_y, perm_z }
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

    pub fn noise(&self, p: &Point3) -> f64 {
        let mut u = p.x - p.x.floor();
        let mut v = p.y - p.y.floor();
        let mut w = p.z - p.z.floor();
        u = u*u*(3.0 - 2.0*u);
        v = v*v*(3.0 - 2.0*v);
        w = w*w*(3.0 - 2.0*w);

        let i = p.x.floor() as usize;
        let j = p.y.floor() as usize;
        let k = p.z.floor() as usize;

        let mut c = [[[0.0; 2]; 2]; 2];   // man this is not easy on the eyes

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.ranfloat[
                        self.perm_x[(i+di) & 255] ^
                        self.perm_y[(j+dj) & 255] ^
                        self.perm_z[(k+dk) & 255]
                    ];
                }
            }
        }
        
        return Self::trilinear_interp(c, u, v, w);
    }

    fn trilinear_interp(c: [[[f64; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let mut accum = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let i = i as f64;
                    let j = j as f64;
                    let k = k as f64;
                    accum += (i*u + (1.0-i)*(1.0-u))*
                        (j*v + (1.0-j)*(1.0-v))*
                        (k*w + (1.0-k)*(1.0-w))*c[i as usize][j as usize][k as usize];
                }
            }
        }
        
        accum
    }
}