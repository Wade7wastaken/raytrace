use std::{fmt::Display, sync::Arc};

use crate::{
    aabb::Aabb,
    hittable::{HitRecord, Hittable},
    interval::Interval,
    material::Material,
    ray::Ray,
    vec3::{Point3, Vec3},
};

pub struct Quad {
    q: Point3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    mat: Arc<dyn Material>,
    bbox: Aabb,
    normal: Vec3,
    d: f64,
}

impl Quad {
    pub fn new(q: Point3, u: Vec3, v: Vec3, mat: Arc<dyn Material>) -> Self {
        let bbox_diag_1 = Aabb::from_points(q, q + u + v);
        let bbox_diag_2 = Aabb::from_points(q + u, q + v);

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
            bbox: Aabb::from_boxes(&bbox_diag_1, &bbox_diag_2),
            normal,
            d,
        }
    }
}

impl Hittable for Quad {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
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

        if !unit_interval.contains(alpha) || !unit_interval.contains(beta) {
            return None;
        }

        Some(HitRecord::new(
            intersection,
            self.mat.to_owned(),
            t,
            alpha,
            beta,
            r,
            self.normal,
        ))
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}

impl Display for Quad {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "todo")
    }
}

pub fn quad(q: Point3, u: Vec3, v: Vec3, mat: Arc<dyn Material>) -> Quad {
    Quad::new(q, u, v, mat)
}
