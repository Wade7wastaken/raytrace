use crate::misc::rand_f64;
use derive_more::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use rand::distr::Distribution;
use rand::distr::StandardUniform;
use std::fmt;
use std::ops::Range;

#[derive(
    Debug,
    Clone,
    Copy,
    Default,
    PartialEq,
    Add,
    Sub,
    Mul,
    Div,
    Neg,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    #[must_use]
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    #[must_use]
    pub fn random() -> Self {
        Self {
            x: rand_f64(),
            y: rand_f64(),
            z: rand_f64(),
        }
    }

    #[must_use]
    pub fn random_range(range: Range<f64>) -> Self {
        Self {
            x: rand::random_range(range.clone()),
            y: rand::random_range(range.clone()),
            z: rand::random_range(range.clone()),
        }
    }

    #[must_use]
    pub fn random_in_unit_sphere() -> Self {
        loop {
            let p = Self::random_range(-1.0..1.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    #[must_use]
    pub fn random_in_unit_disk() -> Self {
        loop {
            let p = vec3(rand_f64(), rand_f64(), 0.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    #[must_use]
    pub fn random_unit_vector() -> Self {
        Self::random_in_unit_sphere().unit_vector()
    }

    #[must_use]
    pub fn random_on_hemisphere(normal: Self) -> Self {
        let on_unit_sphere = Self::random_unit_vector();
        if on_unit_sphere.dot(normal) > 0.0 {
            // In the same hemisphere as normal
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
    }

    #[must_use]
    pub fn axis(&self, index: u8) -> f64 {
        match index {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            _ => panic!("Incorrect index passed to axis"),
        }
    }

    #[must_use]
    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    #[must_use]
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    #[must_use]
    pub fn dot(&self, rhs: Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    #[must_use]
    pub fn cross(&self, rhs: Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    #[must_use]
    pub fn reflect(&self, n: Vec3) -> Vec3 {
        *self - n * self.dot(n) * 2.0
    }

    #[must_use]
    pub fn refract(&self, n: Vec3, eta_i_over_eta_t: f64) -> Vec3 {
        let cos_theta = (-*self).dot(n).min(1.0);
        let r_out_perp = (*self + n * cos_theta) * eta_i_over_eta_t;
        let r_out_parallel = n * -((1.0 - r_out_perp.length_squared()).abs()).sqrt();
        r_out_parallel + r_out_parallel
    }

    #[must_use]
    pub fn unit_vector(&self) -> Self {
        *self / self.length()
    }

    #[must_use]
    pub fn is_near_zero(&self) -> bool {
        let s = 1e-8;
        (self.x.abs() < s) && (self.y.abs() < s) && (self.z.abs() < s)
    }
}

impl Distribution<Vec3> for StandardUniform {
    fn sample<R: rand::Rng + ?Sized>(&self, _rng: &mut R) -> Vec3 {
        Vec3::random_in_unit_sphere().unit_vector()
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

pub type Point3 = Vec3;

// helper initializer to make code look pretty
#[must_use]
pub fn vec3(x: f64, y: f64, z: f64) -> Vec3 {
    Vec3::new(x, y, z)
}

#[must_use]
pub fn point3(x: f64, y: f64, z: f64) -> Point3 {
    Point3::new(x, y, z)
}
