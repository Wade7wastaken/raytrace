use crate::{
    color::{color, Color},
    hittable::HitRecord,
    rand::rand,
    ray::{ray, Ray},
    vec3::Vec3,
};

pub trait Material: Send + Sync {
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
    fn scatter(&self, _r: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
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

pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }

    fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let attenuation = color(1.0, 1.0, 1.0);
        let refraction_index = if rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = r.dir.unit_vector();

        let cos_theta = (-unit_direction).dot(rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_index * sin_theta > 1.0;

        let direction =
            if cannot_refract || Dielectric::reflectance(cos_theta, refraction_index) > rand() {
                unit_direction.reflect(rec.normal)
            } else {
                unit_direction.refract(rec.normal, refraction_index)
            };

        let scattered = ray(rec.p, direction);
        Some((attenuation, scattered))
    }
}
