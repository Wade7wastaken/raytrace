use crate::{
    color::{color, Color},
    hittable::Hittable,
    image_writer::ImageWriter,
    interval::interval,
    rand::rand,
    ray::{ray, Ray},
    vec3::{point3, vec3, Point3, Vec3},
};
use rayon::prelude::*;
use std::error::Error;

pub struct CameraOptions {
    pub aspect_ratio: f64,
    pub image_width: u32,
    pub samples_per_pixel: u32,
    pub max_depth: u32,
    pub v_fov: f64,
    pub look_from: Point3,
    pub look_at: Point3,
    pub vup: Vec3,
    pub defocus_angle: f64,
    pub focus_dist: f64,
}

impl Default for CameraOptions {
    fn default() -> Self {
        Self {
            aspect_ratio: 16.0 / 9.0,
            image_width: 400,
            samples_per_pixel: 10,
            max_depth: 10,
            v_fov: 90.0,
            look_from: Point3::default(),
            look_at: point3(0.0, 0.0, -1.0),
            vup: vec3(0.0, 1.0, 0.0),
            defocus_angle: 0.0,
            focus_dist: 10.0,
        }
    }
}

pub struct Camera {
    // aspect_ratio: f64,
    pub image_height: u32, // public so they can be accessed by image writers
    pub image_width: u32,
    // focal_length: f64,
    samples_per_pixel: u32,
    max_depth: u32,
    // viewport_width: f64,
    // viewport_height: f64,
    look_from: Point3,
    pixel_00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    defocus_angle: f64,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Camera {
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
            defocus_angle,
            focus_dist,
        } = options;
        let image_height = (image_width as f64 / aspect_ratio) as u32;

        // ensure dimensions are more than 0
        assert!(image_width > 0);
        assert!(image_height > 0);

        // let focal_length = (look_from - look_at).length();
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

        Self {
            // aspect_ratio,
            image_height,
            image_width,
            // focal_length,
            samples_per_pixel,
            max_depth,
            // viewport_width,
            // viewport_height,
            look_from,
            pixel_00_loc,
            pixel_delta_u,
            pixel_delta_v,
            defocus_angle,
            defocus_disk_u,
            defocus_disk_v,
        }
    }
    pub fn render(&self, world: &dyn Hittable) -> Vec<Vec<Color>> {
        (0..self.image_height)
            .into_par_iter()
            .map(|y| {
                println!("scanline {}", y);
                (0..self.image_width)
                    .into_par_iter()
                    .map(|x| {
                        (0..self.samples_per_pixel)
                            .into_par_iter()
                            .map(|_| ray_color(&self.get_ray(x, y), self.max_depth, world))
                            .sum::<Color>()
                            / self.samples_per_pixel as f64
                    })
                    .collect()
            })
            .collect()
    }

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

    fn get_ray(&self, x: u32, y: u32) -> Ray {
        let offset = Camera::sample_square();
        let pixel_sample = self.pixel_00_loc
            + (self.pixel_delta_u * (x as f64 + offset.x))
            + (self.pixel_delta_v * (y as f64 + offset.y));

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.look_from
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - ray_origin;

        ray(ray_origin, ray_direction, rand())
    }

    fn sample_square() -> Vec3 {
        vec3(rand() - 0.5, rand() - 0.5, 0.0)
    }

    fn defocus_disk_sample(&self) -> Point3 {
        let p = Vec3::random_in_unit_disk();
        self.look_from + (self.defocus_disk_u * p.x) + (self.defocus_disk_v * p.y)
    }
}

fn ray_color(r: &Ray, depth: u32, world: &dyn Hittable) -> Color {
    // if we hit the bounce limit, no more light it gathered
    if depth == 0 {
        return Color::default();
    }

    if let Some(rec) = world.hit(r, interval(0.001, f64::INFINITY)) {
        if let Some((attenuation, scattered)) = rec.mat.scatter(r, &rec) {
            attenuation * ray_color(&scattered, depth - 1, world)
        } else {
            Color::default()
        }
    } else {
        sky(r)
    }
}

fn sky(r: &Ray) -> Color {
    let unit_direction = r.dir.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * color(1.0, 1.0, 1.0) + t * color(0.5, 0.7, 1.0)
}
