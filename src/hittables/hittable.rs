use crate::{
    material::Material,
    primitives::{Interval, Point3, Ray, Vec3},
    tern,
};

#[derive(Clone)]
pub struct HitRecord<'a> {
    pub p: Point3,
    pub normal: Vec3,
    pub mat: &'a Material,
    pub t: f64,
}

impl<'a> HitRecord<'a> {
    /// `outward_normal` is assumed to have unit length
    pub fn new(
        p: Point3,
        mat: &'a Material,
        t: f64,
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
        }
    }
}

pub trait Hittable: Sync + Send {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord<'_>>;
}
