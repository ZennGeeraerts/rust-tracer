mod camera;
mod color;
mod hittable;
mod hittable_list;
mod point3;
mod ray;
mod renderer;
mod sphere;

use camera::Camera;
use hittable_list::HittableList;
use point3::Point3;
use renderer::Renderer;
use sphere::Sphere;

fn main() {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 400;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;

    let mut renderer = Renderer::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    let mut scene = HittableList::default();
    scene.push(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    scene.push(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    let camera = Camera::new(45.0, IMAGE_WIDTH as f32, IMAGE_HEIGHT as f32, 0.1, 100.0);

    renderer.render(&scene, &camera);

    renderer.img().save("render.png").unwrap();
}
