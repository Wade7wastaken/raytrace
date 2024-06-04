use std::{
    fs::File,
    io::{self, Write},
};

use vec3::Color;

mod color;
mod vec3;

struct ImageWriter {
    f: File,
}

impl ImageWriter {
    fn new(path: &str) -> Self {
        Self {
            f: File::create(path).unwrap(),
        }
    }

    fn write(&mut self, data: &str) {
        self.f.write_all(data.as_bytes()).unwrap();
    }

    fn write_pixel(&mut self, pixel_color: Color) {
        let r = pixel_color.x;
        let g = pixel_color.y;
        let b = pixel_color.z;

        let ir = ((r * 256.0) as i32).clamp(0, 255);
        let ig = ((g * 256.0) as i32).clamp(0, 255);
        let ib = ((b * 256.0) as i32).clamp(0, 255);

        self.write(format!("{} {} {}\n", ir, ig, ib).as_str());
    }
}

fn main() {
    let image_width = 256;
    let image_height = 256;

    let mut f = ImageWriter::new("./output.ppm");

    f.write(format!("P3\n{} {}\n255\n", image_width, image_height).as_str());

    for y in 0..image_height {
        print!("\rScanlines remaining: {}", image_height - y);
        io::stdout().flush().unwrap();
        for x in 0..image_width {
            let r = x as f64 / (image_width - 1) as f64;
            let g = y as f64 / (image_height - 1) as f64;
            let b = 0.0;

            f.write_pixel(Color::new(r, g, b));
        }
    }
    println!("\nDone!");
}
