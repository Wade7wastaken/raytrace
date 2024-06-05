use std::{
    f64::INFINITY,
    io::{self, Write},
    rc::Rc,
};

use hittable::Hittable;
use hittable_list::HittableList;
use interval::Interval;
use ppm_image_writer::PPMImageWriter;
use ray::Ray;
use sphere::Sphere;
use vec3::{Color, Point3, Vec3};

mod hittable;
mod hittable_list;
mod interval;
mod ppm_image_writer;
mod ray;
mod sphere;
mod vec3;

fn ray_color(r: &Ray, world: &HittableList) -> Color {
    if let Some(rec) = world.hit(r, Interval::new(0.0, INFINITY)) {
        return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
    }

    let unit_direction = r.dir.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;

    let image_height = (image_width as f64 / aspect_ratio) as u32;

    // ensure dimensions are more than 0
    assert!(image_width > 0);
    assert!(image_height > 0);

    // World

    let mut world = HittableList::empty();

    world.add(Rc::new(Sphere::new(Vec3::new(0.0, 0.1, -7.0), 0.5)));
    world.add(Rc::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = Point3::empty();

    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    let viewport_upper_left =
        camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;

    let pixel100_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    let mut w = PPMImageWriter::new("./output.ppm");

    w.write_header(image_width, image_height);

    for y in 0..image_height {
        print!("\rScanlines remaining: {}", image_height - y);
        io::stdout().flush().unwrap();
        for x in 0..image_width {
            let pixel_center =
                pixel100_loc + (x as f64 * pixel_delta_u) + (y as f64 * pixel_delta_v);

            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);

            let pixel_color = ray_color(&r, &world);

            w.write_pixel(pixel_color);
        }
    }
    println!("\nDone!");
}
