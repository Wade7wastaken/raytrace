use std::{
    fmt::{self, Display},
    sync::Arc,
};

use crate::{
    materials::Material,
    primitives::{Aabb, Interval, Point3, Ray, Vec3, point3, vec3},
};

use super::{HitRecord, Hittable, HittableList};

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

    fn is_within_surface(alpha: f64, beta: f64) -> bool {
        // parallelogram
        let unit_interval = Interval::new(0.0, 1.0);
        unit_interval.contains(alpha) && unit_interval.contains(beta)

        // disk
        // (alpha * alpha + beta * beta) < 1.0

        // triangle
        // alpha > 0.0 && beta > 0.0 && alpha + beta < 1.0
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

        if !Self::is_within_surface(alpha, beta) {
            return None;
        }

        Some(HitRecord::new(
            intersection,
            self.mat.clone(),
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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "quad({}, {}, {}, {})", self.q, self.u, self.v, self.mat)
    }
}

pub fn quad(q: Point3, u: Vec3, v: Vec3, mat: Arc<dyn Material>) -> Arc<Quad> {
    Arc::new(Quad::new(q, u, v, mat))
}

pub fn cube(a: Point3, b: Point3, material: Arc<dyn Material>) -> Arc<HittableList> {
    let mut sides = HittableList::default();

    let min = point3(a.x.min(b.x), a.y.min(b.y), a.z.min(b.z));
    let max = point3(a.x.max(b.x), a.y.max(b.y), a.z.max(b.z));

    let dx = vec3(max.x - min.x, 0.0, 0.0);
    let dy = vec3(0.0, max.y - min.y, 0.0);
    let dz = vec3(0.0, 0.0, max.z - min.z);

    sides.add(quad(point3(min.x, min.y, max.z), dx, dy, material.clone())); // front
    sides.add(quad(point3(max.x, min.y, max.z), -dz, dy, material.clone())); // right
    sides.add(quad(point3(max.x, min.y, min.z), -dx, dy, material.clone())); // back
    sides.add(quad(point3(min.x, min.y, min.z), dz, dy, material.clone())); // left
    sides.add(quad(point3(min.x, max.y, max.z), dx, -dz, material.clone())); // top
    sides.add(quad(point3(min.x, min.y, min.z), dx, dz, material)); // bottom

    Arc::new(sides)
}
