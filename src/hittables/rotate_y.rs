use core::f64;
use std::sync::Arc;

use crate::primitives::{Aabb, Interval, Ray, point3, ray, vec3};

use super::{HitRecord, Hittable};

pub struct RotateY {
    object: Arc<dyn Hittable>,
    angle: f64,
    sin_theta: f64,
    cos_theta: f64,
    bbox: Aabb,
}

impl RotateY {
    pub fn new(object: Arc<dyn Hittable>, angle: f64) -> Self {
        let (sin_theta, cos_theta) = angle.to_radians().sin_cos();
        let bbox = object.bounding_box();

        let mut max = point3(f64::INFINITY, f64::INFINITY, f64::INFINITY);
        let mut min = point3(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);

        for i in (0..2).map(f64::from) {
            for j in (0..2).map(f64::from) {
                for k in (0..2).map(f64::from) {
                    let x = i * bbox.x.max + (1.0 - i) * bbox.x.min;
                    let y = j * bbox.y.max + (1.0 - i) * bbox.y.min;
                    let z = k * bbox.z.max + (1.0 - i) * bbox.z.min;

                    let new_x = cos_theta * x + sin_theta * z;
                    let new_z = -sin_theta * x + cos_theta * z;

                    let tester = vec3(new_x, y, new_z);

                    min.x = min.x.min(tester.x);
                    max.x = max.x.max(tester.x);

                    min.y = min.y.min(tester.y);
                    max.y = max.y.max(tester.y);

                    min.z = min.z.min(tester.z);
                    max.z = max.z.max(tester.z);
                }
            }
        }

        Self {
            object,
            angle,
            sin_theta,
            cos_theta,
            bbox: Aabb::from_points(min, max),
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        // transform to object space
        let orig = point3(
            self.cos_theta * r.orig.x - self.sin_theta * r.orig.z,
            r.orig.y,
            self.sin_theta * r.orig.x + self.cos_theta * r.orig.z,
        );

        let dir = vec3(
            self.cos_theta * r.dir.x - self.sin_theta * r.dir.z,
            r.dir.y,
            self.sin_theta * r.dir.x + self.cos_theta * r.dir.z,
        );

        let rotated_ray = ray(orig, dir, r.time);

        // check collision

        let mut rec = self.object.hit(&rotated_ray, ray_t)?;

        rec.p = point3(
            self.cos_theta * rec.p.x + self.sin_theta * rec.p.z,
            rec.p.y,
            -self.sin_theta * rec.p.x + self.cos_theta * rec.p.z,
        );
        rec.normal = point3(
            self.cos_theta * rec.normal.x + self.sin_theta * rec.normal.z,
            rec.normal.y,
            -self.sin_theta * rec.normal.x + self.cos_theta * rec.normal.z,
        );
        Some(rec)
    }

    fn bounding_box(&self) -> &crate::primitives::Aabb {
        &self.bbox
    }
}

pub fn rotate_y(object: Arc<dyn Hittable>, angle: f64) -> Arc<RotateY> {
    Arc::new(RotateY::new(object, angle))
}
