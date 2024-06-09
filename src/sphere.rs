use crate::{
    hittable::{HitRecord, Hittable},
    interval::Interval,
    material::Material,
    ray::Ray,
    vec3::Point3,
};
use std::rc::Rc;

pub struct Sphere {
    center: Point3,
    radius: f64,
    mat: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, mat: Rc<dyn Material>) -> Self {
        Self {
            center,
            radius,
            mat,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let oc = self.center - r.orig;
        let a = r.dir.length_squared();
        let h = r.dir.dot(oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let discriminant_sqrt = discriminant.sqrt();

        let root = (h - discriminant_sqrt) / a;

        if !ray_t.surrounds(root) {
            let other_root = (h + discriminant_sqrt) / a;

            if !ray_t.surrounds(other_root) {
                return None;
            }
        }

        let rec_p = r.at(root);

        Some(HitRecord::new(
            rec_p,
            self.mat.to_owned(),
            root,
            r,
            (rec_p - self.center) / self.radius,
        ))
    }
}

// helper initializer to make code look pretty
pub fn sphere(center: Point3, radius: f64, mat: Rc<dyn Material>) -> Sphere {
    Sphere::new(center, radius, mat)
}
