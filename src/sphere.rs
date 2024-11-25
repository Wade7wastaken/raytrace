use std::sync::Arc;
use std::{f64::consts::PI, fmt};

use crate::{
    hittable::{HitRecord, Hittable},
    material::Material,
    primitives::{vec3, Aabb, Interval, Point3, Ray, Vec3},
};

#[derive(Clone)]
pub struct Sphere {
    center: Point3,
    move_dir: Vec3,
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
            move_dir: Vec3::default(),
            radius,
            mat,
            bbox,
        }
    }

    pub fn new_moving(center: Point3, move_dir: Vec3, radius: f64, mat: Arc<dyn Material>) -> Self {
        let r_vec = vec3(radius, radius, radius);
        let box1 = Aabb::from_points(center - r_vec, center + r_vec);
        let box2 = Aabb::from_points(center + move_dir - r_vec, center + move_dir + r_vec);
        let bbox = Aabb::from_boxes(&box1, &box2);
        Self {
            center,
            move_dir,
            radius,
            mat,
            bbox,
        }
    }

    fn center(&self, time: f64) -> Point3 {
        self.center + self.move_dir * time
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

        let outward_normal = (rec_p - self.center) / self.radius;

        let (u, v) = Sphere::get_sphere_uv(outward_normal);

        Some(HitRecord::new(
            rec_p,
            self.mat.to_owned(),
            root,
            u,
            v,
            r,
            (rec_p - self.center) / self.radius,
        ))
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}

impl fmt::Display for Sphere {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "circle({}, {}, {}, {})",
            self.center, self.radius, self.mat, self.bbox
        )
    }
}

// helper initializer to make code look pretty
pub fn sphere(center: Point3, radius: f64, mat: Arc<dyn Material>) -> Sphere {
    Sphere::new(center, radius, mat)
}
