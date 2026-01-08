use crate::point3::Point3;

use glam::{Mat4, Vec3, Vec4, Vec4Swizzles};

#[derive(Clone, Copy)]
pub struct RayDifferential {
    pub center: Vec3,
    pub dx: Vec3,
    pub dy: Vec3,
}

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
    ray_dirs: Vec<RayDifferential>,
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

    pub fn position(&self) -> Point3 {
        self.position
    }

    pub fn forward(&self) -> Vec3 {
        self.forward
    }

    pub fn right(&self) -> Vec3 {
        self.forward.cross(Vec3::Y).normalize()
    }

    pub fn up(&self) -> Vec3 {
        self.right().cross(self.forward).normalize()
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

    pub fn get_ray_dir(&self, x: u32, y: u32) -> RayDifferential {
        self.ray_dirs[(x + y * self.width) as usize]
    }

    pub fn calculate_ray_dirs(&mut self) {
        self.ray_dirs.resize(
            (self.width * self.height) as usize,
            RayDifferential {
                center: Vec3::ZERO,
                dx: Vec3::ZERO,
                dy: Vec3::ZERO,
            },
        );

        for y in 0..self.height {
            for x in 0..self.width {
                let cx = x as f32;
                let cy = y as f32;

                let center = self.ray_dir_at(cx, cy);
                let right = self.ray_dir_at(cx + 1.0, cy);
                let up = self.ray_dir_at(cx, cy + 1.0);

                let idx = (x + y * self.width) as usize;

                self.ray_dirs[idx] = RayDifferential {
                    center,
                    dx: right - center,
                    dy: up - center,
                };
            }
        }
    }

    fn ray_dir_at(&self, px: f32, py: f32) -> Vec3 {
        let u = px / (self.width - 1) as f32;
        let v = 1.0 - py / (self.height - 1) as f32;

        let ndc_x = u * 2.0 - 1.0;
        let ndc_y = v * 2.0 - 1.0;

        let target = self.projection_inverse * Vec4::new(ndc_x, ndc_y, -1.0, 1.0);

        let dir_view = (target.xyz() / target.w).normalize();

        (self.world * Vec4::new(dir_view.x, dir_view.y, dir_view.z, 0.0))
            .xyz()
            .normalize()
    }
}
