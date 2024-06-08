use std::{fs::File, io::Write, path::Path};

use crate::{color::Color, image_writer::ImageWriter};

pub struct PPMImageWriter {
    f: File,
}

impl PPMImageWriter {
    pub fn write(&mut self, data: String) {
        self.f.write_all(data.as_bytes()).unwrap();
    }

    fn linear_to_gamma(linear_component: f64) -> f64 {
        if linear_component > 0.0 {
            linear_component.sqrt()
        } else {
            0.0
        }
    }
}

impl ImageWriter for PPMImageWriter {
    fn new(path: impl AsRef<Path>) -> Option<Self>
    where
        Self: Sized,
    {
        if let Ok(f) = File::create(path) {
            return Some(Self { f });
        }
        None
    }

    fn init(&mut self, image_width: u32, image_height: u32) {
        self.write(format!("P3\n{} {}\n255\n", image_width, image_height));
    }

    fn write_pixel(&mut self, pixel_color: Color) {
        let r = Self::linear_to_gamma(pixel_color.r);
        let g = Self::linear_to_gamma(pixel_color.g);
        let b = Self::linear_to_gamma(pixel_color.b);

        let ir = ((r * 256.0) as i32).clamp(0, 255);
        let ig = ((g * 256.0) as i32).clamp(0, 255);
        let ib = ((b * 256.0) as i32).clamp(0, 255);

        self.write(format!("{} {} {}\n", ir, ig, ib));
    }
}
