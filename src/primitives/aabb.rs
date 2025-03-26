use std::{fmt, ops::Add};

use crate::misc::tern;

use super::{Interval, Point3, Ray, Vec3, interval};

#[derive(Debug, Clone)]
pub struct Aabb {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl Aabb {
    #[must_use]
    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        Self { x, y, z }.pad_to_minimums()
    }

    #[must_use]
    pub fn from_points(a: Point3, b: Point3) -> Self {
        let x = interval(a.x.min(b.x), a.x.max(b.x));
        let y = interval(a.y.min(b.y), a.y.max(b.y));
        let z = interval(a.z.min(b.z), a.z.max(b.z));

        Self::new(x, y, z)
    }

    #[must_use]
    pub fn from_boxes(a: &Self, b: &Self) -> Self {
        let x = Interval::from_intervals(&a.x, &b.x);
        let y = Interval::from_intervals(&a.y, &b.y);
        let z = Interval::from_intervals(&a.z, &b.z);
        Self::new(x, y, z)
    }

    #[must_use]
    pub fn axis_interval(&self, index: u8) -> &Interval {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Incorrect index passed to axis_interval"),
        }
    }

    #[must_use]
    pub fn hit(&self, r: &Ray, ray_t: &Interval) -> bool {
        for axis in 0..3 {
            let ax = self.axis_interval(axis);
            let ad = r.dir.axis(axis);

            let r_orig = r.orig.axis(axis);

            let (t0, t1) = tern(
                ad >= 0.0,
                ((ax.min - r_orig) / ad, (ax.max - r_orig) / ad),
                ((ax.max - r_orig) / ad, (ax.min - r_orig) / ad),
            );

            let (min, max) = (t0.max(ray_t.min), t1.min(ray_t.max));

            if max <= min {
                return false;
            }
        }

        true
    }

    #[must_use]
    pub fn longest_axis(&self) -> u8 {
        [&self.x, &self.y, &self.z]
            .into_iter()
            .enumerate()
            .max_by(|a, b| a.1.size().partial_cmp(&b.1.size()).unwrap())
            .unwrap()
            .0 as u8
    }

    const DELTA: f64 = 0.0001;

    fn pad_to_minimums(mut self) -> Self {
        if self.x.size() < Self::DELTA {
            self.x = self.x.expand(Self::DELTA);
        }
        if self.y.size() < Self::DELTA {
            self.y = self.y.expand(Self::DELTA);
        }
        if self.z.size() < Self::DELTA {
            self.z = self.z.expand(Self::DELTA);
        }

        self
    }
}

impl Add<Vec3> for &Aabb {
    type Output = Aabb;
    fn add(self, rhs: Vec3) -> Self::Output {
        Aabb::new(
            self.x.clone() + rhs.x,
            self.y.clone() + rhs.y,
            self.z.clone() + rhs.z,
        )
    }
}

impl Default for Aabb {
    fn default() -> Self {
        Self::new(
            Interval::default(),
            Interval::default(),
            Interval::default(),
        )
        .pad_to_minimums()
    }
}

impl fmt::Display for Aabb {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "aabb({}, {}, {})", self.x, self.y, self.z)
    }
}
