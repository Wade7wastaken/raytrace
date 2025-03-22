use std::{
    fmt::{self, Display},
    sync::Arc,
};

use crate::primitives::{Color, Point3};

use super::Texture;

pub struct SolidColor {
    albedo: Color,
}

impl SolidColor {
    #[must_use]
    pub fn new(color: Color) -> Self {
        Self { albedo: color }
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: Point3) -> Color {
        self.albedo
    }
}

impl Display for SolidColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "solid_color({})", self.albedo)
    }
}

#[must_use]
pub fn solid_color(color: Color) -> Arc<SolidColor> {
    Arc::new(SolidColor::new(color))
}
