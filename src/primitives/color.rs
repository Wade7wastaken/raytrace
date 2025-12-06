use derive_more::derive::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign, Sum};
use std::ops;

#[derive(
    Debug,
    Default,
    Clone,
    // Copy,
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
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    #[must_use]
    #[inline]
    pub const fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b }
    }

    #[must_use]
    #[inline]
    pub fn map(&self, pred: impl Fn(f32) -> f32) -> Self {
        Self {
            r: pred(self.r),
            g: pred(self.g),
            b: pred(self.b),
        }
    }

    #[must_use]
    #[inline]
    pub fn to_rgb(&self) -> (u8, u8, u8) {
        (
            channel_to_rgb(self.r),
            channel_to_rgb(self.g),
            channel_to_rgb(self.b),
        )
    }
}

impl ops::Mul<Color> for f32 {
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

#[must_use]
#[inline]
fn channel_to_rgb(channel: f32) -> u8 {
    ((channel * 255.999) as u8).clamp(0, 255)
}

#[must_use]
#[inline]
pub const fn color(r: f32, g: f32, b: f32) -> Color {
    Color::new(r, g, b)
}
