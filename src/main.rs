use std::sync::Arc;
use std::time::Instant;

use raytrace::camera::{Camera, CameraOptions};

use raytrace::hittables::{triangle, BvhNode, Hittable, HittableList};
use raytrace::image_writer::PNGImageWriter;
use raytrace::materials::{diffuse_light_from_color, lambertian, lambertian_from_color, Material};
use raytrace::primitives::{color, point3, vec3, Point3};
use raytrace::textures::image_texture_from_bytes;

fn room() -> (BvhNode, Camera) {
    let (gltf, buffers, _) = gltf::import("room.gltf").unwrap();

    let mut world = HittableList::default();

    let materials: [Arc<dyn Material>; 8] = [
        lambertian_from_color(color(
            0.137254998087883,
            0.137254998087883,
            0.137254998087883,
        )),
        lambertian_from_color(color(
            0.22745099663734436,
            0.22745099663734436,
            0.22745099663734436,
        )),
        lambertian_from_color(color(
            0.20000000298023224,
            0.20000000298023224,
            0.20000000298023224,
        )),
        lambertian_from_color(color(0.7, 0.7, 0.7)),
        lambertian(image_texture_from_bytes(include_bytes!("../Wood_Floor_Light.png")).unwrap()),
        lambertian(image_texture_from_bytes(include_bytes!("../Wood_Floor.png")).unwrap()),
        diffuse_light_from_color(color(
            0.3921569883823395,
            0.5843139886856079,
            0.9294120073318481,
        )),
        lambertian_from_color(color(
            0.11764699965715408,
            0.11764699965715408,
            0.11764699965715408,
        )),
    ];

    // assuming only one scene, one node, no children, and one mesh
    let mesh = gltf.meshes().next().unwrap();
    for primitive in mesh.primitives() {
        let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
        let positions = reader
            .read_positions()
            .unwrap()
            .map(|x| Point3::new(x[0] as f64, x[1] as f64, x[2] as f64))
            .collect::<Vec<_>>();
        let indices = reader
            .read_indices()
            .unwrap()
            .into_u32()
            .map(|x| x as usize)
            .collect::<Vec<_>>();

        let mat_idx = primitive.material().index().unwrap();

        for tri in indices.chunks(3) {
            if tri.len() == 3 {
                let p1 = positions[tri[0]];
                let p2 = positions[tri[1]];
                let p3 = positions[tri[2]];
                let t = triangle(p1, p2, p3, materials[mat_idx].clone());
                world.add(t);
            }
        }
    }

    let cam = Camera::new(CameraOptions {
        aspect_ratio: 1.0,
        image_width: 1000,
        v_fov: 50.0,
        samples_per_pixel: 1000,
        look_from: point3(4.0, 10.0, -15.0),
        look_at: point3(4.0, -5.0, 0.0),
        background: color(0.0, 0.0, 0.0),
        vup: vec3(0.0, 0.0, -1.0),
        ..Default::default()
    });

    (BvhNode::from_hittable_list(world), cam)
}

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
    run(room(), "room");
}
