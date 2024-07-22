use std::{rc::Rc, time::Instant};

use camera::{Camera, CameraOptions};
use color::{color, Color};
use hittable_list::HittableList;
use image_writer::PPMImageWriter;
use material::{Dielectric, Lambertian, Metal};
use rand::{rand, rand_range};
use sphere::sphere;
use vec3::point3;

mod camera;
mod color;
mod hittable;
mod hittable_list;
mod image_writer;
mod interval;
mod material;
mod rand;
mod ray;
mod sphere;
mod vec3;

/**
 * TODO:
 * rayon
 * no copy on vec3 and friends
 * timer logging
 * from_str for parsing world objects
 * PNG saving
 * Default trait for vec3 to replace empty()
 * Reduce Rc::new boilerplate
 */

fn scene1() {
    let mut world = HittableList::empty();

    let mat_ground = Rc::new(Lambertian::new(color(0.8, 0.8, 0.0)));
    let mat_center = Rc::new(Lambertian::new(color(0.1, 0.2, 0.5)));
    let mat_left = Rc::new(Dielectric::new(1.50));
    let mat_bubble = Rc::new(Dielectric::new(1.00 / 1.50));
    let mat_right = Rc::new(Metal::new(color(0.8, 0.8, 0.8), 0.5));

    world.take(sphere(point3(0.0, -100.5, -1.0), 100.0, mat_ground));
    world.take(sphere(point3(0.0, 0.0, -1.2), 0.5, mat_center));
    world.take(sphere(point3(-1.0, 0.0, -1.0), 0.5, mat_left));
    world.take(sphere(point3(-1.0, 0.0, -1.0), 0.4, mat_bubble));
    world.take(sphere(point3(1.0, 0.0, -1.0), 0.5, mat_right));

    let mut cam = Camera::new(CameraOptions {
        max_depth: 50,
        samples_per_pixel: 100,
        // image_width: 1920,
        look_from: point3(-2.0, 2.0, 1.0),
        v_fov: 20.0,

        defocus_angle: 10.0,
        focus_dist: 3.4,
        ..Default::default()
    });

    let image_writer =
        PPMImageWriter::new("./output.ppm").expect("failed to initialize PPMImageWriter");

    let start = Instant::now();
    cam.render(image_writer, Rc::new(world));
    println!("Took {:.2?}", start.elapsed());
}

fn scene2() {
    let mut world = HittableList::empty();

    let ground_material = Rc::new(Lambertian::new(color(0.5, 0.5, 0.5)));
    world.take(sphere(point3(0.0, -1000.0, 0.0), 1000.0, ground_material));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rand();
            let center = point3(a as f64 + 0.9 * rand(), 0.2, b as f64 + 0.9 * rand());

            if (center - point3(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    let mat = Rc::new(Lambertian::new(albedo));
                    world.take(sphere(center, 0.2, mat));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_range(0.5..1.0);
                    let fuzz = rand_range(0.0..0.5);
                    let mat = Rc::new(Metal::new(albedo, fuzz));
                    world.take(sphere(center, 0.2, mat));
                } else {
                    // glass
                    let mat = Rc::new(Dielectric::new(1.5));
                    world.take(sphere(center, 0.2, mat));
                }
            }
        }
    }

    let material1 = Rc::new(Dielectric::new(1.5));
    world.take(sphere(point3(0.0, 1.0, 0.0), 1.0, material1));

    let material2 = Rc::new(Lambertian::new(color(0.4, 0.2, 0.1)));
    world.take(sphere(point3(-4.0, 1.0, 0.0), 1.0, material2));

    let material3 = Rc::new(Metal::new(color(0.7, 0.6, 0.5), 0.0));
    world.take(sphere(point3(4.0, 1.0, 0.0), 1.0, material3));

    let mut cam = Camera::new(CameraOptions {
        image_width: 1920,
        samples_per_pixel: 500,
        max_depth: 50,
        v_fov: 20.0,
        look_from: point3(13.0, 2.0, 3.0),
        look_at: point3(0.0, 0.0, 0.0),
        defocus_angle: 0.6,
        ..Default::default()
    });

    let image_writer =
        PPMImageWriter::new("./output.ppm").expect("failed to initialize PPMImageWriter");

    let start = Instant::now();
    cam.render(image_writer, Rc::new(world));
    println!("Took {:.2?}", start.elapsed());
}

fn main() {
    scene1();
}
