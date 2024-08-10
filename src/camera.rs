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
use std::sync::Arc;

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
            look_from: Point3::empty(),
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

        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        let viewport_upper_left =
            look_from - (focus_dist * w) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel_00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

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
    pub fn render(&mut self, mut image_writer: impl ImageWriter, world: Arc<dyn Hittable>) {
        image_writer.init(self);

        let image = (0..self.image_height)
            .into_par_iter()
            .flat_map(|y| {
                println!("scanline {}", y);
                (0..self.image_width)
                    .into_par_iter()
                    .map(|x| {
                        // (0..self.samples_per_pixel).map(|sample_i| {
                        //     let r = self.get_ray(x, y);
                        //     Camera::ray_color(&r, self.max_depth, world.to_owned())
                        // }).sum::<Color>() / self.samples_per_pixel as f64

                        let mut pixel_color = Color::empty();
                        for _ in 0..self.samples_per_pixel {
                            let r = self.get_ray(x, y);
                            // cloning an arc is cheap
                            pixel_color += Camera::ray_color(&r, self.max_depth, world.to_owned());
                        }

                        pixel_color / self.samples_per_pixel as f64
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        for pixel in image {
            image_writer.write_pixel(pixel);
        }

        println!();
        image_writer.finish();
        println!("Done!");
    }

    fn get_ray(&self, x: u32, y: u32) -> Ray {
        let offset = Camera::sample_square();
        let pixel_sample = self.pixel_00_loc
            + ((x as f64 + offset.x) * self.pixel_delta_u)
            + ((y as f64 + offset.y) * self.pixel_delta_v);

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.look_from
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - ray_origin;

        Ray::new_tm(ray_origin, ray_direction, rand())
    }

    fn sample_square() -> Vec3 {
        vec3(rand() - 0.5, rand() - 0.5, 0.0)
    }

    fn defocus_disk_sample(&self) -> Point3 {
        let p = Vec3::random_in_unit_disk();
        self.look_from + (p.x * self.defocus_disk_u) + (p.y * self.defocus_disk_v)
    }

    fn ray_color(r: &Ray, depth: u32, world: Arc<dyn Hittable>) -> Color {
        // if we hit the bounce limit, no more light it gathered
        if depth == 0 {
            return Color::empty();
        }

        if let Some(rec) = world.hit(r, interval(0.001, f64::INFINITY)) {
            if let Some((attenuation, scattered)) = rec.mat.scatter(r, &rec) {
                return attenuation * Self::ray_color(&scattered, depth - 1, world);
            }
            return Color::empty();
        }

        let unit_direction = r.dir.unit_vector();
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * color(1.0, 1.0, 1.0) + t * color(0.5, 0.7, 1.0)
    }
}
