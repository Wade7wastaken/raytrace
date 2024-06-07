use std::rc::Rc;

use camera::Camera;
use hittable_list::HittableList;
use ppm_image_writer::PPMImageWriter;
use sphere::Sphere;
use vec3::{Point3, Vec3};

mod camera;
mod hittable;
mod hittable_list;
mod interval;
mod ppm_image_writer;
mod rand;
mod ray;
mod sphere;
mod vec3;

fn main() {
    // World

    let mut world = HittableList::empty();

    world.add(Rc::new(Sphere::new(Vec3::new(0.0, 0.1, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

    let mut cam = Camera::new(
        PPMImageWriter::new("./output.ppm"),
        16.0 / 9.0,
        400,
        1.0,
        10,
        2.0,
        Point3::new(0.0, 0.0, 0.0),
    );

    cam.render(world);
}
