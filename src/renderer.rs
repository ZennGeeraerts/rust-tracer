use crate::camera::Camera;
use crate::color::Color;
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::ray::Ray;

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

    pub fn render(&mut self, scene: &HittableList, camera: &Camera) {
        for y in 0..self.height {
            for x in 0..self.width {
                let ray = camera.get_ray(x, y);

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
