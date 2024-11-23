use rand::{seq::SliceRandom, Rng};

use crate::vec3::Point3;

const POINT_COUNT: usize = 256;

pub struct Perlin {
    rand_floats: [f64; POINT_COUNT],
    perm_x: [usize; POINT_COUNT],
    perm_y: [usize; POINT_COUNT],
    perm_z: [usize; POINT_COUNT],
}

impl Perlin {
    pub fn new() -> Self {
        let mut rand_floats = [0.0; POINT_COUNT];
        rand::thread_rng().fill(&mut rand_floats);

        Self {
            rand_floats,
            perm_x: Self::generate_perm(),
            perm_y: Self::generate_perm(),
            perm_z: Self::generate_perm(),
        }
    }

    fn generate_perm() -> [usize; POINT_COUNT] {
        let mut perm = [0; POINT_COUNT];
        for (i, p) in perm.iter_mut().enumerate() {
            *p = i;
        }
        let mut rng = rand::thread_rng();
        perm.shuffle(&mut rng);
        perm
    }

    pub fn noise(&self, p: &Point3) -> f64 {
        let i = (4.0 * p.x) as i32 & 255;
        let j = (4.0 * p.y) as i32 & 255;
        let k = (4.0 * p.z) as i32 & 255;

        self.rand_floats
            [self.perm_x[i as usize] ^ self.perm_y[j as usize] ^ self.perm_z[k as usize]]
    }
}
