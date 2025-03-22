use std::{
    error::Error,
    fmt::{self, Display},
    sync::Arc,
};

use crate::primitives::{Color, Point3};

use super::{Texture, rtw_image::RtwImage};

pub struct ImageTexture {
    image: RtwImage,
}

impl ImageTexture {
    #[must_use]
    pub fn new(image: RtwImage) -> Self {
        Self { image }
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            image: RtwImage::new(bytes)?,
        })
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _p: Point3) -> Color {
        let u = u.clamp(0.0, 1.0);
        let v = 1.0 - v.clamp(0.0, 1.0);

        let i = (u * f64::from(self.image.width)) as u32;
        let j = (v * f64::from(self.image.height)) as u32;

        self.image.pixel_data(i, j)
    }
}

impl Display for ImageTexture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "image_texture")
    }
}

#[must_use]
pub fn image_texture(image: RtwImage) -> Arc<ImageTexture> {
    Arc::new(ImageTexture::new(image))
}

pub fn image_texture_from_bytes(bytes: &[u8]) -> Result<Arc<ImageTexture>, Box<dyn Error>> {
    Ok(Arc::new(ImageTexture::from_bytes(bytes)?))
}
