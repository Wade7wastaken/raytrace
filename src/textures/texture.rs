use std::fmt;

use crate::primitives::{Color, Point3};

pub trait Texture: Sync + Send + fmt::Display {
    fn value(&self, u: f64, v: f64, p: Point3) -> Color;
}
