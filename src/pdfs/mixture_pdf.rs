use crate::{misc::rand_f64, pdfs::Pdf, primitives::Vec3, tern};

pub struct MixturePdf<'a>(&'a dyn Pdf, &'a dyn Pdf);

impl<'a> MixturePdf<'a> {
    pub fn new(p0: &'a dyn Pdf, p1: &'a dyn Pdf) -> Self {
        Self(p0, p1)
    }
}

impl Pdf for MixturePdf<'_> {
    fn value(&self, dir: Vec3) -> f64 {
        0.5 * self.0.value(dir) + 0.5 * self.1.value(dir)
    }

    fn generate(&self) -> Vec3 {
        tern!(rand_f64() < 0.5, self.0.generate(), self.1.generate())
    }
}
