use crate::{color::Color, hittable::HitRecord, ray::Ray};

pub trait Material {
    /// Describes how a ray should be scattered given an input ray and the hit record of that ray
    /// Returns an option of the color attenuation and the output ray. None means the ray was absorbed
    /// Defaults to None
    fn scatter(&self, r: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)>;
}
