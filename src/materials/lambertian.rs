use std::{fmt, sync::Arc};

use crate::{
    hittables::HitRecord,
    primitives::{ray, Color, Ray, Vec3},
    textures::{SolidColor, Texture},
};

use super::Material;

#[derive(Clone)]
pub struct Lambertian {
    tex: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn new(tex: Arc<dyn Texture>) -> Self {
        Self { tex }
    }
    pub fn from_color(albedo: Color) -> Self {
        Self {
            tex: Arc::new(SolidColor::new(albedo)),
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();

        if scatter_direction.is_near_zero() {
            scatter_direction = rec.normal;
        }

        let scattered = ray(rec.p, scatter_direction, r.time);
        let attenuation = self.tex.value(rec.u, rec.v, rec.p);
        Some((attenuation, scattered))
    }
}

impl fmt::Display for Lambertian {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "lambertian{}", self.tex)
    }
}

pub fn lambertian(tex: Arc<dyn Texture>) -> Arc<Lambertian> {
    Arc::new(Lambertian::new(tex))
}

pub fn lambertian_from_color(albedo: Color) -> Arc<Lambertian> {
    Arc::new(Lambertian::from_color(albedo))
}