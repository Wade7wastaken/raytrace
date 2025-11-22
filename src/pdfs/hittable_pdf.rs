use crate::{
    hittables::Hittable,
    pdfs::Pdf,
    primitives::{Point3, Vec3},
};

pub struct HittablePdf<'a> {
    objects: &'a dyn Hittable,
    origin: Point3,
}

impl<'a> HittablePdf<'a> {
    pub fn new(objects: &'a dyn Hittable, origin: Point3) -> Self {
        Self { objects, origin }
    }
}

impl Pdf for HittablePdf<'_> {
    fn value(&self, dir: Vec3) -> f64 {
        self.objects.pdf_value(self.origin, dir)
    }

    fn generate(&self) -> Vec3 {
        self.objects.random(self.origin)
    }
}
