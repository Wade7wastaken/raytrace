use crate::{
    material::Material,
    primitives::{Interval, Point3, Ray, Vec3},
    tern,
};

pub struct Quad {
    q: Point3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    mat: Material,
    normal: Vec3,
    d: f64,
}

const UNIT_INTERVAL: Interval = Interval::new(0.0, 1.0);

impl Quad {
    #[must_use]
    #[inline]
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

    #[must_use]
    #[inline]
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

        if !(UNIT_INTERVAL.contains(alpha) && UNIT_INTERVAL.contains(beta)) {
            return None;
        }

        Some(HitRecord::new(intersection, &self.mat, t, r, self.normal))
    }
}

#[must_use]
#[inline]
pub fn quad(q: Point3, u: Vec3, v: Vec3, mat: Material) -> Quad {
    Quad::new(q, u, v, mat)
}

#[derive(Clone)]
pub struct HitRecord<'a> {
    pub p: Point3,
    pub normal: Vec3,
    pub mat: &'a Material,
    pub t: f64,
}

impl<'a> HitRecord<'a> {
    /// `outward_normal` is assumed to have unit length
    #[must_use]
    #[inline]
    pub fn new(p: Point3, mat: &'a Material, t: f64, r: &Ray, outward_normal: Vec3) -> Self {
        let front_face = r.dir.dot(outward_normal) < 0.0;
        let normal = tern!(front_face, outward_normal, -outward_normal);

        Self { p, normal, mat, t }
    }
}
