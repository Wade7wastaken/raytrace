use std::time::Instant;

use raytrace::camera::Camera;
use raytrace::hittables::Hittable;
use raytrace::image_writer::PNGImageWriter;

use raytrace::example_worlds::*;

fn run<H: Hittable>((world, cam): (H, Camera), name: &str) {
    let image_writer = PNGImageWriter::new(format!("./output/{}.png", name))
        .expect("failed to initialize image writer");

    let start = Instant::now();
    cam.render_and_save(&world, image_writer)
        .expect("failed to save image");
    let end = start.elapsed();
    println!("Took {:.2?}", end);
}

fn main() {
    // simple();
    // bouncing_spheres();
    // checkered_spheres();
    // earth();
    // perlin_spheres();
    // quads();
    // simple_light();
    // run(cornell_box(), "cornel_box");
    // run(triangles(), "triangles");
    // run(room(), "room");
    run(book_2_final(), "book_2_final");
}
