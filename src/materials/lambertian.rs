use std::{f64::consts::PI, fmt, sync::Arc};

use crate::{
    hittables::HitRecord, misc::random_cosine_direction, primitives::{ray, Color, Onb, Ray, Vec3}, tern, textures::{SolidColor, Texture}
};

use super::Material;

#[derive(Clone)]
pub struct Lambertian {
    tex: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn new(tex: Arc<dyn Texture>) -> Self {
        Self { tex }
    }
    #[must_use]
    pub fn from_color(albedo: Color) -> Self {
        Self {
            tex: Arc::new(SolidColor::new(albedo)),
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<(Color, Ray, f64)> {
        let uvw = Onb::new(rec.normal);
        let scatter_direction = uvw.transform(random_cosine_direction());

        let scattered = ray(rec.p, scatter_direction.unit_vector(), r.time);
        let attenuation = self.tex.value(rec.u, rec.v, rec.p);
        let pdf = uvw.w().dot(scattered.dir) / PI;
        Some((attenuation, scattered, pdf))
    }

    fn scattering_pdf(&self, _r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> f64 {
        let cos_theta = rec.normal.dot(scattered.dir.unit_vector());
        tern!(cos_theta < 0.0, 0.0, cos_theta / PI)
    }
}

impl fmt::Display for Lambertian {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "lambertian{}", self.tex)
    }
}

pub fn lambertian(tex: Arc<dyn Texture>) -> Arc<Lambertian> {
    Arc::new(Lambertian::new(tex))
}

#[must_use]
pub fn lambertian_from_color(albedo: Color) -> Arc<Lambertian> {
    Arc::new(Lambertian::from_color(albedo))
}
