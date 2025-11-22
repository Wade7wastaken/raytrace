use crate::{
    material::Material,
    primitives::{Interval, Point3, Ray, Vec3},
    tern,
};

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat: Material,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
}

impl HitRecord {
    /// `outward_normal` is assumed to have unit length
    pub fn new(
        p: Point3,
        mat: Material,
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

pub trait Hittable: Sync + Send {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord>;
}
