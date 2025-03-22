use super::{Point3, Vec3};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Ray {
    pub orig: Point3,
    pub dir: Vec3,
    pub time: f64,
}

impl Ray {
    #[must_use]
    pub fn new(orig: Point3, dir: Vec3, time: f64) -> Self {
        Self { orig, dir, time }
    }

    #[must_use]
    pub fn at(&self, t: f64) -> Point3 {
        self.orig + self.dir * t
    }
}

#[must_use]
pub fn ray(orig: Point3, dir: Vec3, time: f64) -> Ray {
    Ray::new(orig, dir, time)
}
