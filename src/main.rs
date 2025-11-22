mod camera;
mod hittables;
mod material;
mod misc;
mod primitives;

use std::{fs::File, time::Instant};

use camera::{Camera, CameraOptions};
use hittables::{HittableList, cube, quad, rotate_y, translate};

use primitives::{color, point3, vec3};

use crate::{
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
pub fn cornell_box() -> (HittableList, Camera) {
    let mut world = HittableList::default();

    let red = lambertian(color(0.65, 0.05, 0.05));
    let white = lambertian(color(0.73, 0.73, 0.73));
    let green = lambertian(color(0.12, 0.45, 0.15));

    let light = diffuse_light(color(15.0, 15.0, 15.0));

    world.add(quad(
        point3(555.0, 0.0, 0.0),
        vec3(0.0, 0.0, 555.0),
        vec3(0.0, 555.0, 0.0),
        green,
    ));
    world.add(quad(
        point3(0.0, 0.0, 555.0),
        vec3(0.0, 0.0, -555.0),
        vec3(0.0, 555.0, 0.0),
        red,
    ));
    world.add(quad(
        point3(0.0, 555.0, 0.0),
        vec3(555.0, 0.0, 0.0),
        vec3(0.0, 0.0, 555.0),
        white.clone(),
    ));
    world.add(quad(
        point3(0.0, 0.0, 555.0),
        vec3(555.0, 0.0, 0.0),
        vec3(0.0, 0.0, -555.0),
        white.clone(),
    ));
    world.add(quad(
        point3(555.0, 0.0, 555.0),
        vec3(-555.0, 0.0, 0.0),
        vec3(0.0, 555.0, 0.0),
        white.clone(),
    ));

    world.add(quad(
        point3(213.0, 554.0, 226.0),
        vec3(130.0, 0.0, 0.0),
        vec3(0.0, 0.0, 105.0),
        light,
    ));

    let box1 = cube(
        point3(0.0, 0.0, 0.0),
        point3(165.0, 330.0, 165.0),
        white.clone(),
    );
    let box1 = rotate_y(box1, 15.0);
    let box1 = translate(box1, vec3(265.0, 0.0, 295.0));
    world.add(box1);

    let box2 = cube(
        point3(0.0, 0.0, 0.0),
        point3(165.0, 165.0, 165.0),
        white.clone(),
    );
    let box2 = rotate_y(box2, -18.0);
    let box2 = translate(box2, vec3(130.0, 0.0, 65.0));
    world.add(box2);

    let cam = Camera::new(CameraOptions {
        aspect_ratio: 1.0,
        image_width: 600,
        samples_per_pixel: 1000,
        v_fov: 40.0,
        look_from: point3(278.0, 278.0, -800.0),
        look_at: point3(278.0, 278.0, 0.0),
        background: color(0.0, 0.0, 0.0),
        ..Default::default()
    })
    .unwrap();

    (world, cam)
}

fn main() {
    let (world, cam) = cornell_box();

    let start = Instant::now();
    let pixels = cam.render(&world);
    let end = start.elapsed();

    println!("Rendering took {:.3}", end.as_secs_f64());

    let start = Instant::now();
    write_png("output/cornel_box.png", pixels, cam.image_width, cam.image_height);
    let end = start.elapsed();

    println!("Saving took {:.3}", end.as_secs_f64());
}
