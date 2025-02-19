use std::{fmt::Display, sync::Arc};

use crate::{
    materials::{Isotropic, Material},
    primitives::{interval, vec3, Color, Interval, Ray},
    rand::rand,
    textures::Texture,
};

use super::{HitRecord, Hittable};

pub struct ConstantMedium {
    boundary: Arc<dyn Hittable>,
    neg_inv_density: f64,
    phase_function: Arc<dyn Material>,
}

impl ConstantMedium {
    pub fn new(boundary: Arc<dyn Hittable>, density: f64, tex: Arc<dyn Texture>) -> Self {
        Self {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function: Arc::new(Isotropic::new(tex)),
        }
    }
    pub fn from_color(boundary: Arc<dyn Hittable>, density: f64, albedo: Color) -> Self {
        Self {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function: Arc::new(Isotropic::from_color(albedo)),
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        // if we never hit the boundary, return
        let mut rec1 = self.boundary.hit(r, &Interval::full())?;

        // find the exit point of the ray
        let mut rec2 = self
            .boundary
            .hit(r, &interval(rec1.t + 0.0001, f64::INFINITY))?;

        if rec1.t < ray_t.min {
            rec1.t = ray_t.min;
        }
        if rec2.t > ray_t.max {
            rec2.t = ray_t.max;
        }

        if rec1.t >= rec2.t {
            return None;
        }

        if rec1.t < 0.0 {
            rec1.t = 0.0;
        }

        let ray_length = r.dir.length();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * rand().ln();

        if hit_distance > distance_inside_boundary {
            return None;
        }

        let t = rec1.t + hit_distance / ray_length;
        let p = r.at(t);
        let outward_normal = vec3(1.0, 0.0, 0.0); // arbitrary
        let mat = self.phase_function.clone();

        let u = 0.0;
        let v = 0.0;

        Some(HitRecord::new(p, mat, t, u, v, r, outward_normal))
    }

    fn bounding_box(&self) -> &crate::primitives::Aabb {
        self.boundary.bounding_box()
    }
}

impl Display for ConstantMedium {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "constant_medium({}, {}, {})",
            self.boundary,
            -1.0 / self.neg_inv_density,
            self.phase_function
        )
    }
}

pub fn constant_medium_from_color(
    boundary: Arc<dyn Hittable>,
    density: f64,
    albedo: Color,
) -> Arc<ConstantMedium> {
    Arc::new(ConstantMedium::from_color(boundary, density, albedo))
}
