use std::sync::Arc;

use crate::{
    hittables::HitRecord,
    primitives::{Color, Ray, Vec3, ray},
};

use super::Material;

#[derive(Clone)]
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    #[must_use]
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();

        if scatter_direction.is_near_zero() {
            scatter_direction = rec.normal;
        }

        let scattered = ray(rec.p, scatter_direction, r.time);
        let attenuation = self.albedo;
        Some((attenuation, scattered))
    }
}

#[must_use]
pub fn lambertian(albedo: Color) -> Arc<Lambertian> {
    Arc::new(Lambertian::new(albedo))
}
