use std::fmt;

use super::{interval, Interval, Point3, Ray};

#[derive(Debug, Clone)]
pub struct Aabb {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl Aabb {
    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        Self { x, y, z }.pad_to_minimums()
    }

    pub fn from_points(a: Point3, b: Point3) -> Self {
        let x = if a.x <= b.x {
            interval(a.x, b.x)
        } else {
            interval(b.x, a.x)
        };
        let y = if a.y <= b.y {
            interval(a.y, b.y)
        } else {
            interval(b.y, a.y)
        };
        let z = if a.z <= b.z {
            interval(a.z, b.z)
        } else {
            interval(b.z, a.z)
        };

        Self { x, y, z }
    }

    pub fn from_boxes(a: &Self, b: &Self) -> Self {
        let x = Interval::from_intervals(&a.x, &b.x);
        let y = Interval::from_intervals(&a.y, &b.y);
        let z = Interval::from_intervals(&a.z, &b.z);
        Self { x, y, z }
    }

    pub fn axis_interval(&self, index: u8) -> &Interval {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Incorrect index passed to axis_interval"),
        }
    }

    pub fn hit(&self, r: &Ray, ray_t: &Interval) -> bool {
        for axis in 0..3 {
            let ax = self.axis_interval(axis);
            let adinv = 1.0 / r.dir.axis(axis);

            let t0 = (ax.min - r.orig.axis(axis)) * adinv;
            let t1 = (ax.max - r.orig.axis(axis)) * adinv;

            let mut min = ray_t.min;
            let mut max = ray_t.max;

            if t0 < t1 {
                if t0 > ray_t.min {
                    min = t0;
                }
                if t1 < ray_t.max {
                    max = t1;
                }
            } else {
                if t0 > ray_t.min {
                    min = t1;
                }
                if t1 < ray_t.max {
                    max = t0;
                }
            }

            if max <= min {
                return false;
            }
        }

        true
    }

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
            self.x = self.x.expand(Self::DELTA)
        }
        if self.y.size() < Self::DELTA {
            self.y = self.y.expand(Self::DELTA)
        }
        if self.z.size() < Self::DELTA {
            self.y = self.z.expand(Self::DELTA)
        }

        self
    }
}

impl Default for Aabb {
    fn default() -> Self {
        Self::new(
            Interval::default(),
            Interval::default(),
            Interval::default(),
        )
    }
}

impl fmt::Display for Aabb {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(x: {}, y: {}, z: {})", self.x, self.y, self.z)
    }
}
