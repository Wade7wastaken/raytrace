use std::{fmt, sync::Arc};

use crate::primitives::{Aabb, Interval, Ray, interval};

use super::{HitRecord, Hittable};

#[derive(Clone, Default)]
pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
    bbox: Aabb,
}

impl HittableList {
    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.bbox = Aabb::from_boxes(&self.bbox, object.bounding_box());
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        self.objects.iter().fold(None, |rec, object| {
            let max = rec.as_ref().map_or(ray_t.max, |r| r.t);
            object.hit(r, &interval(ray_t.min, max)).or(rec)
        })
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}
