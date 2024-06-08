use std::path::Path;

use crate::vec3::Color;

pub trait ImageWriter {
    fn new(path: impl AsRef<Path>) -> Option<Self>
    where
        Self: Sized;

    fn init(&mut self, image_width: u32, image_height: u32);

    fn write_pixel(&mut self, pixel_color: Color);
}
