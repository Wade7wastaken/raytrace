use std::f32::consts::PI;

use crate::{
    geometry::HitRecord,
    misc::rand_f32,
    primitives::{Color, Ray, Vec3, ray, vec3},
    tern,
};

#[derive(Debug, Clone)]
pub struct Material {
    pub color: Color,
    pub is_light: bool,
}

#[must_use]
#[inline]
fn random_cosine_direction() -> Vec3 {
    let r1 = rand_f32();
    let r2 = rand_f32();

    let phi = 2.0 * PI * r1;
    let (sin, cos) = phi.sin_cos();
    let sqrt = r2.sqrt();
    vec3(cos * sqrt, sin * sqrt, (1.0 - r2).sqrt()).unit_vector()
}

#[must_use]
#[inline]
fn onb_transform(n: Vec3, orig: Vec3) -> Vec3 {
    let w = n.unit_vector();
    let a = tern!(w.x.abs() > 0.9, vec3(0.0, 1.0, 0.0), vec3(1.0, 0.0, 0.0));
    let v = w.cross(a).unit_vector();
    let u = w.cross(v);

    (u * orig.x) + (v * orig.y) + (w * orig.z)
}

pub fn lambertian_scatter2(rec: &HitRecord) -> Ray {
    let scatter_direction = onb_transform(rec.normal, random_cosine_direction());

    ray(rec.p, scatter_direction)
}

#[must_use]
#[inline]
pub const fn lambertian(albedo: Color) -> Material {
    Material {
        color: albedo,
        is_light: false,
    }
}

#[must_use]
#[inline]
pub const fn diffuse_light(emit: Color) -> Material {
    Material {
        color: emit,
        is_light: true,
    }
}
