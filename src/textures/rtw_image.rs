use png::DecodingError;

use crate::primitives::{Color, color};

#[derive(Debug)]
pub struct RtwImage {
    pub pixel_data: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub bytes_per_row: u32,
    pub bytes_per_pixel: u32,
}

impl RtwImage {
    pub fn new(bytes: &[u8]) -> Result<Self, DecodingError> {
        let decoder = png::Decoder::new(bytes);
        let mut reader = decoder.read_info()?;

        let info = reader.info();
        let (width, height) = info.size();

        let bytes_per_pixel = info.bytes_per_pixel() as u32;
        let bytes_per_row = bytes_per_pixel * width;

        let mut buf = vec![0; reader.output_buffer_size()];

        reader.next_frame(&mut buf)?;

        Ok(Self {
            pixel_data: buf,
            width,
            height,
            bytes_per_row,
            bytes_per_pixel,
        })
    }

    pub fn get_raw_pixel(&self, x: u32, y: u32) -> &[u8] {
        let offset = y * self.bytes_per_row + x * self.bytes_per_pixel;
        &self.pixel_data[offset as usize..(offset + self.bytes_per_pixel) as usize]
    }

    pub fn pixel_data(&self, x: u32, y: u32) -> Color {
        let raw = self.get_raw_pixel(x, y);
        let [r, g, b, ..] = raw else {
            panic!();
            // return color(1.0, 0.0, 1.0); // return magenta
        };
        let scale = 1.0 / 255.0;
        color(
            f64::from(*r) * scale,
            f64::from(*g) * scale,
            f64::from(*b) * scale,
        )
    }
}
