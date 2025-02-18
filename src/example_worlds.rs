use std::sync::Arc;

use crate::{
    camera::{Camera, CameraOptions},
    hittables::{cube, quad, rotate_y, sphere, sphere_moving, translate, triangle, BvhNode, Hittable, HittableList},
    materials::{dielectric, diffuse_light_from_color, lambertian, lambertian_from_color, metal, Material},
    primitives::{color, point3, vec3, Color, Point3},
    rand::{self, rand},
    textures::{checker_texture_from_colors, image_texture_from_bytes, noise_texture},
};

pub fn simple() -> (HittableList, Camera) {
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
        background: color(0.7, 0.8, 1.0),
        ..Default::default()
    });

    (world, cam)
}

pub fn bouncing_spheres() -> (BvhNode, Camera) {
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
                    let fuzz = rand::rand_range(0.0..0.5);
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
        background: color(0.7, 0.8, 1.0),
        ..Default::default()
    });

    let world_bvh = BvhNode::from_hittable_list(world);

    (world_bvh, cam)
}

pub fn checkered_spheres() -> (HittableList, Camera) {
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
        background: color(0.7, 0.8, 1.0),
        ..Default::default()
    });

    (world, cam)
}

pub fn earth() -> (HittableList, Camera) {
    let mut world = HittableList::default();
    let earth_texture = image_texture_from_bytes(include_bytes!("../textures/earthmap.png"))
        .expect("couldn't load texture");

    let earth_mat = lambertian(earth_texture);
    let globe = sphere(point3(0.0, 0.0, 0.0), 2.0, earth_mat);

    world.add(globe);

    let cam = Camera::new(CameraOptions {
        v_fov: 20.0,
        look_from: point3(0.0, 0.0, 12.0),
        look_at: point3(0.0, 0.0, 0.0),
        vup: vec3(0.0, 1.0, 0.0),
        background: color(0.7, 0.8, 1.0),
        ..Default::default()
    });

    (world, cam)
}

pub fn perlin_spheres() -> (HittableList, Camera) {
    let mut world = HittableList::default();

    let perlin = lambertian(noise_texture(4.0));

    world.add(sphere(point3(0.0, -1000.0, 0.0), 1000.0, perlin.clone()));
    world.add(sphere(point3(0.0, 2.0, 0.0), 2.0, perlin));

    let cam = Camera::new(CameraOptions {
        v_fov: 20.0,
        look_from: point3(13.0, 2.0, 3.0),
        look_at: point3(0.0, 0.0, 0.0),
        background: color(0.7, 0.8, 1.0),
        ..Default::default()
    });

    (world, cam)
}

pub fn quads() -> (HittableList, Camera) {
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
        background: color(0.7, 0.8, 1.0),
        ..Default::default()
    });

    (world, cam)
}

pub fn simple_light() -> (HittableList, Camera) {
    let mut world = HittableList::default();

    let noise = lambertian(noise_texture(4.0));
    world.add(sphere(point3(0.0, -1000.0, 0.0), 1000.0, noise.clone()));
    world.add(sphere(point3(0.0, 2.0, 0.0), 2.0, noise));

    let light = diffuse_light_from_color(color(4.0, 4.0, 4.0));
    world.add(sphere(point3(0.0, 7.0, 0.0), 2.0, light.clone()));
    world.add(quad(
        point3(3.0, 1.0, -2.0),
        vec3(2.0, 0.0, 0.0),
        vec3(0.0, 2.0, 0.0),
        light,
    ));

    let cam = Camera::new(CameraOptions {
        v_fov: 20.0,
        look_from: point3(26.0, 3.0, 6.0),
        look_at: point3(0.0, 2.0, 0.0),
        background: color(0.0, 0.0, 0.0),
        ..Default::default()
    });

    (world, cam)
}

pub fn cornell_box() -> (HittableList, Camera) {
    let mut world = HittableList::default();

    let red = lambertian_from_color(color(0.65, 0.05, 0.05));
    let white = lambertian_from_color(color(0.73, 0.73, 0.73));
    let green = lambertian_from_color(color(0.12, 0.45, 0.15));

    let light = diffuse_light_from_color(color(15.0, 15.0, 15.0));

    world.add(quad(
        point3(555.0, 0.0, 0.0),
        point3(0.0, 555.0, 0.0),
        vec3(0.0, 0.0, 555.0),
        green,
    ));
    world.add(quad(
        point3(0.0, 0.0, 0.0),
        point3(0.0, 555.0, 0.0),
        vec3(0.0, 0.0, 555.0),
        red,
    ));
    world.add(quad(
        point3(343.0, 554.0, 332.0),
        point3(-130.0, 0.0, 0.0),
        vec3(0.0, 0.0, -105.0),
        light,
    ));
    world.add(quad(
        point3(0.0, 0.0, 0.0),
        point3(555.0, 0.0, 0.0),
        vec3(0.0, 0.0, 555.0),
        white.clone(),
    ));
    world.add(quad(
        point3(555.0, 555.0, 555.0),
        point3(-555.0, 0.0, 0.0),
        vec3(0.0, 0.0, -555.0),
        white.clone(),
    ));
    world.add(quad(
        point3(0.0, 0.0, 555.0),
        point3(555.0, 0.0, 0.0),
        vec3(0.0, 555.0, 0.0),
        white.clone(),
    ));

    let mut box1: Arc<dyn Hittable> = cube(
        point3(0.0, 0.0, 0.0),
        point3(165.0, 330.0, 165.0),
        white.clone(),
    );
    box1 = rotate_y(box1, 15.0);
    box1 = translate(box1, vec3(265.0, 0.0, 295.0));
    world.add(box1);
    let mut box1: Arc<dyn Hittable> = cube(
        point3(0.0, 0.0, 0.0),
        point3(165.0, 165.0, 165.0),
        white.clone(),
    );
    box1 = rotate_y(box1, -18.0);
    box1 = translate(box1, vec3(130.0, 0.0, 65.0));
    world.add(box1);

    let cam = Camera::new(CameraOptions {
        aspect_ratio: 1.0,
        image_width: 600,
        v_fov: 40.0,
        samples_per_pixel: 200,
        look_from: point3(278.0, 278.0, -800.0),
        look_at: point3(278.0, 278.0, 0.0),
        background: color(0.0, 0.0, 0.0),
        ..Default::default()
    });

    (world, cam)
}

pub fn triangles() -> (HittableList, Camera) {
    let mut world = HittableList::default();

    let red = lambertian_from_color(color(0.65, 0.05, 0.05));

    world.add(triangle(
        point3(0.0, 0.0, 0.0),
        point3(1.0, 1.0, 0.0),
        point3(1.0, 0.0, 0.0),
        red.clone(),
    ));

    let cam = Camera::new(CameraOptions {
        aspect_ratio: 1.0,
        image_width: 400,
        v_fov: 40.0,
        samples_per_pixel: 10,
        look_from: point3(1.0, 0.0, -5.0),
        look_at: point3(0.0, 0.0, 0.0),
        background: color(1.0, 1.0, 1.0),
        ..Default::default()
    });

    (world, cam)
}

pub fn room() -> (BvhNode, Camera) {
    let (gltf, buffers, _) = gltf::import("src/res/room.gltf").unwrap();

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
        lambertian(image_texture_from_bytes(include_bytes!("./res/Wood_Floor_Light.png")).unwrap()),
        lambertian(image_texture_from_bytes(include_bytes!("./res/Wood_Floor.png")).unwrap()),
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
