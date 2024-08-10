use std::sync::Arc;

use crate::{
    hittable::{HitRecord, Hittable},
    interval::Interval,
    material::Material,
    ray::Ray,
    vec3::{vec3, Point3, Vec3},
};

pub struct Sphere {
    center: Point3,
    move_dir: Vec3,
    radius: f64,
    mat: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, mat: Arc<dyn Material>) -> Self {
        Self {
            center,
            move_dir: vec3(0.0, 0.0, 0.0),
            radius,
            mat,
        }
    }

    pub fn new_moving(center: Point3, move_dir: Vec3, radius: f64, mat: Arc<dyn Material>) -> Self {
        Self {
            center,
            move_dir,
            radius,
            mat,
        }
    }

    fn center(&self, time: f64) -> Point3 {
        self.center + self.move_dir * time
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let center = self.center(r.time);
        let oc = center - r.orig;
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
pub fn sphere(center: Point3, radius: f64, mat: Arc<dyn Material>) -> Sphere {
    Sphere::new(center, radius, mat)
}
