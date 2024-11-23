use std::{
    error::Error,
    fmt::Write as FmtWrite,
    fs::File,
    io::Write as IOWrite,
    path::Path,
};

use crate::color::Color;

pub trait ImageWriter {
    fn write(&mut self, pixels: Vec<Vec<Color>>) -> Result<(), Box<dyn Error>>;
}

pub struct PPMImageWriter {
    f: File,
    buffer: String,
}

impl PPMImageWriter {
    pub fn new(
        path: impl AsRef<Path>,
        image_width: u32,
        image_height: u32,
    ) -> Result<Self, Box<dyn Error>> {
        let f = File::create(path)?;
        let mut buffer = String::new();

        let pixel_expected_value = 4.0;
        let reserve_length =
            (image_width as f64 * image_height as f64 * 3.0 * pixel_expected_value).round(); // add 3 for space, space, newline

        println!("Reserving {} bytes", reserve_length);
        buffer.reserve((reserve_length) as usize);
        writeln!(buffer, "P3\n{} {}\n255", image_width, image_height)?;

        Ok(Self { f, buffer })
    }
}

impl ImageWriter for PPMImageWriter {
    fn write(&mut self, pixels: Vec<Vec<Color>>) -> Result<(), Box<dyn Error>> {
        for row in pixels {
            for pixel in row {
                let (r, g, b) = pixel.map(linear_to_gamma).to_rgb();

                writeln!(self.buffer, "{} {} {}", r, g, b)?;
            }
        }

        println!("Final buffer capacity: {}", self.buffer.capacity());
        println!("Final buffer len: {}", self.buffer.len());
        self.f.write_all(self.buffer.as_bytes())?;

        Ok(())
    }
}

pub struct PNGImageWriter {
    f: File,
}

impl PNGImageWriter {
    pub fn new(path: impl AsRef<Path>) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            f: File::create(path)?,
        })
    }
}

impl ImageWriter for PNGImageWriter {
    fn write(&mut self, pixels: Vec<Vec<Color>>) -> Result<(), Box<dyn Error>> {
        let height = pixels.len();
        let width = pixels.first().map(|row| row.len()).unwrap_or(0);

        let mut encoder = png::Encoder::new(&self.f, width.try_into()?, height.try_into()?);
        encoder.set_color(png::ColorType::Rgb);
        encoder.set_depth(png::BitDepth::Eight);
        encoder.set_source_gamma(png::ScaledFloat::new(1.0 / 2.2));
        let mut writer = encoder.write_header()?;

        let data: Vec<u8> = pixels
            .into_iter()
            .flat_map(|row| {
                row.into_iter().flat_map(|pixel| {
                    let (r, g, b) = pixel.map(linear_to_gamma).to_rgb();
                    [r, g, b]
                })
            })
            .collect();
        writer.write_image_data(data.as_slice())?;

        Ok(())
    }
}

fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        linear_component.powf(1.0 / 2.2)
        // linear_component.sqrt()
    } else {
        0.0
    }
}
