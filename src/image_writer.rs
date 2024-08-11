use std::{fmt::Write as FmtWrite, fs::File, io::Write as IOWrite, path::Path};

use crate::{camera::Camera, color::Color};

pub trait ImageWriter {
    // should change this to a result some day
    fn init(&mut self, cam: &Camera) -> Result<(), std::fmt::Error>;

    fn write_pixel(&mut self, pixel_color: Color) -> Result<(), std::fmt::Error>;

    fn finish(&mut self) -> Result<(), std::io::Error>;
}

pub struct PPMImageWriter {
    f: File,
    buffer: String,
}

impl PPMImageWriter {
    pub fn new(path: impl AsRef<Path>) -> Result<Self, std::io::Error> {
        File::create(path).map(|f| Self {
            f,
            buffer: String::new(),
        })
    }
}

impl ImageWriter for PPMImageWriter {
    fn init(&mut self, cam: &Camera) -> Result<(), std::fmt::Error> {
        let pixel_expected_value = 4.0;
        let reserve_length =
            (cam.image_width as f64 * cam.image_height as f64 * 3.0 * pixel_expected_value).round(); // add 3 for space, space, newline

        println!("Reserving {} bytes", reserve_length);
        self.buffer.reserve((reserve_length) as usize);
        writeln!(
            self.buffer,
            "P3\n{} {}\n255",
            cam.image_width, cam.image_height
        )
    }

    fn write_pixel(&mut self, pixel_color: Color) -> Result<(), std::fmt::Error> {
        let (r, g, b) = pixel_color.map(linear_to_gamma).to_rgb();

        writeln!(self.buffer, "{} {} {}", r, g, b)
    }

    fn finish(&mut self) -> Result<(), std::io::Error> {
        println!("Final buffer capacity: {}", self.buffer.capacity());
        println!("Final buffer len: {}", self.buffer.len());
        self.f.write_all(self.buffer.as_bytes())
    }
}

fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        linear_component.sqrt()
    } else {
        0.0
    }
}
