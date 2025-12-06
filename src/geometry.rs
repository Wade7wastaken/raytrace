use std::hint::black_box;

use crate::{
    material::Material,
    primitives::{Interval, Point3, Ray, Vec3},
    tern,
};

pub struct Quad {
    q: Point3,
    u: Vec3,
    v: Vec3,
    // precomputed
    normal: Vec3,
    d: f64,
    // 2x2 system for solving alpha/beta:
    uu: f64,
    uv: f64,
    vv: f64,
    inv_det: f64,
    mat: Material,
}

impl Quad {
    #[must_use]
    #[inline]
    pub fn new(q: Point3, u: Vec3, v: Vec3, mat: Material) -> Self {
        let n = u.cross(v);
        let normal = n.unit_vector();
        let d = normal.dot(q);

        // precompute 2x2 symmetric matrix M = [[u·u, u·v],
        //                                     [u·v, v·v]]
        let uu = u.dot(u);
        let uv = u.dot(v);
        let vv = v.dot(v);

        // determinant = uu*vv - uv*uv
        let det = uu * vv - uv * uv;
        // handle degeneracy (very small det). If degenerate, inv_det = 0 (no valid hits).
        let inv_det = 1.0 / det;

        Quad {
            q,
            u,
            v,
            normal,
            d,
            uu,
            uv,
            vv,
            inv_det,
            mat,
        }
    }

    #[must_use]
    #[inline]
    pub fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord<'_>> {
        // denom = n · dir
        let denom = self.normal.dot(r.dir);

        if denom > 0.0 {
            return None;
        }

        // parallel check
        if denom.abs() < 1e-8 {
            return None;
        }

        let t = (self.d - self.normal.dot(r.orig)) / denom;
        if !ray_t.contains(t) {
            return None;
        }

        // compute planar hit vector p' = (r.orig + t*r.dir) - q
        // compute o_minus_q first to avoid extra allocs
        let o_minus_q = r.orig - self.q;
        // p' = o_minus_q + t * r.dir
        let planar = o_minus_q + r.dir * t;

        // Solve M * [alpha; beta] = [u·planar; v·planar]
        // a = ( vv * up - uv * vp ) / det
        // b = ( uu * vp - uv * up ) / det
        // we use inv_det precomputed (0 if degenerate)

        let up = self.u.dot(planar);
        let vp = self.v.dot(planar);

        let alpha = (self.vv * up - self.uv * vp) * self.inv_det;
        let beta = (self.uu * vp - self.uv * up) * self.inv_det;

        // check within unit interval [0,1]
        if !(alpha >= 0.0 && alpha <= 1.0 && beta >= 0.0 && beta <= 1.0) {
            return None;
        }

        // compute world-space intersection point (we need it in the HitRecord)
        let intersection = r.at(t);

        Some(HitRecord::new(intersection, &self.mat, t, self.normal))
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
    pub fn new(p: Point3, mat: &'a Material, t: f64, outward_normal: Vec3) -> Self {
        // let front_face = r.dir.dot(outward_normal) < 0.0;
        let front_face = black_box(true);
        let normal = tern!(front_face, outward_normal, -outward_normal);

        Self { p, normal, mat, t }
    }
}
