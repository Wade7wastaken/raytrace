use std::{
    fmt::{self, Display},
    sync::Arc,
};

use crate::primitives::{Color, Point3};

use super::{SolidColor, Texture};

pub struct CheckerTexture {
    inv_scale: f64,
    even: Arc<dyn Texture>,
    odd: Arc<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(scale: f64, even: Arc<dyn Texture>, odd: Arc<dyn Texture>) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            even,
            odd,
        }
    }
    pub fn from_colors(scale: f64, c1: Color, c2: Color) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            even: Arc::new(SolidColor::new(c1)),
            odd: Arc::new(SolidColor::new(c2)),
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: Point3) -> Color {
        let x = (p.x * self.inv_scale).floor() as i32;
        let y = (p.y * self.inv_scale).floor() as i32;
        let z = (p.z * self.inv_scale).floor() as i32;

        let is_even = (x + y + z) % 2 == 0;

        if is_even {
            self.even.value(u, v, p)
        } else {
            self.odd.value(u, v, p)
        }
    }
}

impl Display for CheckerTexture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "checker_texture({}, {}, {})",
            1.0 / self.inv_scale,
            self.even,
            self.odd
        )
    }
}

pub fn checker_texture(
    scale: f64,
    even: Arc<dyn Texture>,
    odd: Arc<dyn Texture>,
) -> Arc<CheckerTexture> {
    Arc::new(CheckerTexture::new(scale, even, odd))
}

pub fn checker_texture_from_colors(scale: f64, c1: Color, c2: Color) -> Arc<CheckerTexture> {
    Arc::new(CheckerTexture::from_colors(scale, c1, c2))
}
