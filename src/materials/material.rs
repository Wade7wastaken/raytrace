use std::fmt;

use crate::{
    hittables::HitRecord,
    primitives::{Color, Point3, Ray, color},
};

pub trait Material: Send + Sync + fmt::Display {
    /// Describes how a ray should be scattered given an input ray and the hit record of that ray
    /// Returns an option of the color attenuation and the output ray. None means the ray was absorbed
    /// Defaults to None
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;

    fn emitted(&self, u: f64, v: f64, p: Point3) -> Color {
        let _ = (u, v, p);
        color(0.0, 0.0, 0.0)
    }
}
