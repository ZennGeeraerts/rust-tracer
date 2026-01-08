use crate::material::Material;
use crate::point3::Point3;
use crate::ray::Ray;

use glam::Vec3;
use std::sync::Arc;

pub struct HitRecord {
    pub hit_point: Point3,
    pub t_val: f32,
    pub normal: Vec3,
    pub material: Arc<dyn Material>,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = ray.direction().dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}
