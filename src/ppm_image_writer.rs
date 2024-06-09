use std::{fmt::Write as WriteFmt, fs::File, io::Write as WriteIo, path::Path};

use crate::{color::Color, image_writer::ImageWriter};

pub struct PPMImageWriter {
    f: File,
    buffer: String,
}

impl PPMImageWriter {
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
            return Some(Self {
                f,
                buffer: String::new(),
            });
        }
        None
    }

    fn init(&mut self, image_width: u32, image_height: u32) {
        let pixel_expected_value = 4.0;
        let reserve_length =
            (image_width as f64 * image_height as f64 * 3.0 * pixel_expected_value).round(); // add 3 for space, space, newline

        println!("Reserving {} bytes", reserve_length);
        self.buffer.reserve((reserve_length) as usize);
        writeln!(self.buffer, "P3\n{} {}\n255", image_width, image_height).unwrap();
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
