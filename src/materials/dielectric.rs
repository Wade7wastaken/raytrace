use std::{fmt, sync::Arc};

use crate::{
    hittables::HitRecord,
    misc::rand_f64,
    primitives::{Color, Ray, color, ray},
    tern,
};

use super::Material;

#[derive(Clone)]
pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    #[must_use]
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }

    fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        let r0 = ((1.0 - refraction_index) / (1.0 + refraction_index)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let attenuation = color(1.0, 1.0, 1.0);
        let refraction_index = tern!(
            rec.front_face,
            1.0 / self.refraction_index,
            self.refraction_index
        );

        let unit_direction = r.dir.unit_vector();

        let cos_theta = (-unit_direction).dot(rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_index * sin_theta > 1.0;

        let direction = tern!(
            cannot_refract || Dielectric::reflectance(cos_theta, refraction_index) > rand_f64(),
            unit_direction.reflect(rec.normal),
            unit_direction.refract(rec.normal, refraction_index)
        );

        let scattered = ray(rec.p, direction, r.time);
        Some((attenuation, scattered))
    }
}

impl fmt::Display for Dielectric {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "dielectric({})", self.refraction_index)
    }
}

#[must_use]
pub fn dielectric(refraction_index: f64) -> Arc<Dielectric> {
    Arc::new(Dielectric::new(refraction_index))
}
