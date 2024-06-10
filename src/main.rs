use std::{rc::Rc, time::Instant};

use camera::{Camera, CameraOptions};
use color::color;
use hittable_list::HittableList;
use image_writer::ImageWriter;
use material::{Dielectric, Lambertian, Metal};
use ppm_image_writer::PPMImageWriter;
use sphere::sphere;
use vec3::{point3, vec3};

mod camera;
mod color;
mod hittable;
mod hittable_list;
mod image_writer;
mod interval;
mod material;
mod ppm_image_writer;
mod rand;
mod ray;
mod sphere;
mod vec3;

/*
TODO:
* no copy on vec3 and friends
* timer logging
* from_str for parsing world objects
* PNG saving
*/

fn main() {
    // World

    let mut world = HittableList::empty();

    let mat_ground = Rc::new(Lambertian::new(color(0.8, 0.8, 0.0)));
    let mat_center = Rc::new(Lambertian::new(color(0.1, 0.2, 0.5)));
    let mat_left = Rc::new(Dielectric::new(1.50));
    let mat_bubble = Rc::new(Dielectric::new(1.00 / 1.50));
    let mat_right = Rc::new(Metal::new(color(0.8, 0.6, 0.2), 0.7));

    world.add(Rc::new(sphere(
        point3(0.0, -100.5, -1.0),
        100.0,
        mat_ground.clone(),
    )));
    world.add(Rc::new(sphere(
        point3(0.0, 0.0, -1.2),
        0.5,
        mat_center.clone(),
    )));
    world.add(Rc::new(sphere(
        point3(-1.0, 0.0, -1.0),
        0.5,
        mat_left.clone(),
    )));
    world.add(Rc::new(sphere(
        point3(-1.0, 0.0, -1.0),
        0.4,
        mat_bubble.clone(),
    )));
    world.add(Rc::new(sphere(
        point3(1.0, 0.0, -1.0),
        0.5,
        mat_right.clone(),
    )));

    let mut cam = Camera::new(
        Box::new(PPMImageWriter::new("./output.ppm").unwrap()),
        CameraOptions {
            // max_depth: 100,
            // samples_per_pixel: 200,
            // image_width: 1920,
            ..Default::default()
        },
    );

    let start = Instant::now();
    cam.render(Rc::new(world));
    println!("Took {:.2?}", start.elapsed());
}
