use std::fmt;

use crate::{
    hittables::HitRecord,
    primitives::{Color, Ray},
};

pub trait Material: Send + Sync + fmt::Display {
    /// Describes how a ray should be scattered given an input ray and the hit record of that ray
    /// Returns an option of the color attenuation and the output ray. None means the ray was absorbed
    /// Defaults to None
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
}
