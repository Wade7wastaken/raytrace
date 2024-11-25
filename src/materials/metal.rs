use std::{fmt, sync::Arc};

use crate::{
    hittables::HitRecord,
    primitives::{ray, Color, Ray, Vec3},
};

use super::Material;

#[derive(Clone)]
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = r.dir.reflect(rec.normal);
        let reflected_fuzzed = reflected.unit_vector() + (Vec3::random_unit_vector() * self.fuzz);
        let scattered = ray(rec.p, reflected_fuzzed, r.time);
        let attenuation = self.albedo.to_owned();

        // if we scatter below the surface, just absorb the ray
        if scattered.dir.dot(rec.normal) > 0.0 {
            Some((attenuation, scattered))
        } else {
            None
        }
    }
}

impl fmt::Display for Metal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "metal({}, {})", self.albedo, self.fuzz)
    }
}

pub fn metal(albedo: Color, fuzz: f64) -> Arc<Metal> {
    Arc::new(Metal::new(albedo, fuzz))
}