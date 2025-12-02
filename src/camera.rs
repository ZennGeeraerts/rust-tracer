use crate::point3::Point3;
use crate::ray::Ray;

use glam::{Mat4, Vec3, Vec4, Vec4Swizzles};

pub struct Camera {
    projection: Mat4,
    position: Point3,
    forward: Vec3,
    width: f32,
    height: f32,
    vertical_fov: f32,
}

impl Camera {
    pub fn new(
        vertical_fov: f32,
        width: f32,
        height: f32,
        near_plane: f32,
        far_plane: f32,
    ) -> Self {
        let aspect_ratio = width / height;

        Self {
            projection: Mat4::perspective_rh(vertical_fov, aspect_ratio, near_plane, far_plane),
            position: Vec3::default(),
            forward: Vec3::NEG_Z,
            width,
            height,
            vertical_fov,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let ndc_x = u * 2.0 - 1.0;
        let ndc_y = v * 2.0 - 1.0;

        let target = self.projection.inverse() * Vec4::new(ndc_x, ndc_y, 1.0, 1.0);
        let view = Mat4::look_at_rh(self.position, self.position + self.forward, Vec3::Y);

        let ray_dir_view = (target.xyz() / target.w).normalize();
        let ray_dir_world = (view.inverse()
            * Vec4::new(ray_dir_view.x, ray_dir_view.y, ray_dir_view.z, 0.0))
        .xyz()
        .normalize();

        Ray::new(self.position, ray_dir_world)
    }
}
