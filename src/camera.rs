use crate::point3::Point3;
use crate::ray::Ray;

use glam::{Mat4, Vec3, Vec4, Vec4Swizzles};

pub struct Camera {
    projection: Mat4,
    projection_inverse: Mat4,
    view: Mat4,
    world: Mat4,
    position: Point3,
    forward: Vec3,
    width: u32,
    height: u32,
    vertical_fov: f32,
    near_plane: f32,
    far_plane: f32,
    ray_dirs: Vec<Vec3>,
}

impl Camera {
    pub fn new(
        vertical_fov: f32,
        width: u32,
        height: u32,
        near_plane: f32,
        far_plane: f32,
    ) -> Self {
        let position = Vec3::default();
        let forward = Vec3::NEG_Z;
        let aspect_ratio = width as f32 / height as f32;

        let projection = Mat4::perspective_rh(vertical_fov, aspect_ratio, near_plane, far_plane);
        let view = Mat4::look_at_rh(position, position + forward, Vec3::Y);

        Self {
            projection,
            projection_inverse: projection.inverse(),
            view,
            world: view.inverse(),
            position,
            forward,
            width,
            height,
            vertical_fov,
            near_plane,
            far_plane,
            ray_dirs: vec![],
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;

        let aspect_ratio = width as f32 / height as f32;

        self.projection = Mat4::perspective_rh(
            self.vertical_fov,
            aspect_ratio,
            self.near_plane,
            self.far_plane,
        );

        self.projection_inverse = self.projection.inverse();

        self.calculate_ray_dirs();
    }

    pub fn get_ray(&self, x: u32, y: u32) -> Ray {
        Ray::new(self.position, self.ray_dirs[(x + y * self.width) as usize])
    }

    pub fn calculate_ray_dirs(&mut self) {
        self.ray_dirs
            .resize((self.width * self.height) as usize, Vec3::ZERO);

        for y in 0..self.height {
            for x in 0..self.width {
                let u = x as f32 / (self.width - 1) as f32;
                let v = 1.0 - (y as f32 / (self.height - 1) as f32);

                let ndc_x = u * 2.0 - 1.0;
                let ndc_y = v * 2.0 - 1.0;

                let target = self.projection_inverse * Vec4::new(ndc_x, ndc_y, 1.0, 1.0);

                let ray_dir_view = (target.xyz() / target.w).normalize();
                let ray_dir_world = (self.world
                    * Vec4::new(ray_dir_view.x, ray_dir_view.y, ray_dir_view.z, 0.0))
                .xyz()
                .normalize();

                self.ray_dirs[(x + y * self.width) as usize] = ray_dir_world;
            }
        }
    }
}
