use crate::{vec3::{Point3, Vec3}, util::{random_int, random_double}};

pub struct Perlin {
    ranvec: [Vec3; Perlin::POINT_COUNT],
    perm_x: [usize; Perlin::POINT_COUNT],
    perm_y: [usize; Perlin::POINT_COUNT],
    perm_z: [usize; Perlin::POINT_COUNT]
}

impl Perlin {
    const POINT_COUNT: usize = 256;

    pub fn new() -> Self {
        let mut ranvec = [Vec3 {x: 0.0, y:0.0, z:0.0}; Perlin::POINT_COUNT];
        for i in 0..Perlin::POINT_COUNT {
            ranvec[i] = Vec3::random(-1.0, 1.0).unit_vector();
        }

        let perm_x = Perlin::perlin_generate_perm();
        let perm_y = Perlin::perlin_generate_perm();
        let perm_z = Perlin::perlin_generate_perm();

        Perlin { ranvec, perm_x, perm_y, perm_z }
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
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();
        let i = p.x.floor() as usize;
        let j = p.y.floor() as usize;
        let k = p.z.floor() as usize;

        let mut c = [[[Vec3 {x: 0.0, y: 0.0, z: 0.0}; 2]; 2]; 2];   // man this is not easy on the eyes

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.ranvec[
                        self.perm_x[(i+di) & 255] ^
                        self.perm_y[(j+dj) & 255] ^
                        self.perm_z[(k+dk) & 255]
                    ];
                }
            }
        }
        
        Self::trilinear_interp(c, u, v, w)
    }

    fn trilinear_interp(c: [[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let uu = u*u*(3.0-2.0*u);
        let vv = v*v*(3.0-2.0*v);
        let ww = w*w*(3.0-2.0*w);
        let mut accum = 0.0;

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let vec = c[i][j][k];
                    let i = i as f64;
                    let j = j as f64;
                    let k = k as f64;
                   
                    let weight_v = Vec3::new(u-i, v-j, w-k);

                    accum += (i*uu + (1.0-i)*(1.0-uu))
                               * (j*vv + (1.0-j)*(1.0-vv))
                               * (k*ww + (1.0-k)*(1.0-ww))
                               * Vec3::dot(&vec, &weight_v);
                }
            }
        }
        
        accum
    }

    pub fn turb(&self, p: &Point3, depth: usize) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = Vec3 {x: p.x, y: p.y, z: p.z};
        let mut weight = 1.0;

        for _ in 0..depth {
            accum += weight*self.noise(&temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }

        accum.abs()
    }
}