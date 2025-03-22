use std::{
    fmt::{self, Display},
    sync::Arc,
};

use crate::{
    hittables::HitRecord,
    primitives::{Color, Point3, Ray},
    textures::{Texture, solid_color},
};

use super::Material;

pub struct DiffuseLight {
    tex: Arc<dyn Texture>,
}

impl DiffuseLight {
    fn new(tex: Arc<dyn Texture>) -> Self {
        Self { tex }
    }

    fn new_from_color(emit: Color) -> Self {
        Self {
            tex: solid_color(emit),
        }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, _r: &Ray, _rec: &HitRecord) -> Option<(Color, Ray)> {
        None
    }

    fn emitted(&self, u: f64, v: f64, p: Point3) -> Color {
        self.tex.value(u, v, p)
    }
}

impl Display for DiffuseLight {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "diffuse light({})", self.tex)
    }
}

pub fn diffuse_light(tex: Arc<dyn Texture>) -> Arc<DiffuseLight> {
    Arc::new(DiffuseLight::new(tex))
}

#[must_use]
pub fn diffuse_light_from_color(color: Color) -> Arc<DiffuseLight> {
    Arc::new(DiffuseLight::new_from_color(color))
}
