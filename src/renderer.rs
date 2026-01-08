use crate::camera::Camera;
use crate::color::Color;
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::ray::Ray;
use crate::utils;

use image::{Rgb, RgbImage};

pub struct Renderer {
    img: RgbImage,
    width: u32,
    height: u32,
    samples_per_pixel: u32,
}

impl Renderer {
    pub fn new(width: u32, height: u32, samples_per_pixel: u32) -> Self {
        Self {
            img: RgbImage::new(width, height),
            width,
            height,
            samples_per_pixel,
        }
    }

    pub fn render(&mut self, scene: &HittableList, camera: &Camera) {
        for y in 0..self.height {
            for x in 0..self.width {
                let mut pixel_color = Color::ZERO;
                let ray_dir = camera.get_ray_dir(x, y);

                for _ in 0..self.samples_per_pixel {
                    let jx = utils::random_float() - 0.5;
                    let jy = utils::random_float() - 0.5;

                    let jittered_dir =
                        (ray_dir.center + ray_dir.dx * jx + ray_dir.dy * jy).normalize();

                    let camera_pos = camera.position();
                    let ray = Ray::new(camera_pos, jittered_dir);

                    pixel_color += self.trace_ray(&ray, &scene, 50);
                }

                self.put_pixel(x, y, pixel_color);
            }
        }
    }

    pub fn img(&self) -> &RgbImage {
        &self.img
    }

    fn trace_ray(&self, ray: &Ray, scene: &HittableList, recursion_depth: i32) -> Color {
        if recursion_depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        let mut hit_record = HitRecord::default();
        if scene.hit(ray, &mut hit_record, 0.001, f32::INFINITY) {
            let mut attenuation = Color::default();
            let mut scattered = Ray::default();

            if hit_record.material.as_ref().unwrap().scatter(
                ray,
                &hit_record,
                &mut attenuation,
                &mut scattered,
            ) {
                return attenuation * self.trace_ray(&scattered, scene, recursion_depth - 1);
            } else {
                return Color::new(0.0, 0.0, 0.0);
            }
        }

        let unit_dir = ray.direction().normalize();
        let t = 0.5 * (unit_dir.y + 1.0);
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }

    fn put_pixel(&mut self, x: u32, y: u32, pixel_color: Color) {
        let scale = 1.0 / self.samples_per_pixel as f32;
        let r = f32::sqrt(pixel_color.x * scale);
        let g = f32::sqrt(pixel_color.y * scale);
        let b = f32::sqrt(pixel_color.z * scale);

        self.img.put_pixel(
            x,
            y,
            Rgb([
                (255.999 * utils::clamp(r, 0.0, 0.999)) as u8,
                (255.999 * utils::clamp(g, 0.0, 0.999)) as u8,
                (255.999 * utils::clamp(b, 0.0, 0.999)) as u8,
            ]),
        );
    }
}
