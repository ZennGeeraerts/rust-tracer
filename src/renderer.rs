use crate::color::Color;
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::point3::Point3;
use crate::ray::Ray;

use glam::Vec3;
use image::{Rgb, RgbImage};

pub struct Renderer {
    img: RgbImage,
    width: u32,
    height: u32,
}

impl Renderer {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            img: RgbImage::new(width, height),
            width,
            height,
        }
    }

    pub fn render(&mut self, scene: &HittableList) {
        let aspect_ratio = self.width as f32 / self.height as f32;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let origin = Point3::new(0.0, 0.0, 0.0);
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

        for y in 0..self.height {
            for x in 0..self.width {
                let u = x as f32 / (self.width - 1) as f32;
                let v = 1.0 - (y as f32 / (self.height - 1) as f32);
                let ray = Ray::new(
                    origin,
                    lower_left_corner + u * horizontal + v * vertical - origin,
                );

                let pixel_color = self.trace_ray(&ray, &scene);

                self.img.put_pixel(
                    x,
                    y,
                    Rgb([
                        (255.999 * pixel_color.x) as u8,
                        (255.999 * pixel_color.y) as u8,
                        (255.999 * pixel_color.z) as u8,
                    ]),
                );
            }
        }
    }

    pub fn img(&self) -> &RgbImage {
        &self.img
    }

    fn trace_ray(&self, ray: &Ray, scene: &HittableList) -> Color {
        let mut hit_record = HitRecord::default();
        if scene.hit(ray, &mut hit_record, 0.0, f32::INFINITY) {
            return hit_record.color;
        }

        let unit_dir = ray.direction().normalize();
        let t = 0.5 * (unit_dir.y + 1.0);
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
}
