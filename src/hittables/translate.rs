use std::{fmt::Display, sync::Arc};

use crate::primitives::{Aabb, Interval, Ray, Vec3};

use super::{HitRecord, Hittable};

pub struct Translate {
    object: Arc<dyn Hittable>,
    offset: Vec3,
    bbox: Aabb,
}

impl Translate {
    pub fn new(object: Arc<dyn Hittable>, offset: Vec3) -> Self {
        let bbox = object.bounding_box() + offset;
        Self {
            object,
            offset,
            bbox,
        }
    }
}

impl Hittable for Translate {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        // move the ray backwards
        let offset_ray = Ray::new(r.orig - self.offset, r.dir, r.time);

        // check for a hit and move the hit position forward if there was a hit
        self.object.hit(&offset_ray, ray_t).map(|mut rec| {
            rec.p += self.offset;
            rec
        })
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}

impl Display for Translate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Translate {} by {}", self.object, self.offset)
    }
}

pub fn translate(object: Arc<dyn Hittable>, offset: Vec3) -> Arc<Translate> {
    Arc::new(Translate::new(object, offset))
}
