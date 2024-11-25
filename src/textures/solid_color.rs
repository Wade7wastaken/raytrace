use std::sync::Arc;

use crate::primitives::{Color, Point3};

use super::Texture;

pub struct SolidColor {
    albedo: Color,
}

impl SolidColor {
    pub fn new(color: Color) -> Self {
        Self { albedo: color }
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: Point3) -> Color {
        self.albedo
    }
}

pub fn solid_color(color: Color) -> Arc<SolidColor> {
    Arc::new(SolidColor::new(color))
}
