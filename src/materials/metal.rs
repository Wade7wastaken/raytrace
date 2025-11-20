use std::{fmt, sync::Arc};

use crate::{
    hittables::HitRecord,
    primitives::{Color, Ray, Vec3, ray},
};

use super::Material;

#[derive(Clone)]
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    #[must_use]
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<(Color, Ray, f64)> {
        let reflected = r.dir.reflect(rec.normal);
        let reflected_fuzzed = reflected.unit_vector() + (Vec3::random_unit_vector() * self.fuzz);
        let scattered = ray(rec.p, reflected_fuzzed, r.time);
        let attenuation = self.albedo;

        // if we scatter below the surface, just absorb the ray
        (scattered.dir.dot(rec.normal) > 0.0).then_some((attenuation, scattered, 0.0))
    }
}

impl fmt::Display for Metal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "metal({}, {})", self.albedo, self.fuzz)
    }
}

#[must_use]
pub fn metal(albedo: Color, fuzz: f64) -> Arc<Metal> {
    Arc::new(Metal::new(albedo, fuzz))
}
