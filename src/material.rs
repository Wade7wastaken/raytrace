use crate::{
    color::Color,
    hittable::HitRecord,
    ray::{ray, Ray},
    vec3::Vec3,
};

pub trait Material {
    /// Describes how a ray should be scattered given an input ray and the hit record of that ray
    /// Returns an option of the color attenuation and the output ray. None means the ray was absorbed
    /// Defaults to None
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();

        if scatter_direction.is_near_zero() {
            scatter_direction = rec.normal;
        }

        let scattered = ray(rec.p, scatter_direction);
        let attenuation = self.albedo;
        Some((attenuation, scattered))
    }
}

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
        let reflected_fuzzed = reflected.unit_vector() + (self.fuzz * Vec3::random_unit_vector());
        let scattered = ray(rec.p, reflected_fuzzed);
        let attenuation = self.albedo;

        // if we scatter below the surface, just absorb the ray
        if scattered.dir.dot(rec.normal) > 0.0 {
            Some((attenuation, scattered))
        } else {
            None
        }
    }
}
