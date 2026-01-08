mod camera;
mod color;
mod hittable;
mod hittable_list;
mod material;
mod point3;
mod ray;
mod renderer;
mod sphere;
mod utils;

use camera::Camera;
use color::Color;
use hittable_list::HittableList;
use material::{Lambertian, Metal};
use point3::Point3;
use renderer::Renderer;
use sphere::Sphere;
use std::rc::Rc;

use crate::material::Dielectric;

fn main() {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 400;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
    const SAMPLES_PER_PIXEL: u32 = 100;

    let mut renderer = Renderer::new(IMAGE_WIDTH, IMAGE_HEIGHT, SAMPLES_PER_PIXEL);

    let mut scene = HittableList::default();

    let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Rc::new(Dielectric::new(1.5));
    let material_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.0));

    scene.push(Box::new(Sphere::new(
        Point3::new(0.0, -100.5, -2.0),
        100.0,
        material_ground,
    )));
    scene.push(Box::new(Sphere::new(
        Point3::new(0.0, 0.0, -2.0),
        0.5,
        material_center,
    )));
    scene.push(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -2.0),
        0.5,
        material_left,
    )));
    scene.push(Box::new(Sphere::new(
        Point3::new(1.0, 0.0, -2.0),
        0.5,
        material_right,
    )));

    let mut camera = Camera::new(45.0, IMAGE_WIDTH, IMAGE_HEIGHT, 0.1, 100.0);
    camera.calculate_ray_dirs();

    renderer.render(&scene, &camera);

    renderer.img().save("render.png").unwrap();
}
