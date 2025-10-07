use rayon::prelude::*;
use std::{
    error::Error,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
};

use crate::{
    hittables::Hittable,
    image_writer::ImageWriter,
    misc::rand_f64,
    primitives::{Color, Point3, Ray, Vec3, color, interval, point3, ray, vec3},
};

fn opt_assert(cond: bool) -> Option<()> {
    cond.then_some(())
}

#[derive(Debug, Clone, Copy)]
pub struct CameraOptions {
    /// The aspect ratio of the output image.
    pub aspect_ratio: f64,
    /// The width of the output image.
    pub image_width: usize,
    /// The number of rays to sample per pixel.
    pub samples_per_pixel: u32,
    /// The maximum depth a ray is allowed to search.
    pub max_depth: u32,
    /// The field of view in degrees.
    pub v_fov: f64,
    /// The origin of the camera.
    pub look_from: Point3,
    /// The target of the camera.
    pub look_at: Point3,
    /// A vector representing the upwards direction.
    pub vup: Vec3,
    /// The angle of the defocus cone.
    pub defocus_angle: f64,
    /// The distance where objects are perfectly in focus.
    pub focus_dist: f64,
    /// The default color if a ray doesn't collide with anything.
    pub background: Color,
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
            defocus_angle: 0.0,
            focus_dist: 10.0,
            background: color(0.0, 0.0, 0.0),
        }
    }
}

pub struct Camera {
    image_height: usize,
    image_width: usize,
    samples_per_pixel: u32,
    max_depth: u32,
    look_from: Point3,
    pixel_00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    defocus_angle: f64,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
    background: Color,
}

impl Camera {
    #[must_use]
    pub fn new(options: CameraOptions) -> Option<Self> {
        let CameraOptions {
            aspect_ratio,
            image_width,
            samples_per_pixel,
            max_depth,
            v_fov,
            look_from,
            look_at,
            vup,
            defocus_angle,
            focus_dist,
            background,
        } = options;
        let image_height = (image_width as f64 / aspect_ratio).round() as usize;

        // ensure dimensions are greater than 0.
        opt_assert(aspect_ratio > 0.0)?;
        opt_assert(image_width > 0)?;
        opt_assert(image_height > 0)?;

        let theta = v_fov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focus_dist;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        let w = (look_from - look_at).unit_vector();
        let u = vup.cross(w);
        let v = w.cross(u);

        let viewport_u = u * viewport_width;
        let viewport_v = -v * viewport_height;

        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        let viewport_upper_left =
            look_from - (w * focus_dist) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel_00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

        let defocus_radius = focus_dist * (defocus_angle / 2.0).to_radians().tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Some(Self {
                    image_height,
                    image_width,
                    samples_per_pixel,
                    max_depth,
                    look_from,
                    pixel_00_loc,
                    pixel_delta_u,
                    pixel_delta_v,
                    defocus_angle,
                    defocus_disk_u,
                    defocus_disk_v,
                    background,
                })
    }

    // Renders a scanline into Vec of colors
    pub fn scanline(&self, world: &dyn Hittable, y: usize) -> Vec<Color> {
        (0..self.image_width)
            .map(|x| {
                (0..self.samples_per_pixel)
                    .map(|_| self.ray_color(&self.get_ray(x, y), self.max_depth, world))
                    .sum::<Color>()
                    / f64::from(self.samples_per_pixel)
            })
            .collect()
    }

    // Renders a hittable into a 2d array of colors
    pub fn render(&self, world: &dyn Hittable) -> Vec<Vec<Color>> {
        // AtomicUsize is faster than Mutex
        let count = Arc::new(AtomicUsize::new(0));

        (0..self.image_height)
            .into_par_iter()
            .map(|y| {
                let prev = count.fetch_add(1, Ordering::SeqCst);
                println!(
                    "starting {prev} / {} ({:.2}%)",
                    self.image_height,
                    prev as f64 / self.image_height as f64 * 100.0
                );
                self.scanline(world, y)
            })
            .collect()
    }

    // Renders a hittable and saves it to a given image_writer
    pub fn render_and_save(
        &self,
        world: &dyn Hittable,
        mut image_writer: impl ImageWriter,
    ) -> Result<(), Box<dyn Error>> {
        let pixels = self.render(world);
        println!("Done rendering");

        image_writer.write(pixels)?;
        println!("Done Saving");
        Ok(())
    }

    // Gets the final color of a ray through a given world. Recursively calls
    // itself for scattered rays
    fn ray_color(&self, r: &Ray, depth: u32, world: &dyn Hittable) -> Color {
        // if we hit the bounce limit, no more light is gathered
        if depth == 0 {
            return color(0.0, 0.0, 0.0);
        }

        if let Some(rec) = world.hit(r, &interval(0.001, f64::INFINITY)) {
            let mut color = rec.mat.emitted(rec.u, rec.v, rec.p);

            if let Some((attenuation, scattered)) = rec.mat.scatter(r, &rec) {
                let color_from_scatter = attenuation * self.ray_color(&scattered, depth - 1, world);
                color += color_from_scatter;
            }
            color
        } else {
            self.background
        }
    }

    fn get_ray(&self, x: usize, y: usize) -> Ray {
        let offset = sample_square();
        let pixel_sample = self.pixel_00_loc
            + (self.pixel_delta_u * (x as f64 + offset.x))
            + (self.pixel_delta_v * (y as f64 + offset.y));

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.look_from
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - ray_origin;

        ray(ray_origin, ray_direction, rand_f64())
    }

    fn defocus_disk_sample(&self) -> Point3 {
        let p = Vec3::random_in_unit_disk();
        self.look_from + (self.defocus_disk_u * p.x) + (self.defocus_disk_v * p.y)
    }
}

fn sample_square() -> Vec3 {
    vec3(rand_f64() - 0.5, rand_f64() - 0.5, 0.0)
}
