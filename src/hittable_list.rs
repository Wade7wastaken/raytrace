use std::sync::Arc;

use crate::{
    aabb::Aabb,
    hittable::{HitRecord, Hittable},
    interval::{interval, Interval},
    ray::Ray,
};

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

    pub fn take(&mut self, object: impl Hittable + 'static) {
        self.objects.push(Arc::new(object));
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

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}
