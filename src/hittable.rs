
use crate::{
    interval::Interval, ray::Ray, vec3::{Point3, Vec3}
};

#[derive(Debug)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    // Sets the hit record normal vector
    // outward_normal is assumed to have unit length
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = r.dir.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }

    pub fn empty() -> Self {
        Self { p: Vec3::empty(), normal: Vec3::empty(), t: 0.0, front_face: false }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord>;
}
