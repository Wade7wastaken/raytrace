use crate::misc::rand_f64;
use derive_more::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign, derive::Sum};
use std::{
    fmt,
    ops::{self, Range},
};

#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    Add,
    Sub,
    Mul,
    Div,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
    Sum,
)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    #[must_use]
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }

    #[must_use]
    pub fn random() -> Self {
        Self {
            r: rand_f64(),
            g: rand_f64(),
            b: rand_f64(),
        }
    }

    #[must_use]
    pub fn random_range(range: Range<f64>) -> Self {
        Self {
            r: rand::random_range(range.clone()),
            g: rand::random_range(range.clone()),
            b: rand::random_range(range.clone()),
        }
    }

    #[must_use]
    pub fn map(&self, pred: fn(f64) -> f64) -> Self {
        Self {
            r: pred(self.r),
            g: pred(self.g),
            b: pred(self.b),
        }
    }

    #[must_use]
    pub fn to_rgb(self) -> (u8, u8, u8) {
        (
            channel_to_rgb(self.r),
            channel_to_rgb(self.g),
            channel_to_rgb(self.b),
        )
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (r, g, b) = self.to_rgb();
        write!(f, "#{r:02x}{g:02x}{b:02x}")
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

fn channel_to_rgb(channel: f64) -> u8 {
    ((channel * 255.999) as u8).clamp(0, 255)
}

#[must_use]
pub fn color(r: f64, g: f64, b: f64) -> Color {
    Color::new(r, g, b)
}
