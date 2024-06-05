use std::{fs::File, io::Write};

use crate::vec3::Color;

pub struct PPMImageWriter {
    f: File,
}

impl PPMImageWriter {
    pub fn new(path: &str) -> Self {
        Self {
            f: File::create(path).unwrap(),
        }
    }

    pub fn write(&mut self, data: String) {
        self.f.write_all(data.as_bytes()).unwrap();
    }

    pub fn write_header(&mut self, image_width: u32, image_height: u32) {
        self.write(format!("P3\n{} {}\n255\n", image_width, image_height));
    }

    pub fn write_pixel(&mut self, pixel_color: Color) {
        let r = pixel_color.x;
        let g = pixel_color.y;
        let b = pixel_color.z;

        let ir = ((r * 256.0) as i32).clamp(0, 255);
        let ig = ((g * 256.0) as i32).clamp(0, 255);
        let ib = ((b * 256.0) as i32).clamp(0, 255);

        self.write(format!("{} {} {}\n", ir, ig, ib));
    }
}
