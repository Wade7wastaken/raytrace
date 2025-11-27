use std::f64::consts::PI;

use crate::{
    hittables::HitRecord,
    misc::rand_f64,
    primitives::{Color, Ray, Vec3, color, ray, vec3},
    tern,
};

#[derive(Debug, Clone)]
pub enum Material {
    Lambertian { albedo: Color },
    DiffuseLight { emit: Color },
}

fn random_cosine_direction() -> Vec3 {
    let r1 = rand_f64();
    let r2 = rand_f64();

    let phi = 2.0 * PI * r1;
    let (sin, cos) = phi.sin_cos();
    let sqrt = r2.sqrt();
    vec3(cos * sqrt, sin * sqrt, (1.0 - r2).sqrt()).unit_vector()
}

fn onb_transform(n: Vec3, orig: Vec3) -> Vec3 {
    let w = n.unit_vector();
    let a = tern!(w.x.abs() > 0.9, vec3(0.0, 1.0, 0.0), vec3(1.0, 0.0, 0.0));
    let v = w.cross(a).unit_vector();
    let u = w.cross(v);

    (u * orig.x) + (v * orig.y) + (w * orig.z)
}

fn lambertian_scatter(albedo: Color, rec: &HitRecord) -> (Color, Ray) {
    let scatter_direction = onb_transform(rec.normal, random_cosine_direction());
    // let mut scatter_direction = rec.normal + Vec3::random_unit_vector();

    // if scatter_direction.is_near_zero() {
    //     scatter_direction = rec.normal;
    // }

    let scattered = ray(rec.p, scatter_direction);
    let attenuation = albedo;
    (attenuation, scattered)
}

impl Material {
    pub fn scatter(&self, rec: &HitRecord) -> Option<(Color, Ray)> {
        match self {
            Self::Lambertian { albedo } => Some(lambertian_scatter(*albedo, rec)),
            Self::DiffuseLight { emit: _ } => None,
        }
    }

    pub fn emitted(&self) -> Color {
        match self {
            Self::Lambertian { albedo: _ } => color(0.0, 0.0, 0.0),
            Self::DiffuseLight { emit } => *emit,
        }
    }
}

pub fn lambertian(albedo: Color) -> Material {
    Material::Lambertian { albedo }
}

pub fn diffuse_light(emit: Color) -> Material {
    Material::DiffuseLight { emit }
}
