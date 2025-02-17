use std::{fmt::Display, sync::Arc};

use crate::{
    materials::Material,
    primitives::{point3, Aabb, Interval, Point3, Ray, Vec3},
};

use super::{HitRecord, Hittable};

#[derive(Clone)]
pub struct Triangle {
    a: Point3,
    b: Point3,
    c: Point3,
    edge1: Vec3,
    edge2: Vec3,
    pub outward_normal: Vec3,
    mat: Arc<dyn Material>,
    bbox: Aabb,
}

impl Triangle {
    pub fn new(a: Point3, b: Point3, c: Point3, mat: Arc<dyn Material>) -> Self {
        let edge1 = b - a;
        let edge2 = c - a;
        let outward_normal = edge1.cross(edge2).unit_vector();

        let min = point3(
            a.x.min(b.x).min(c.x),
            a.y.min(b.y).min(c.y),
            a.z.min(b.z).min(c.z),
        );
        let max = point3(
            a.x.max(b.x).max(c.x),
            a.y.max(b.y).max(c.y),
            a.z.max(b.z).max(c.z),
        );
        let bbox = Aabb::from_points(min, max);

        Self {
            a,
            b,
            c,
            edge1,
            edge2,
            outward_normal,
            mat,
            bbox,
        }
    }
}

impl Hittable for Triangle {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let ray_cross_e2 = r.dir.cross(self.edge2);
        let denom = self.edge1.dot(ray_cross_e2);

        if denom.abs() < 1e-8 {
            return None;
        }

        let s = r.orig - self.a;
        let u = s.dot(ray_cross_e2) / denom;
        if !(0.0..=1.0).contains(&u) {
            return None;
        }

        let s_cross_e1 = s.cross(self.edge1);
        let v = r.dir.dot(s_cross_e1) / denom;
        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let t = self.edge2.dot(s_cross_e1) / denom;

        if !(ray_t.surrounds(t)) {
            return None;
        }

        let p = r.at(t);

        Some(HitRecord::new(
            p,
            self.mat.to_owned(),
            t,
            u,
            v,
            r,
            self.outward_normal,
        ))
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}

impl Display for Triangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Triangle {} {} {}", self.a, self.b, self.c)
    }
}

pub fn triangle(a: Point3, b: Point3, c: Point3, mat: Arc<dyn Material>) -> Arc<Triangle> {
    Arc::new(Triangle::new(a, b, c, mat))
}
