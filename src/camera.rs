use rayon::prelude::*;

use crate::{
    geometry::{HitRecord, Quad},
    material::lambertian_scatter2,
    misc::rand_f32,
    primitives::{Color, Point3, Ray, Vec3, color, interval, point3, ray, vec3},
};

#[derive(Debug, Clone, Copy)]
pub struct CameraOptions {
    /// The aspect ratio of the output image.
    pub aspect_ratio: f32,
    /// The width of the output image.
    pub image_width: usize,
    /// The number of rays to sample per pixel.
    pub samples_per_pixel: u32,
    /// The maximum depth a ray is allowed to search.
    pub max_depth: usize,
    /// The field of view in degrees.
    pub v_fov: f32,
    /// The origin of the camera.
    pub look_from: Point3,
    /// The target of the camera.
    pub look_at: Point3,
    /// A vector representing the upwards direction.
    pub vup: Vec3,
}

impl Default for CameraOptions {
    fn default() -> Self {
        Self {
            aspect_ratio: 16.0 / 9.0,
            image_width: 400,
            samples_per_pixel: 100,
            max_depth: 50,
            v_fov: 90.0,
            look_from: point3(0.0, 0.0, 0.0),
            look_at: point3(0.0, 0.0, -1.0),
            vup: vec3(0.0, 1.0, 0.0),
        }
    }
}

pub struct Camera {
    pub image_height: usize,
    pub image_width: usize,
    samples_per_pixel: u32,
    max_depth: usize,
    look_from: Point3,
    pixel_00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    #[must_use]
    pub fn new(options: CameraOptions) -> Self {
        let CameraOptions {
            aspect_ratio,
            image_width,
            samples_per_pixel,
            max_depth,
            v_fov,
            look_from,
            look_at,
            vup,
        } = options;
        let image_height = (image_width as f32 / aspect_ratio).round() as usize;

        // ensure dimensions are greater than 0.
        assert!(aspect_ratio > 0.0);
        assert!(image_width > 0);
        assert!(image_height > 0);

        let focal_length = (look_from - look_at).length();
        let theta = v_fov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focal_length;
        let viewport_width = viewport_height * (image_width as f32 / image_height as f32);

        let w = (look_from - look_at).unit_vector();
        let u = vup.cross(w);
        let v = w.cross(u);

        let viewport_u = u * viewport_width;
        let viewport_v = -v * viewport_height;

        let pixel_delta_u = viewport_u / image_width as f32;
        let pixel_delta_v = viewport_v / image_height as f32;

        let viewport_upper_left =
            look_from - (w * focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel_00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

        Self {
            image_height,
            image_width,
            samples_per_pixel,
            max_depth,
            look_from,
            pixel_00_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    #[must_use]
    #[inline]
    pub fn render(&self, world: &[Quad]) -> Vec<Vec<Color>> {
        let mut result = vec![];

        (0..self.image_height)
            .into_par_iter()
            .map(|y| self.scanline(world, y))
            .collect_into_vec(&mut result);

        result
    }

    #[must_use]
    #[inline]
    pub fn scanline(&self, world: &[Quad], y: usize) -> Vec<Color> {
        (0..self.image_width)
            .map(|x| {
                (0..self.samples_per_pixel)
                    .map(|_| self.ray_color(self.get_ray(x, y), world))
                    .sum::<Color>()
                    / (self.samples_per_pixel as f32)
            })
            .collect()
    }

    #[must_use]
    #[inline]
    fn ray_color(&self, mut r: Ray, world: &[Quad]) -> Color {
        let mut ray_c = color(1.0, 1.0, 1.0);

        for _ in 0..self.max_depth {
            let world_hit = world.iter().fold(None, |rec, object| {
                let max = rec.as_ref().map_or(f32::INFINITY, |r: &HitRecord| r.t);
                object.hit(&r, &interval(0.001, max)).or(rec)
            });

            let Some(rec) = world_hit else {
                return color(0.0, 0.0, 0.0);
            };

            let mat = rec.mat;

            if mat.is_light {
                return ray_c * mat.color.clone();
            }

            ray_c = ray_c * mat.color.clone();
            r = lambertian_scatter2(&rec);
        }
        color(0.0, 0.0, 0.0)
    }

    #[must_use]
    #[inline]
    fn get_ray(&self, x: usize, y: usize) -> Ray {
        let offset = sample_square();
        let pixel_sample = self.pixel_00_loc
            + (self.pixel_delta_u * (x as f32 + offset.x))
            + (self.pixel_delta_v * (y as f32 + offset.y));

        let ray_direction = pixel_sample - self.look_from;

        ray(self.look_from, ray_direction)
    }
}

#[must_use]
#[inline]
fn sample_square() -> Vec3 {
    vec3(rand_f32() - 0.5, rand_f32() - 0.5, 0.0)
}
