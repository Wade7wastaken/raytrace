use std::{rc::Rc, time::Instant};

use camera::{Camera, CameraOptions};
use hittable_list::HittableList;
use image_writer::ImageWriter;
use ppm_image_writer::PPMImageWriter;
use sphere::sphere;
use vec3::vec3;

mod camera;
mod color;
mod hittable;
mod hittable_list;
mod image_writer;
mod interval;
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

    world.add(Rc::new(sphere(vec3(0.0, 0.1, -1.0), 0.5)));
    world.add(Rc::new(sphere(vec3(0.0, -100.5, -1.0), 100.0)));

    let mut cam = Camera::new(
        Box::new(PPMImageWriter::new("./output.ppm").unwrap()),
        CameraOptions {
            max_depth: 50,
            samples_per_pixel: 30,
            ..Default::default()
        },
    );

    let start = Instant::now();
    cam.render(Rc::new(world));
    println!("Took {:.2?}", start.elapsed());
}
