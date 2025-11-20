use std::fmt;

use crate::{
    hittables::HitRecord,
    primitives::{Color, Point3, Ray, color},
};

pub trait Material: Send + Sync + fmt::Display {
    /// Describes how a ray should be scattered given an input ray and the hit record of that ray
    /// Returns an option of the color attenuation and the output ray. None means the ray was absorbed
    /// Defaults to None
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<(Color, Ray, f64)>;

    fn emitted(&self, r: &Ray, rec: &HitRecord, u: f64, v: f64, p: Point3) -> Color {
        let _ = (r, rec, u, v, p);
        color(0.0, 0.0, 0.0)
    }

    fn scattering_pdf(&self, r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> f64 {
        let _ = (r_in, rec, scattered);
        0.0
    }
}
