use crate::rand::{rand, rand_range};
use derive_more::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use std::{
    fmt,
    ops::{self, Range},
};

#[derive(
    Debug, Clone, Copy, PartialEq, Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign,
)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }

    pub fn empty() -> Self {
        Self {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        }
    }

    pub fn random() -> Self {
        Self {
            r: rand(),
            g: rand(),
            b: rand(),
        }
    }

    pub fn random_range(range: Range<f64>) -> Self {
        Self {
            r: rand_range(range.to_owned()),
            g: rand_range(range.to_owned()),
            b: rand_range(range.to_owned()),
        }
    }

    pub fn map(&self, pred: fn(f64) -> f64) -> Self {
        Self {
            r: pred(self.r),
            g: pred(self.g),
            b: pred(self.b),
        }
    }

    pub fn map_any<T>(&self, pred: fn(f64) -> T) -> (T, T, T) {
        (pred(self.r), pred(self.g), pred(self.b))
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "rgb({}, {}, {})", self.r, self.g, self.b)
    }
}

impl ops::Mul<Color> for f64 {
    type Output = Color;
    fn mul(self, rhs: Color) -> Self::Output {
        rhs * self
    }
}

// hack for multiplying two colors together
impl ops::Mul<Color> for Color {
    type Output = Color;
    fn mul(self, rhs: Color) -> Self::Output {
        Color {
            r: self.r * rhs.r,
            b: self.b * rhs.b,
            g: self.g * rhs.g,
        }
    }
}

// helper initializer to make code look pretty
pub fn color(r: f64, g: f64, b: f64) -> Color {
    Color::new(r, g, b)
}
