use std::f64::consts::PI;

use crate::{
    misc::random_cosine_direction,
    pdfs::Pdf,
    primitives::{Onb, Vec3},
};

pub struct CosinePdf {
    uvw: Onb,
}

impl CosinePdf {
    #[must_use]
    pub fn new(uvw: Onb) -> Self {
        Self { uvw }
    }
}

impl Pdf for CosinePdf {
    fn value(&self, dir: Vec3) -> f64 {
        let cos_theta = dir.unit_vector().dot(self.uvw.w());
        (cos_theta / PI).max(0.0)
    }

    fn generate(&self) -> Vec3 {
        self.uvw.transform(random_cosine_direction())
    }
}
