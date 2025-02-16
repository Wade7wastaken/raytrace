// use gltf::Gltf;
// use raytrace::example_worlds::*;

use raytrace::primitives::Point3;

fn gltf() {
    let (gltf, buffers, _) = gltf::import("untitled.glb").unwrap();

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

        for triangle in indices.chunks(3) {
            if triangle.len() == 3 {
                let p1 = positions[triangle[0]];
                let p2 = positions[triangle[1]];
                let p3 = positions[triangle[2]];
                println!("Triangle: {}, {}, {}", p1, p2, p3);
            }
        }
    }
}

fn main() {
    // simple();
    // bouncing_spheres();
    // checkered_spheres();
    // earth();
    // perlin_spheres();
    // quads();
    // simple_light();
    // cornell_box();
    gltf();
}
