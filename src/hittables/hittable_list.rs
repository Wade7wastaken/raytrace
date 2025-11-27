use crate::primitives::{Interval, Ray, interval};

use super::{HitRecord, Hittable};

#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn add<H: Hittable + 'static>(&mut self, object: H) {
        self.objects.push(Box::new(object));
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord<'_>> {
        self.objects.iter().fold(None, |rec, object| {
            let max = rec.as_ref().map_or(ray_t.max, |r| r.t);
            object.hit(r, &interval(ray_t.min, max)).or(rec)
        })
    }
}
