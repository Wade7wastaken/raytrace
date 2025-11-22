use std::sync::Arc;

use crate::{
    hittables::HitRecord,
    primitives::{Color, Point3, Ray},
};

use super::Material;

pub struct DiffuseLight {
    color: Color,
}

impl DiffuseLight {
    fn new_from_color(emit: Color) -> Self {
        Self { color: emit }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, _r: &Ray, _rec: &HitRecord) -> Option<(Color, Ray)> {
        None
    }

    fn emitted(&self, _u: f64, _v: f64, _p: Point3) -> Color {
        self.color
    }
}

#[must_use]
pub fn diffuse_light(color: Color) -> Arc<DiffuseLight> {
    Arc::new(DiffuseLight::new_from_color(color))
}
