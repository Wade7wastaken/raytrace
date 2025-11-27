use core::f64;

use crate::primitives::{Interval, Ray, point3, ray, vec3};

use super::{HitRecord, Hittable};

pub struct RotateY<H: Hittable> {
    object: H,
    sin_theta: f64,
    cos_theta: f64,
}

impl<H: Hittable> RotateY<H> {
    pub fn new(object: H, angle: f64) -> Self {
        let (sin_theta, cos_theta) = angle.to_radians().sin_cos();

        Self {
            object,
            sin_theta,
            cos_theta,
        }
    }
}

impl<H: Hittable> Hittable for RotateY<H> {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord<'_>> {
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

        let rotated_ray = ray(orig, dir);

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
}

pub fn rotate_y<H: Hittable>(object: H, angle: f64) -> RotateY<H> {
    RotateY::new(object, angle)
}
