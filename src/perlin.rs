use rand::seq::SliceRandom;

use crate::primitives::{vec3, Point3, Vec3};

const POINT_COUNT: usize = 256;

pub struct Perlin {
    rand_vec: [Vec3; POINT_COUNT],
    perm_x: [usize; POINT_COUNT],
    perm_y: [usize; POINT_COUNT],
    perm_z: [usize; POINT_COUNT],
}

impl Perlin {
    pub fn new() -> Self {
        Self {
            rand_vec: Self::generate_rand_vecs(),
            perm_x: Self::generate_perm(),
            perm_y: Self::generate_perm(),
            perm_z: Self::generate_perm(),
        }
    }

    fn generate_rand_vecs() -> [Vec3; POINT_COUNT] {
        let mut rand_floats = [Vec3::default(); POINT_COUNT];
        for rand_float in rand_floats.iter_mut() {
            *rand_float = Vec3::random_unit_vector();
        }
        rand_floats
    }

    fn generate_perm() -> [usize; POINT_COUNT] {
        let mut perm = [0; POINT_COUNT];
        for (i, p) in perm.iter_mut().enumerate() {
            *p = i;
        }
        perm.shuffle(&mut rand::thread_rng());
        perm
    }

    pub fn noise(&self, p: Point3) -> f64 {
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();

        let i = p.x.floor() as i32;
        let j = p.y.floor() as i32;
        let k = p.z.floor() as i32;

        let mut c = [[[Vec3::default(); 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.rand_vec[self.perm_x[((i + di as i32) & 255) as usize]
                        ^ self.perm_y[((j + dj as i32) & 255) as usize]
                        ^ self.perm_z[((k + dk as i32) & 255) as usize]]
                }
            }
        }

        Self::perlin_interp(c, u, v, w)
    }

    pub fn turb(&self, mut p: Point3, depth: u32) -> f64 {
        let mut acc = 0.0;
        let mut weight = 1.0;

        for _ in 0..depth {
            acc += weight * self.noise(p);
            weight *= 0.5;
            p *= 2.0;
        }

        acc.abs()
    }

    fn perlin_interp(c: [[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        let mut acc = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight_v = vec3(u - i as f64, v - j as f64, w - k as f64);
                    acc += (i as f64 * uu + (1.0 - i as f64) * (1.0 - uu))
                        * (j as f64 * vv + (1.0 - j as f64) * (1.0 - vv))
                        * (k as f64 * ww + (1.0 - k as f64) * (1.0 - ww))
                        * c[i][j][k].dot(weight_v)
                }
            }
        }
        acc
    }
}
