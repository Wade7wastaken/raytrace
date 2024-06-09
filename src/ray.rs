use crate::vec3::{Point3, Vec3};

#[derive(Debug, Clone, PartialEq)]
pub struct Ray {
    pub orig: Point3,
    pub dir: Vec3,
}

impl Ray {
    pub fn empty() -> Self {
        Self {
            orig: Point3::empty(),
            dir: Vec3::empty(),
        }
    }

    pub fn new(orig: Point3, dir: Vec3) -> Self {
        Self { orig, dir }
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.orig + self.dir * t
    }
}

// helper initializer to make code look pretty
pub fn ray(orig: Point3, dir: Vec3) -> Ray {
    Ray::new(orig, dir)
}
