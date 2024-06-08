use camera::{Camera, CameraOptions};
use hittable_list::HittableList;
use image_writer::ImageWriter;
use ppm_image_writer::PPMImageWriter;
use sphere::Sphere;
use vec3::Vec3;

mod camera;
mod hittable;
mod hittable_list;
mod image_writer;
mod interval;
mod ppm_image_writer;
mod rand;
mod ray;
mod sphere;
mod vec3;

fn main() {
    // World

    let mut world = HittableList::empty();

    world.add(Box::new(Sphere::new(Vec3::new(0.0, 0.1, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

    let mut cam = Camera::new(
        Box::new(PPMImageWriter::new("./output.ppm").unwrap()),
        CameraOptions {
            ..Default::default()
        },
    );

    cam.render(world);
}
