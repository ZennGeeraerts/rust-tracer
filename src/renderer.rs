use crate::color::Color;
use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3;
use crate::vec3::{Point3, Vec3};

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

    pub fn render(&mut self) {
        let aspect_ratio = self.width as f64 / self.height as f64;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let origin = Point3::new(0.0, 0.0, 0.0);
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

        for j in (0..self.height).rev() {
            for i in 0..self.width {
                let u = i as f64 / (self.width - 1) as f64;
                let v = j as f64 / (self.height - 1) as f64;
                let ray = Ray::new(
                    origin,
                    lower_left_corner + u * horizontal + v * vertical - origin,
                );

                let pixel_color = self.background_color(&ray);

                self.img.put_pixel(
                    i,
                    j,
                    Rgb([
                        (255.999 * pixel_color.x()) as u8,
                        (255.999 * pixel_color.y()) as u8,
                        (255.999 * pixel_color.z()) as u8,
                    ]),
                );
            }
        }
    }

    pub fn img(&self) -> &RgbImage {
        &self.img
    }

    fn background_color(&self, ray: &Ray) -> Color {
        let sphere = Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5);
        let mut hit_record = HitRecord::default();
        if sphere.hit(ray, &mut hit_record) {
            return hit_record.color;
        }

        let unit_dir = vec3::unit_vector(ray.direction());
        let t = 0.5 * (unit_dir.y() + 1.0);
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
}
