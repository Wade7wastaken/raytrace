use std::rc::Rc;

use camera::{Camera, CameraOptions};
use hittable_list::HittableList;
use image_writer::ImageWriter;
use ppm_image_writer::PPMImageWriter;
use sphere::Sphere;
use vec3::Vec3;

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
* write to pre-allocated buffer, then write to file
* separate color from point/vec
*/

fn main() {
    // World

    let mut world = HittableList::empty();

    world.add(Rc::new(Sphere::new(Vec3::new(0.0, 0.1, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

    let mut cam = Camera::new(
        Box::new(PPMImageWriter::new("./output.ppm").unwrap()),
        CameraOptions {
            max_depth: 50,
            samples_per_pixel: 30,
            ..Default::default()
        },
    );

    cam.render(Rc::new(world));
}
