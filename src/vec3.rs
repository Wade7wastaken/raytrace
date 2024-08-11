use crate::rand::rand;
use crate::rand::rand_range;
use derive_more::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use std::fmt;
use std::ops;
use std::ops::Range;

#[derive(
    Debug,
    Clone,
    Copy,
    Default,
    PartialEq,
    Add,
    AddAssign,
    Sub,
    SubAssign,
    Mul,
    MulAssign,
    Div,
    DivAssign,
    Neg,
)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn random() -> Self {
        Self {
            x: rand(),
            y: rand(),
            z: rand(),
        }
    }

    pub fn random_range(range: Range<f64>) -> Self {
        Self {
            x: rand_range(range.to_owned()),
            y: rand_range(range.to_owned()),
            z: rand_range(range.to_owned()),
        }
    }

    pub fn random_in_unit_sphere() -> Self {
        loop {
            let p = Self::random_range(-1.0..1.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn random_in_unit_disk() -> Self {
        loop {
            let p = vec3(rand(), rand(), 0.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn random_unit_vector() -> Self {
        Self::random_in_unit_sphere().unit_vector()
    }

    pub fn random_on_hemisphere(normal: Self) -> Self {
        let on_unit_sphere = Self::random_unit_vector();
        if on_unit_sphere.dot(normal) > 0.0 {
            // In the same hemisphere as normal
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
    }

    pub fn axis(&self, index: i32) -> f64 {
        match index {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            _ => panic!("Incorrect index passed to axis"),
        }
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn dot(&self, rhs: Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(&self, rhs: Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    pub fn reflect(&self, n: Vec3) -> Vec3 {
        *self - 2.0 * self.dot(n) * n
    }

    pub fn refract(&self, n: Vec3, eta_i_over_eta_t: f64) -> Vec3 {
        let cos_theta = (-*self).dot(n).min(1.0);
        let r_out_perp = eta_i_over_eta_t * (*self + cos_theta * n);
        let r_out_parallel = -((1.0 - r_out_perp.length_squared()).abs()).sqrt() * n;
        r_out_parallel + r_out_parallel
    }

    pub fn unit_vector(&self) -> Self {
        *self / self.length()
    }

    pub fn is_near_zero(&self) -> bool {
        let s = 1e-8;
        (self.x.abs() < s) && (self.y.abs() < s) && (self.z.abs() < s)
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

pub type Point3 = Vec3;

// helper initializer to make code look pretty
pub fn vec3(x: f64, y: f64, z: f64) -> Vec3 {
    Vec3::new(x, y, z)
}

pub fn point3(x: f64, y: f64, z: f64) -> Point3 {
    Point3::new(x, y, z)
}
