use crate::rand::{rand, rand_range};
use derive_more::{derive::Sum, Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
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
    AddAssign,
    Sub,
    SubAssign,
    Mul,
    MulAssign,
    Div,
    DivAssign,
    Sum,
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
        write!(f, "#{:02x}{:02x}{:02x}", r, g, b)
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

pub fn color(r: f64, g: f64, b: f64) -> Color {
    Color::new(r, g, b)
}