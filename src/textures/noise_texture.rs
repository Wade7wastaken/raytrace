use std::{
    fmt::{self, Display},
    sync::Arc,
};

use crate::primitives::{Color, Point3, color};

use super::{Texture, perlin::Perlin};

pub struct NoiseTexture {
    noise: Perlin,
    scale: f64,
}

impl NoiseTexture {
    #[must_use]
    pub fn new(scale: f64) -> Self {
        Self {
            noise: Perlin::new(),
            scale,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: Point3) -> Color {
        color(0.5, 0.5, 0.5) * (1.0 + (self.scale * p.z + 10.0 * self.noise.turb(p, 7)).sin())
        // color(1.0, 1.0, 1.0) * self.noise.turb(p, 7)
    }
}

impl Display for NoiseTexture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "noise_texture({})", self.scale)
    }
}

#[must_use]
pub fn noise_texture(scale: f64) -> Arc<NoiseTexture> {
    Arc::new(NoiseTexture::new(scale))
}
