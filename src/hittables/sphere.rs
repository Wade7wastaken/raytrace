use std::f64::consts::PI;
use std::fmt::{self, Display};
use std::sync::Arc;

use crate::{
    materials::Material,
    primitives::{Aabb, Interval, Point3, Ray, vec3},
};

use super::{HitRecord, Hittable};

#[derive(Clone)]
pub struct Sphere {
    center: Point3,
    radius: f64,
    mat: Arc<dyn Material>,
    bbox: Aabb,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, mat: Arc<dyn Material>) -> Self {
        let r_vec = vec3(radius, radius, radius);
        let bbox = Aabb::from_points(center - r_vec, center + r_vec);
        Self {
            center,
            radius,
            mat,
            bbox,
        }
    }

    fn get_sphere_uv(p: Point3) -> (f64, f64) {
        let theta = (-p.y).acos();
        let phi = f64::atan2(-p.z, p.x) + PI;
        let u = phi / (2.0 * PI);
        let v = theta / PI;

        (u, v)
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let oc = self.center - r.orig;
        let a = r.dir.length_squared();
        let h = r.dir.dot(oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let discriminant_sqrt = discriminant.sqrt();

        let mut root = (h - discriminant_sqrt) / a;

        if !ray_t.surrounds(root) {
            root = (h + discriminant_sqrt) / a;

            if !ray_t.surrounds(root) {
                return None;
            }
        }

        let rec_p = r.at(root);

        let outward_normal = (rec_p - self.center) / self.radius;

        let (u, v) = Sphere::get_sphere_uv(outward_normal);

        Some(HitRecord::new(
            rec_p,
            self.mat.clone(),
            root,
            u,
            v,
            r,
            outward_normal,
        ))
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}

impl Display for Sphere {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "circle({}, {}, {})", self.center, self.radius, self.mat)
    }
}

pub fn sphere(center: Point3, radius: f64, mat: Arc<dyn Material>) -> Arc<Sphere> {
    Arc::new(Sphere::new(center, radius, mat))
}
