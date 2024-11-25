use std::error::Error;

use crate::{
    primitives::{Color, Point3},
    rtw_image::RtwImage,
};

use super::Texture;

pub struct ImageTexture {
    image: RtwImage,
}

impl ImageTexture {
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

        let i = (u * self.image.width as f64) as u32;
        let j = (v * self.image.height as f64) as u32;

        self.image.pixel_data(i, j)
    }
}
