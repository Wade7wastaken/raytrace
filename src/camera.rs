use crate::{
    hittable::Hittable,
    hittable_list::HittableList,
    interval::Interval,
    ppm_image_writer::PPMImageWriter,
    rand::rand,
    ray::Ray,
    vec3::{Color, Point3, Vec3},
};
use std::f64::INFINITY;
use std::io::{self, Write};

pub struct CameraOptions {
    pub aspect_ratio: f64,
    pub image_width: u32,
    pub focal_length: f64,
    pub samples_per_pixel: u32,
    pub viewport_height: f64,
    pub camera_center: Point3,
}

impl Default for CameraOptions {
    fn default() -> Self {
        Self {
            aspect_ratio: 16.0 / 9.0,
            image_width: 400,
            focal_length: 1.0,
            samples_per_pixel: 10,
            viewport_height: 2.0,
            camera_center: Vec3::empty(),
        }
    }
}

pub struct Camera {
    pub image_writer: PPMImageWriter,
    pub aspect_ratio: f64,
    image_height: u32,
    pub image_width: u32,
    pub focal_length: f64,
    pub samples_per_pixel: u32,
    viewport_width: f64,
    pub viewport_height: f64,
    pub camera_center: Point3,
    pixel_00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn new(
        image_writer: PPMImageWriter,
        options: CameraOptions,
    ) -> Self {
        let CameraOptions { aspect_ratio, image_width, focal_length, samples_per_pixel, viewport_height, camera_center } = options;
        let image_height = (image_width as f64 / aspect_ratio) as u32;

        // ensure dimensions are more than 0
        assert!(image_width > 0);
        assert!(image_height > 0);

        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        let viewport_upper_left =
            camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;

        let pixel_00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Self {
            image_writer,
            aspect_ratio,
            image_height,
            image_width,
            focal_length,
            samples_per_pixel,
            viewport_width,
            viewport_height,
            camera_center,
            pixel_00_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
    }
    pub fn render(&mut self, world: HittableList) {
        self.image_writer
            .write_header(self.image_width, self.image_height);

        for y in 0..self.image_height {
            print!("\rScanlines remaining: {}", self.image_height - y);
            io::stdout().flush().unwrap();
            for x in 0..self.image_width {
                let mut pixel_color = Color::empty();
                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(x, y);
                    pixel_color += Camera::ray_color(&r, &world);
                }

                self.image_writer
                    .write_pixel(pixel_color / self.samples_per_pixel as f64);
            }
        }
        println!("\nDone!");
    }

    fn get_ray(&self, x: u32, y: u32) -> Ray {
        let offset = Camera::sample_square();
        let pixel_sample = self.pixel_00_loc
            + ((x as f64 + offset.x) * self.pixel_delta_u)
            + ((y as f64 + offset.y) * self.pixel_delta_v);

        let ray_origin = self.camera_center;
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn sample_square() -> Vec3 {
        Vec3::new(rand() - 0.5, rand() - 0.5, 0.0)
    }

    fn ray_color(r: &Ray, world: &HittableList) -> Color {
        if let Some(rec) = world.hit(r, Interval::new(0.0, INFINITY)) {
            return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
        }

        let unit_direction = r.dir.unit_vector();
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
}
