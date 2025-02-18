use std::{fmt::Display, sync::Arc};

use crate::{
    primitives::{ray, Color, Vec3},
    textures::{solid_color, Texture},
};

use super::Material;

pub struct Isotropic {
    tex: Arc<dyn Texture>,
}

impl Isotropic {
    pub fn new(tex: Arc<dyn Texture>) -> Self {
        Self { tex }
    }

    pub fn from_color(albedo: Color) -> Self {
        Self {
            tex: solid_color(albedo),
        }
    }
}

impl Material for Isotropic {
    fn scatter(
        &self,
        r: &crate::primitives::Ray,
        rec: &crate::hittables::HitRecord,
    ) -> Option<(Color, crate::primitives::Ray)> {
        let scattered = ray(rec.p, Vec3::random_unit_vector(), r.time);
        let attenuation = self.tex.value(rec.u, rec.v, rec.p);
        Some((attenuation, scattered))
    }
}

impl Display for Isotropic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Isotropic")
    }
}
