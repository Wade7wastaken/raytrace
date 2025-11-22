use super::{Point3, Vec3};

#[derive(Debug, Clone, PartialEq)]
pub struct Ray {
    pub orig: Point3,
    pub dir: Vec3,
}

impl Ray {
    #[must_use]
    pub fn new(orig: Point3, dir: Vec3) -> Self {
        Self { orig, dir }
    }

    #[must_use]
    pub fn at(&self, t: f64) -> Point3 {
        self.orig + self.dir * t
    }
}

#[must_use]
pub fn ray(orig: Point3, dir: Vec3) -> Ray {
    Ray::new(orig, dir)
}
