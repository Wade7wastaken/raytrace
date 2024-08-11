use std::sync::Arc;

use crate::{
    aabb::Aabb,
    interval::Interval,
    material::Material,
    ray::Ray,
    vec3::{Point3, Vec3},
};

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat: Arc<dyn Material>,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    /// outward_normal is assumed to have unit length
    pub fn new(p: Point3, mat: Arc<dyn Material>, t: f64, r: &Ray, outward_normal: Vec3) -> Self {
        let front_face = r.dir.dot(outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        Self {
            p,
            normal,
            mat,
            t,
            front_face,
        }
    }
}

pub trait Hittable: Sync + Send {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord>;

    fn bounding_box(&self) -> &Aabb;
}
