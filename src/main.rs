#![allow(dead_code)]

use std::{sync::Arc, time::Instant};

use bvh_node::BvhNode;
use camera::{Camera, CameraOptions};
use color::{color, Color};
use hittable_list::HittableList;
use image_writer::PNGImageWriter;
use material::{Dielectric, Lambertian, Metal};
use rand::{rand, rand_range};
use sphere::{sphere, Sphere};
use texture::{CheckerTexture, ImageTexture, NoiseTexture};
use vec3::{point3, vec3};

mod aabb;
mod bvh_node;
mod camera;
mod color;
mod hittable;
mod hittable_list;
mod image_writer;
mod interval;
mod material;
mod perlin;
mod rand;
mod ray;
mod rtw_image;
mod sphere;
mod texture;
mod vec3;

/**
 * TODO:
 * from_str for parsing world objects
 * PNG saving
 * Reduce Arc::new boilerplate
 */

fn simple() {
    let mut world = HittableList::default();

    let mat_ground = Arc::new(Lambertian::from_color(color(0.8, 0.8, 0.0)));
    let mat_center = Arc::new(Lambertian::from_color(color(0.1, 0.2, 0.5)));
    let mat_left = Arc::new(Dielectric::new(1.50));
    let mat_bubble = Arc::new(Dielectric::new(1.00 / 1.50));
    let mat_right = Arc::new(Metal::new(color(0.8, 0.8, 0.8), 0.5));

    world.take(sphere(point3(0.0, -100.5, -1.0), 100.0, mat_ground));
    world.take(sphere(point3(0.0, 0.0, -1.2), 0.5, mat_center));
    world.take(sphere(point3(-1.0, 0.0, -1.0), 0.5, mat_left));
    world.take(sphere(point3(-1.0, 0.0, -1.0), 0.4, mat_bubble));
    world.take(sphere(point3(1.0, 0.0, -1.0), 0.5, mat_right));

    let cam = Camera::new(CameraOptions {
        max_depth: 20,
        samples_per_pixel: 100,
        image_width: 400,
        look_from: point3(-2.0, 2.0, 1.0),
        v_fov: 20.0,

        defocus_angle: 10.0,
        focus_dist: 3.4,
        ..Default::default()
    });

    let image_writer =
        PNGImageWriter::new("./output/simple.png").expect("failed to initialize PPMImageWriter");

    let world_bvh = BvhNode::from_hittable_list(world);

    let start = Instant::now();
    cam.render_and_save(&world_bvh, image_writer).unwrap();
    println!("Took {:.2?}", start.elapsed());
}

fn bouncing_spheres() {
    let mut world = HittableList::default();

    let ground_material = Arc::new(Lambertian::new(Arc::new(CheckerTexture::from_colors(
        0.32,
        color(0.2, 0.3, 0.1),
        color(0.9, 0.9, 0.9),
    ))));
    world.take(sphere(point3(0.0, -1000.0, 0.0), 1000.0, ground_material));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rand();
            let center = point3(a as f64 + 0.9 * rand(), 0.2, b as f64 + 0.9 * rand());

            if (center - point3(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    let mat = Arc::new(Lambertian::from_color(albedo));
                    world.take(Sphere::new_moving(
                        center,
                        vec3(0.0, rand::rand_range(0.0..0.5), 0.0),
                        0.2,
                        mat,
                    ));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_range(0.5..1.0);
                    let fuzz = rand_range(0.0..0.5);
                    let mat = Arc::new(Metal::new(albedo, fuzz));
                    world.take(sphere(center, 0.2, mat));
                } else {
                    // glass
                    let mat = Arc::new(Dielectric::new(1.5));
                    world.take(sphere(center, 0.2, mat));
                }
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    world.take(sphere(point3(0.0, 1.0, 0.0), 1.0, material1));

    let material2 = Arc::new(Lambertian::from_color(color(0.4, 0.2, 0.1)));
    world.take(sphere(point3(-4.0, 1.0, 0.0), 1.0, material2));

    let material3 = Arc::new(Metal::new(color(0.7, 0.6, 0.5), 0.0));
    world.take(sphere(point3(4.0, 1.0, 0.0), 1.0, material3));

    let cam = Camera::new(CameraOptions {
        image_width: 400,
        samples_per_pixel: 100,
        max_depth: 50,
        v_fov: 20.0,
        look_from: point3(13.0, 2.0, 3.0),
        look_at: point3(0.0, 0.0, 0.0),
        defocus_angle: 0.6,
        ..Default::default()
    });

    let world_bvh = BvhNode::from_hittable_list(world);

    let image_writer = PNGImageWriter::new("./output/bouncing_spheres.png")
        .expect("failed to initialize PPMImageWriter");

    let start = Instant::now();
    cam.render_and_save(&world_bvh, image_writer).unwrap();
    println!("Took {:.2?}", start.elapsed());
}

fn checkered_spheres() {
    let mut world = HittableList::default();

    let checker = Arc::new(Lambertian::new(Arc::new(CheckerTexture::from_colors(
        0.32,
        color(0.2, 0.3, 0.1),
        color(0.9, 0.9, 0.9),
    ))));

    world.take(sphere(point3(0.0, -10.0, 0.0), 10.0, checker.clone()));
    world.take(sphere(point3(0.0, 10.0, 0.0), 10.0, checker));

    let cam = Camera::new(CameraOptions {
        image_width: 400,
        samples_per_pixel: 100,
        max_depth: 50,
        v_fov: 20.0,
        look_from: point3(13.0, 2.0, 3.0),
        look_at: point3(0.0, 0.0, 0.0),
        defocus_angle: 0.0,
        ..Default::default()
    });

    let image_writer = PNGImageWriter::new("./output/checkered_spheres.png")
        .expect("failed to initialize PPMImageWriter");

    let start = Instant::now();
    cam.render_and_save(&world, image_writer).unwrap();
    println!("Took {:.2?}", start.elapsed());
}

fn earth() {
    let earth_texture = Arc::new(
        ImageTexture::from_bytes(include_bytes!("../textures/earthmap.png"))
            .expect("couldn't load texture"),
    );
    let earth_mat = Arc::new(Lambertian::new(earth_texture));
    let globe = sphere(point3(0.0, 0.0, 0.0), 2.0, earth_mat);

    let cam = Camera::new(CameraOptions {
        image_width: 400,
        samples_per_pixel: 100,
        max_depth: 50,
        v_fov: 20.0,
        look_from: point3(0.0, 0.0, 12.0),
        look_at: point3(0.0, 0.0, 0.0),
        vup: vec3(0.0, 1.0, 0.0),
        defocus_angle: 0.0,
        ..Default::default()
    });

    let image_writer =
        PNGImageWriter::new("./output/earth.png").expect("failed to initialize PPMImageWriter");

    let start = Instant::now();
    cam.render_and_save(&globe, image_writer).unwrap();
    println!("Took {:.2?}", start.elapsed());
}

fn perlin_spheres() {
    let mut world = HittableList::default();

    let perlin = Arc::new(Lambertian::new(Arc::new(NoiseTexture::new(4.0))));

    world.take(sphere(point3(0.0, -1000.0, 0.0), 1000.0, perlin.clone()));
    world.take(sphere(point3(0.0, 2.0, 0.0), 2.0, perlin));

    let cam = Camera::new(CameraOptions {
        image_width: 400,
        samples_per_pixel: 100,
        max_depth: 50,
        v_fov: 20.0,
        look_from: point3(13.0, 2.0, 3.0),
        look_at: point3(0.0, 0.0, 0.0),
        defocus_angle: 0.0,
        ..Default::default()
    });

    let image_writer = PNGImageWriter::new("./output/perlin_spheres.png")
        .expect("failed to initialize PPMImageWriter");

    let start = Instant::now();
    cam.render_and_save(&world, image_writer).unwrap();
    println!("Took {:.2?}", start.elapsed());
}

fn main() {
    perlin_spheres();
}
