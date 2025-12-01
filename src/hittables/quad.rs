use crate::{
    material::Material,
    primitives::{Interval, Point3, Ray, Vec3},
};

use super::HitRecord;

pub struct Quad {
    q: Point3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    mat: Material,
    normal: Vec3,
    d: f64,
}

impl Quad {
    pub fn new(q: Point3, u: Vec3, v: Vec3, mat: Material) -> Self {
        let n = u.cross(v);
        let normal = n.unit_vector();
        let d = normal.dot(q);
        let w = n / n.dot(n);

        Quad {
            q,
            u,
            v,
            w,
            mat,
            normal,
            d,
        }
    }

    pub fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord<'_>> {
        let denom = self.normal.dot(r.dir);

        if denom.abs() < 1e-8 {
            return None;
        }

        let t = (self.d - self.normal.dot(r.orig)) / denom;
        if !ray_t.contains(t) {
            return None;
        }

        let intersection = r.at(t);

        // check if the intersection is within the quad
        let planar_hitpoint = intersection - self.q;
        let alpha = self.w.dot(planar_hitpoint.cross(self.v));
        let beta = self.w.dot(self.u.cross(planar_hitpoint));

        let unit_interval = Interval::new(0.0, 1.0);

        if !(unit_interval.contains(alpha) && unit_interval.contains(beta)) {
            return None;
        }

        Some(HitRecord::new(intersection, &self.mat, t, r, self.normal))
    }
}

pub fn quad(q: Point3, u: Vec3, v: Vec3, mat: Material) -> Quad {
    Quad::new(q, u, v, mat)
}
