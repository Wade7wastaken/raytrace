mod camera;
mod hittables;
mod image_writer;
mod materials;
mod misc;
mod primitives;
mod textures;

use std::time::Instant;

use camera::{Camera, CameraOptions};
use hittables::{HittableList, cube, quad, rotate_y, translate};
use image_writer::PNGImageWriter;

use materials::{diffuse_light_from_color, lambertian_from_color};
use primitives::{color, point3, vec3};

#[must_use]
pub fn cornell_box() -> (HittableList, Camera) {
    let mut world = HittableList::default();

    let red = lambertian_from_color(color(0.65, 0.05, 0.05));
    let white = lambertian_from_color(color(0.73, 0.73, 0.73));
    let green = lambertian_from_color(color(0.12, 0.45, 0.15));

    let light = diffuse_light_from_color(color(15.0, 15.0, 15.0));

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
        samples_per_pixel: 200,
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
    let name: &str = "cornel_box";
    let start = Instant::now();
    cam.render_and_save::<_, PNGImageWriter>(&world, format!("./output/{name}.png"))
        .expect("failed to save image");
    let end = start.elapsed();
    println!("Took {:.3}", end.as_secs_f64());
}
