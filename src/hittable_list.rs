use crate::{
    hittable::{HitRecord, Hittable},
    interval::{interval, Interval},
    ray::Ray,
};
use std::rc::Rc;

pub struct HittableList {
    // would rather call them hittables
    pub objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    pub fn empty() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn take(&mut self, object: impl Hittable + 'static) {
        self.objects.push(Rc::new(object));
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut closest_so_far = ray_t.max;

        let mut rec = None;

        for object in &self.objects {
            if let Some(temp_rec) = object.hit(r, interval(ray_t.min, closest_so_far)) {
                closest_so_far = temp_rec.t;
                rec = Some(temp_rec);
            }
        }

        rec
    }
}
