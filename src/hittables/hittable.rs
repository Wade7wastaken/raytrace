use std::{fmt, sync::Arc};

use crate::{
    materials::Material,
    primitives::{Aabb, Interval, Point3, Ray, Vec3, vec3},
    tern,
};

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat: Arc<dyn Material>,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
}

impl HitRecord {
    /// `outward_normal` is assumed to have unit length
    pub fn new(
        p: Point3,
        mat: Arc<dyn Material>,
        t: f64,
        u: f64,
        v: f64,
        r: &Ray,
        outward_normal: Vec3,
    ) -> Self {
        let front_face = r.dir.dot(outward_normal) < 0.0;
        let normal = tern!(front_face, outward_normal, -outward_normal);

        Self {
            p,
            normal,
            mat,
            t,
            u,
            v,
            front_face,
        }
    }
}

pub trait Hittable: Sync + Send + fmt::Display {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord>;

    fn bounding_box(&self) -> &Aabb;

    fn pdf_value(&self, origin: Point3, dir: Vec3) -> f64 {
        _ = (origin, dir);
        0.0
    }

    fn random(&self, origin: Point3) -> Vec3 {
        _ = origin;
        vec3(1.0, 0.0, 0.0)
    }
}
