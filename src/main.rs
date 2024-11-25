#![allow(dead_code)]

use std::time::Instant;

use camera::{Camera, CameraOptions};
use hittables::{quad, sphere, sphere_moving, BvhNode, HittableList};
use image_writer::PNGImageWriter;
use materials::{dielectric, lambertian, lambertian_from_color, metal};
use primitives::{color, point3, vec3, Color};
use rand::{rand, rand_range};
use textures::{checker_texture_from_colors, image_texture_from_bytes, noise_texture};

mod camera;
mod hittables;
mod image_writer;
mod materials;
mod primitives;
mod rand;
mod textures;

/**
 * TODO:
 * from_str for parsing world objects
 */

fn simple() {
    let mut world = HittableList::default();

    let mat_ground = lambertian_from_color(color(0.8, 0.8, 0.0));
    let mat_center = lambertian_from_color(color(0.1, 0.2, 0.5));
    let mat_left = dielectric(1.50);
    let mat_bubble = dielectric(1.00 / 1.50);
    let mat_right = metal(color(0.8, 0.8, 0.8), 0.5);

    world.add(sphere(point3(0.0, -100.5, -1.0), 100.0, mat_ground));
    world.add(sphere(point3(0.0, 0.0, -1.2), 0.5, mat_center));
    world.add(sphere(point3(-1.0, 0.0, -1.0), 0.5, mat_left));
    world.add(sphere(point3(-1.0, 0.0, -1.0), 0.4, mat_bubble));
    world.add(sphere(point3(1.0, 0.0, -1.0), 0.5, mat_right));

    let cam = Camera::new(CameraOptions {
        look_from: point3(-2.0, 2.0, 1.0),
        v_fov: 20.0,
        defocus_angle: 10.0,
        focus_dist: 3.4,
        ..Default::default()
    });

    let image_writer =
        PNGImageWriter::new("./output/simple.png").expect("failed to initialize image writer");

    let start = Instant::now();
    cam.render_and_save(&world, image_writer)
        .expect("failed to save image");
    let end = start.elapsed();
    println!("Took {:.2?}", end);
}

fn bouncing_spheres() {
    let mut world = HittableList::default();

    let ground_material = lambertian(checker_texture_from_colors(
        0.32,
        color(0.2, 0.3, 0.1),
        color(0.9, 0.9, 0.9),
    ));

    world.add(sphere(point3(0.0, -1000.0, 0.0), 1000.0, ground_material));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rand();
            let center = point3(a as f64 + 0.9 * rand(), 0.2, b as f64 + 0.9 * rand());

            if (center - point3(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    let mat = lambertian_from_color(albedo);
                    world.add(sphere_moving(
                        center,
                        vec3(0.0, rand::rand_range(0.0..0.5), 0.0),
                        0.2,
                        mat,
                    ));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_range(0.5..1.0);
                    let fuzz = rand_range(0.0..0.5);
                    let mat = metal(albedo, fuzz);
                    world.add(sphere(center, 0.2, mat));
                } else {
                    // glass
                    let mat = dielectric(1.5);
                    world.add(sphere(center, 0.2, mat));
                }
            }
        }
    }

    let material1 = dielectric(1.5);
    world.add(sphere(point3(0.0, 1.0, 0.0), 1.0, material1));

    let material2 = lambertian_from_color(color(0.4, 0.2, 0.1));
    world.add(sphere(point3(-4.0, 1.0, 0.0), 1.0, material2));

    let material3 = metal(color(0.7, 0.6, 0.5), 0.0);
    world.add(sphere(point3(4.0, 1.0, 0.0), 1.0, material3));

    let cam = Camera::new(CameraOptions {
        v_fov: 20.0,
        look_from: point3(13.0, 2.0, 3.0),
        look_at: point3(0.0, 0.0, 0.0),
        defocus_angle: 0.6,
        ..Default::default()
    });

    let world_bvh = BvhNode::from_hittable_list(world);

    let image_writer = PNGImageWriter::new("./output/bouncing_spheres.png")
        .expect("failed to initialize image writer");

    let start = Instant::now();
    cam.render_and_save(&world_bvh, image_writer)
        .expect("failed to save image");
    let end = start.elapsed();
    println!("Took {:.2?}", end);
}

fn checkered_spheres() {
    let mut world = HittableList::default();

    let checker = lambertian(checker_texture_from_colors(
        0.32,
        color(0.2, 0.3, 0.1),
        color(0.9, 0.9, 0.9),
    ));

    world.add(sphere(point3(0.0, -10.0, 0.0), 10.0, checker.clone()));
    world.add(sphere(point3(0.0, 10.0, 0.0), 10.0, checker));

    let cam = Camera::new(CameraOptions {
        v_fov: 20.0,
        look_from: point3(13.0, 2.0, 3.0),
        look_at: point3(0.0, 0.0, 0.0),
        ..Default::default()
    });

    let image_writer = PNGImageWriter::new("./output/checkered_spheres.png")
        .expect("failed to initialize image writer");

    let start = Instant::now();
    cam.render_and_save(&world, image_writer)
        .expect("failed to save image");
    let end = start.elapsed();
    println!("Took {:.2?}", end);
}

fn earth() {
    let earth_texture = image_texture_from_bytes(include_bytes!("../textures/earthmap.png"))
        .expect("couldn't load texture");

    let earth_mat = lambertian(earth_texture);
    let globe = sphere(point3(0.0, 0.0, 0.0), 2.0, earth_mat);

    let cam = Camera::new(CameraOptions {
        v_fov: 20.0,
        look_from: point3(0.0, 0.0, 12.0),
        look_at: point3(0.0, 0.0, 0.0),
        vup: vec3(0.0, 1.0, 0.0),
        ..Default::default()
    });

    let image_writer =
        PNGImageWriter::new("./output/earth.png").expect("failed to initialize image writer");

    let start = Instant::now();
    cam.render_and_save(&*globe, image_writer)
        .expect("failed to save image");
    let end = start.elapsed();
    println!("Took {:.2?}", end);
}

fn perlin_spheres() {
    let mut world = HittableList::default();

    let perlin = lambertian(noise_texture(4.0));

    world.add(sphere(point3(0.0, -1000.0, 0.0), 1000.0, perlin.clone()));
    world.add(sphere(point3(0.0, 2.0, 0.0), 2.0, perlin));

    let cam = Camera::new(CameraOptions {
        v_fov: 20.0,
        look_from: point3(13.0, 2.0, 3.0),
        look_at: point3(0.0, 0.0, 0.0),
        ..Default::default()
    });

    let image_writer = PNGImageWriter::new("./output/perlin_spheres.png")
        .expect("failed to initialize image writer");

    let start = Instant::now();
    cam.render_and_save(&world, image_writer)
        .expect("failed to save image");
    let end = start.elapsed();
    println!("Took {:.2?}", end);
}

fn quads() {
    let mut world = HittableList::default();

    let left = lambertian_from_color(color(1.0, 0.2, 0.2));
    let back = lambertian_from_color(color(0.2, 1.0, 0.2));
    let right = lambertian_from_color(color(0.2, 0.2, 1.0));
    let upper = lambertian_from_color(color(1.0, 0.5, 0.0));
    let lower = lambertian_from_color(color(0.2, 0.8, 0.8));

    world.add(quad(
        point3(-3.0, -2.0, 5.0),
        vec3(0.0, 0.0, -4.0),
        vec3(0.0, 4.0, 0.0),
        left,
    ));
    world.add(quad(
        point3(-2.0, -2.0, 0.0),
        vec3(4.0, 0.0, 0.0),
        vec3(0.0, 4.0, 0.0),
        back,
    ));
    world.add(quad(
        point3(3.0, -2.0, 1.0),
        vec3(0.0, 0.0, 4.0),
        vec3(0.0, 4.0, 0.0),
        right,
    ));
    world.add(quad(
        point3(-2.0, 3.0, 1.0),
        vec3(4.0, 0.0, 0.0),
        vec3(0.0, 0.0, 4.0),
        upper,
    ));
    world.add(quad(
        point3(-2.0, -3.0, 5.0),
        vec3(4.0, 0.0, 0.0),
        vec3(0.0, 0.0, -4.0),
        lower,
    ));

    let cam = Camera::new(CameraOptions {
        aspect_ratio: 1.0,
        v_fov: 80.0,
        look_from: point3(0.0, 0.0, 9.0),
        look_at: point3(0.0, 0.0, 0.0),
        ..Default::default()
    });

    let image_writer =
        PNGImageWriter::new("./output/quads.png").expect("failed to initialize image writer");

    let start = Instant::now();
    cam.render_and_save(&world, image_writer)
        .expect("failed to save image");
    let end = start.elapsed();
    println!("Took {:.2?}", end);
}

fn main() {
    simple();
    bouncing_spheres();
    checkered_spheres();
    earth();
    perlin_spheres();
    quads();
}
