mod camera;
mod hittables;
mod material;
mod misc;
mod primitives;

use std::{fs::File, time::Instant};

use camera::{Camera, CameraOptions};
use hittables::quad;

use primitives::{color, point3, vec3};

use crate::{
    hittables::Quad,
    material::{diffuse_light, lambertian},
    primitives::Color,
};

fn write_png(path: &str, pixels: Vec<Vec<Color>>, width: usize, height: usize) {
    let f = File::create(path).unwrap();

    let mut encoder = png::Encoder::new(f, width as u32, height as u32);
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);
    encoder.set_source_gamma(png::ScaledFloat::new(1.0 / 2.2));
    let mut writer = encoder.write_header().unwrap();

    let data: Vec<u8> = pixels
        .into_iter()
        .flat_map(|row| {
            row.into_iter().flat_map(|pixel| {
                let (r, g, b) = pixel.map(linear_to_gamma).to_rgb();
                [r, g, b]
            })
        })
        .collect();
    writer.write_image_data(data.as_slice()).unwrap();
}

fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        linear_component.powf(1.0 / 2.2)
    } else {
        0.0
    }
}

#[must_use]
pub fn cornell_box() -> (Vec<Quad>, Camera) {
    let mut world = Vec::new();

    let red = lambertian(color(0.65, 0.05, 0.05));
    let white = lambertian(color(0.73, 0.73, 0.73));
    let green = lambertian(color(0.12, 0.45, 0.15));

    let light = diffuse_light(color(15.0, 15.0, 15.0));

    world.push(quad(
        point3(555.0, 0.0, 0.0),
        vec3(0.0, 0.0, 555.0),
        vec3(0.0, 555.0, 0.0),
        green,
    ));
    world.push(quad(
        point3(0.0, 0.0, 555.0),
        vec3(0.0, 0.0, -555.0),
        vec3(0.0, 555.0, 0.0),
        red,
    ));
    world.push(quad(
        point3(0.0, 555.0, 0.0),
        vec3(555.0, 0.0, 0.0),
        vec3(0.0, 0.0, 555.0),
        white.clone(),
    ));
    world.push(quad(
        point3(0.0, 0.0, 555.0),
        vec3(555.0, 0.0, 0.0),
        vec3(0.0, 0.0, -555.0),
        white.clone(),
    ));
    world.push(quad(
        point3(555.0, 0.0, 555.0),
        vec3(-555.0, 0.0, 0.0),
        vec3(0.0, 555.0, 0.0),
        white.clone(),
    ));

    world.push(quad(
        point3(213.0, 554.0, 226.0),
        vec3(130.0, 0.0, 0.0),
        vec3(0.0, 0.0, 105.0),
        light,
    ));

    // tall box
    world.push(quad(
        point3(424.377761, 0.000000, 252.294858),
        vec3(42.705142, 0.000000, 159.377761),
        vec3(0.000000, 330.000000, 0.000000),
        white.clone(),
    ));
    world.push(quad(
        point3(265.000000, 0.000000, 295.000000),
        vec3(42.705142, 0.000000, 159.377761),
        vec3(0.000000, 330.000000, 0.000000),
        white.clone(),
    ));
    world.push(quad(
        point3(307.705142, 0.000000, 454.377761),
        vec3(159.377761, 0.000000, -42.705142),
        vec3(0.000000, 330.000000, 0.000000),
        white.clone(),
    ));
    world.push(quad(
        point3(265.000000, 0.000000, 295.000000),
        vec3(159.377761, 0.000000, -42.705142),
        vec3(0.000000, 330.000000, 0.000000),
        white.clone(),
    ));
    world.push(quad(
        point3(265.000000, 330.000000, 295.000000),
        vec3(159.377761, 0.000000, -42.705142),
        vec3(42.705142, 0.000000, 159.377761),
        white.clone(),
    ));

    // short box
    world.push(quad(
        point3(286.924325, 0.000000, 115.987804),
        vec3(-50.987804, 0.000000, 156.924325),
        vec3(0.000000, 165.000000, 0.000000),
        white.clone(),
    ));
    world.push(quad(
        point3(130.000000, 0.000000, 65.000000),
        vec3(-50.987804, 0.000000, 156.924325),
        vec3(0.000000, 165.000000, 0.000000),
        white.clone(),
    ));
    world.push(quad(
        point3(79.012196, 0.000000, 221.924325),
        vec3(156.924325, 0.000000, 50.987804),
        vec3(0.000000, 165.000000, 0.000000),
        white.clone(),
    ));
    world.push(quad(
        point3(130.000000, 0.000000, 65.000000),
        vec3(156.924325, 0.000000, 50.987804),
        vec3(0.000000, 165.000000, 0.000000),
        white.clone(),
    ));
    world.push(quad(
        point3(130.000000, 165.000000, 65.000000),
        vec3(156.924325, 0.000000, 50.987804),
        vec3(-50.987804, 0.000000, 156.924325),
        white,
    ));

    let cam = Camera::new(CameraOptions {
        aspect_ratio: 1.0,
        image_width: 600,
        samples_per_pixel: 1000,
        v_fov: 40.0,
        look_from: point3(278.0, 278.0, -800.0),
        look_at: point3(278.0, 278.0, 0.0),
        ..Default::default()
    });

    (world, cam)
}

fn main() {
    let (world, cam) = cornell_box();

    let start = Instant::now();
    let pixels = cam.render(&world);
    let end = start.elapsed();

    println!("Rendering took {:.3}", end.as_secs_f64());

    let start = Instant::now();
    write_png(
        "output/cornel_box.png",
        pixels,
        cam.image_width,
        cam.image_height,
    );
    let end = start.elapsed();

    println!("Saving took {:.3}", end.as_secs_f64());
}
