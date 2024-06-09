use std::rc::Rc;

use crate::{
    hittable::{HitRecord, Hittable},
    interval::{interval, Interval},
    ray::Ray,
};

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

    pub fn new(object: Rc<dyn Hittable>) -> Self {
        Self {
            objects: vec![object],
        }
    }

    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object);
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
