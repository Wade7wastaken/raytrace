use rayon::prelude::*;
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

use crate::{
    hittables::Hittable,
    misc::rand_f64,
    primitives::{Color, Point3, Ray, Vec3, color, interval, point3, ray, vec3},
};

#[derive(Debug, Clone, Copy)]
pub struct CameraOptions {
    /// The aspect ratio of the output image.
    pub aspect_ratio: f64,
    /// The width of the output image.
    pub image_width: usize,
    /// The number of rays to sample per pixel.
    pub samples_per_pixel: u32,
    /// The maximum depth a ray is allowed to search.
    pub max_depth: usize,
    /// The field of view in degrees.
    pub v_fov: f64,
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
            look_from: Point3::default(),
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
        let image_height = (image_width as f64 / aspect_ratio).round() as usize;

        // ensure dimensions are greater than 0.
        assert!(aspect_ratio > 0.0);
        assert!(image_width > 0);
        assert!(image_height > 0);

        let focal_length = (look_from - look_at).length();
        let theta = v_fov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focal_length;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        let w = (look_from - look_at).unit_vector();
        let u = vup.cross(w);
        let v = w.cross(u);

        let viewport_u = u * viewport_width;
        let viewport_v = -v * viewport_height;

        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

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

    pub fn render(&self, world: &dyn Hittable) -> Vec<Vec<Color>> {
        let count = Arc::new(AtomicUsize::new(0));

        let mut result = vec![];

        (0..self.image_height)
            .into_par_iter()
            .map(|y| {
                let prev = count.fetch_add(1, Ordering::Relaxed);
                println!(
                    "starting {prev} / {} ({:.2}%)",
                    self.image_height,
                    prev as f64 / self.image_height as f64 * 100.0
                );
                self.scanline(world, y)
            })
            .collect_into_vec(&mut result);

        result
    }

    pub fn scanline(&self, world: &dyn Hittable, y: usize) -> Vec<Color> {
        let mut emitted_values = Vec::with_capacity(self.max_depth);
        let mut attenuation_values = Vec::with_capacity(self.max_depth);

        (0..self.image_width)
            .map(|x| {
                (0..self.samples_per_pixel)
                    .map(|_| {
                        self.ray_color(
                            self.get_ray(x, y),
                            world,
                            &mut emitted_values,
                            &mut attenuation_values,
                        )
                    })
                    .sum::<Color>()
                    / f64::from(self.samples_per_pixel)
            })
            .collect()
    }

    fn ray_color(
        &self,
        r: Ray,
        world: &dyn Hittable,
        emitted_values: &mut Vec<Color>,
        attenuation_values: &mut Vec<Color>,
    ) -> Color {
        let ending_color = self.bounce_ray(r, world, emitted_values, attenuation_values);

        emitted_values
            .iter()
            .zip(attenuation_values)
            .fold(ending_color, |prev, (emitted, attenuation)| {
                (prev * *attenuation) + *emitted
            })
    }


    fn bounce_ray(
        &self,
        mut r: Ray,
        world: &dyn Hittable,
        emitted_values: &mut Vec<Color>,
        attenuation_values: &mut Vec<Color>,
    ) -> Color {
        attenuation_values.clear();
        emitted_values.clear();
        for _ in 0..self.max_depth {
            if let Some(rec) = world.hit(&r, &interval(0.001, f64::INFINITY)) {
                let emitted = rec.mat.emitted();

                if let Some((attenuation, scattered)) = rec.mat.scatter(&rec) {
                    attenuation_values.push(attenuation);
                    emitted_values.push(emitted);
                    r = scattered;
                } else {
                    return emitted;
                }
            } else {
                return color(0.0, 0.0, 0.0);
            }
        }
        color(0.0, 0.0, 0.0)
    }

    fn get_ray(&self, x: usize, y: usize) -> Ray {
        let offset = sample_square();
        let pixel_sample = self.pixel_00_loc
            + (self.pixel_delta_u * (x as f64 + offset.x))
            + (self.pixel_delta_v * (y as f64 + offset.y));

        let ray_direction = pixel_sample - self.look_from;

        ray(self.look_from, ray_direction)
    }
}

fn sample_square() -> Vec3 {
    vec3(rand_f64() - 0.5, rand_f64() - 0.5, 0.0)
}
