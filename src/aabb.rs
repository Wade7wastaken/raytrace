use crate::{
    interval::{interval, Interval},
    ray::Ray,
    vec3::Point3,
};

#[derive(Debug, Clone, Default)]
pub struct Aabb {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl Aabb {
    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        Self { x, y, z }
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

    pub fn axis_interval(&self, index: i32) -> &Interval {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Incorrect index passed to axis_interval"),
        }
    }

    pub fn hit(&self, r: Ray, mut ray_t: Interval) -> bool {
        for axis in 0..3 {
            let ax = self.axis_interval(axis);
            let adinv = 1.0 / r.dir.axis(axis);

            let t0 = (ax.min - r.orig.axis(axis)) * adinv;
            let t1 = (ax.max - r.orig.axis(axis)) * adinv;

            if t0 < t1 {
                if t0 > ray_t.min {
                    ray_t.min = t0;
                }
                if t1 < ray_t.max {
                    ray_t.max = t1;
                }
            } else {
                if t0 > ray_t.min {
                    ray_t.min = t1;
                }
                if t1 < ray_t.max {
                    ray_t.max = t0;
                }
            }

            if ray_t.max <= ray_t.min {
                return false;
            }
        }

        true
    }
}
