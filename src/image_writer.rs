use std::{
    error::Error,
    fs::File,
    io::{BufWriter, Write as _},
    path::Path,
};

use crate::primitives::Color;

pub trait ImageWriter<T>: Sized {
    fn new(data: T, width: usize, height: usize) -> Result<Self, Box<dyn Error>>;
    fn write(&mut self, pixels: Vec<Vec<Color>>) -> Result<(), Box<dyn Error>>;
    fn finalize(&mut self) -> Result<(), Box<dyn Error>>;
}

pub struct PPMImageWriter {
    f: BufWriter<File>,
}

impl<T: AsRef<Path>> ImageWriter<T> for PPMImageWriter {
    fn new(data: T, width: usize, height: usize) -> Result<Self, Box<dyn Error>> {
        let f = File::create(data)?;
        let mut writer = BufWriter::new(f);

        writeln!(writer, "P3\n{width} {height}\n255")?;

        Ok(Self { f: writer })
    }

    fn write(&mut self, pixels: Vec<Vec<Color>>) -> Result<(), Box<dyn Error>> {
        for row in pixels {
            for pixel in row {
                let (r, g, b) = pixel.map(linear_to_gamma).to_rgb();

                writeln!(self.f, "{r} {g} {b}")?;
            }
        }

        Ok(())
    }

    fn finalize(&mut self) -> Result<(), Box<dyn Error>> {
        Ok(self.f.flush()?)
    }
}

pub struct PNGImageWriter {
    f: File,
    width: usize,
    height: usize,
}

impl<T: AsRef<Path>> ImageWriter<T> for PNGImageWriter {
    fn new(path: T, width: usize, height: usize) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            f: File::create(path)?,
            width,
            height,
        })
    }

    fn write(&mut self, pixels: Vec<Vec<Color>>) -> Result<(), Box<dyn Error>> {
        let mut encoder =
            png::Encoder::new(&self.f, self.width.try_into()?, self.height.try_into()?);
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

    fn finalize(&mut self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        linear_component.powf(1.0 / 2.2)
    } else {
        0.0
    }
}
