use std::{
    fmt::{self, Display},
    sync::Arc,
};

use crate::primitives::{Aabb, Interval, Ray, Vec3};

use super::{HitRecord, Hittable};

pub struct Moving {
    object: Arc<dyn Hittable>,
    offset: Vec3,
    bbox: Aabb,
}

impl Moving {
    pub fn new(object: Arc<dyn Hittable>, offset: Vec3) -> Self {
        let bbox = object.bounding_box() + offset;
        Self {
            object,
            offset,
            bbox,
        }
    }
}

impl Hittable for Moving {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let offset = self.offset * r.time;
        // move the ray backwards
        let offset_ray = Ray::new(r.orig - offset, r.dir, r.time);

        // check for a hit and move the hit position forward if there was a hit
        let mut rec = self.object.hit(&offset_ray, ray_t)?;
        rec.p += offset;

        Some(rec)
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}

impl Display for Moving {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "moving({}, {})", self.object, self.offset)
    }
}

pub fn moving(object: Arc<dyn Hittable>, offset: Vec3) -> Arc<Moving> {
    Arc::new(Moving::new(object, offset))
}
