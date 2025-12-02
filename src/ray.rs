use crate::point3::Point3;

use glam::Vec3;

#[derive(Default)]
pub struct Ray {
    origin: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, dir: Vec3) -> Self {
        Self { origin, dir }
    }

    pub fn origin(&self) -> Point3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    pub fn sample(&self, t: f32) -> Point3 {
        self.origin + self.dir * t
    }
}
