use std::{fmt::Write as FmtWrite, fs::File, io::Write as IOWrite, path::Path};

use crate::{camera::Camera, color::Color};

pub trait ImageWriter {
    // should change this to a result some day
    fn init(&mut self, cam: &Camera) -> Option<()>;

    fn write_pixel(&mut self, pixel_color: Color);

    fn finish(&mut self);
}

pub struct PPMImageWriter {
    f: File,
    buffer: String,
}

impl PPMImageWriter {
    pub fn new(path: impl AsRef<Path>) -> Option<Self> {
        if let Ok(f) = File::create(path) {
            return Some(Self {
                f,
                buffer: String::new(),
            });
        }
        None
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
    fn init(&mut self, cam: &Camera) -> Option<()> {
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
        .ok()?;

        Some(())
    }

    fn write_pixel(&mut self, pixel_color: Color) {
        let (r, g, b) = pixel_color
            .map(Self::linear_to_gamma)
            .map_any::<u8>(|channel| (channel.clamp(0.0, 1.0) * 255.0) as u8);

        writeln!(self.buffer, "{} {} {}", r, g, b).unwrap();
    }

    fn finish(&mut self) {
        println!("Final buffer capacity: {}", self.buffer.capacity());
        println!("Final buffer len: {}", self.buffer.len());
        self.f.write_all(self.buffer.as_bytes()).unwrap();
    }
}
