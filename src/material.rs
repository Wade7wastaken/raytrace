use crate::{
    hittables::HitRecord,
    primitives::{Color, Ray, Vec3, color, ray},
};

#[derive(Debug, Clone)]
pub enum Material {
    Lambertian { albedo: Color },
    DiffuseLight { emit: Color },
}

fn lambertian_scatter(albedo: Color, rec: &HitRecord) -> (Color, Ray) {
    let mut scatter_direction = rec.normal + Vec3::random_unit_vector();

    if scatter_direction.is_near_zero() {
        scatter_direction = rec.normal;
    }

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
