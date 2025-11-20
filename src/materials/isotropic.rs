use std::{
    f64::consts::PI,
    fmt::{self, Display},
    sync::Arc,
};

use crate::{
    hittables::HitRecord,
    primitives::{Color, Ray, Vec3, ray},
    textures::{Texture, solid_color},
};

use super::Material;

pub struct Isotropic {
    tex: Arc<dyn Texture>,
}

impl Isotropic {
    pub fn new(tex: Arc<dyn Texture>) -> Self {
        Self { tex }
    }

    #[must_use]
    pub fn from_color(albedo: Color) -> Self {
        Self {
            tex: solid_color(albedo),
        }
    }
}

impl Material for Isotropic {
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<(Color, Ray, f64)> {
        let scattered = ray(rec.p, Vec3::random_unit_vector(), r.time);
        let attenuation = self.tex.value(rec.u, rec.v, rec.p);
        Some((attenuation, scattered, 1.0 / (4.0 * PI)))
    }

    fn scattering_pdf(&self, _r_in: &Ray, _rec: &HitRecord, _scattered: &Ray) -> f64 {
        1.0 / (4.0 * PI)
    }
}

impl Display for Isotropic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Isotropic({})", self.tex)
    }
}
