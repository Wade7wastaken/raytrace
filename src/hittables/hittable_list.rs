use crate::{
    hittables::Quad,
    primitives::{Interval, Ray, interval},
};

use super::HitRecord;

#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Quad>,
}

impl HittableList {
    pub fn add(&mut self, object: Quad) {
        self.objects.push(object);
    }

    pub fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord<'_>> {
        self.objects.iter().fold(None, |rec, object| {
            let max = rec.as_ref().map_or(ray_t.max, |r| r.t);
            object.hit(r, &interval(ray_t.min, max)).or(rec)
        })
    }
}
