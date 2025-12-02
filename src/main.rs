mod color;
mod hittable;
mod ray;
mod renderer;
mod sphere;
mod vec3;

use renderer::Renderer;

fn main() {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 400;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;

    let mut renderer = Renderer::new(IMAGE_WIDTH, IMAGE_HEIGHT);
    renderer.render();

    renderer.img().save("render.png").unwrap();
}
