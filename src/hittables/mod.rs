mod bvh_node;
mod constant_medium;
mod hittable;
mod hittable_list;
mod quad;
mod rotate_y;
mod sphere;
mod translate;
mod triangle;

pub use bvh_node::*;
pub use constant_medium::*;
pub use hittable::*;
pub use hittable_list::*;
pub use quad::*;
pub use rotate_y::*;
pub use sphere::*;
pub use translate::*;
pub use triangle::*;

use std::sync::Arc;

use crate::{
    materials::Material,
    primitives::{point3, vec3, Point3},
};

pub fn cube(a: Point3, b: Point3, mat: Arc<dyn Material>) -> Arc<HittableList> {
    let mut sides = HittableList::default();

    let min = point3(a.x.min(b.x), a.y.min(b.y), a.z.min(b.z));
    let max = point3(a.x.max(b.x), a.y.max(b.y), a.z.max(b.z));

    let dx = vec3(max.x - min.x, 0.0, 0.0);
    let dy = vec3(0.0, max.y - min.y, 0.0);
    let dz = vec3(0.0, 0.0, max.z - min.z);

    sides.add(quad(point3(min.x, min.y, max.z), dx, dy, mat.clone())); // front
    sides.add(quad(point3(max.x, min.y, max.z), -dz, dy, mat.clone())); // right
    sides.add(quad(point3(max.x, min.y, min.z), -dx, dy, mat.clone())); // back
    sides.add(quad(point3(min.x, min.y, min.z), dz, dy, mat.clone())); // left
    sides.add(quad(point3(min.x, max.y, max.z), dx, -dz, mat.clone())); // top
    sides.add(quad(point3(min.x, min.y, min.z), dx, dz, mat)); // bottom

    Arc::new(sides)
}
