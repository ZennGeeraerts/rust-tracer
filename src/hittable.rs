use crate::color::Color;
use crate::ray::Ray;
use crate::vec3;
use crate::vec3::{Point3, Vec3};

pub const T_MIN: f64 = 0.00001;
pub const T_MAX: f64 = f64::MAX;

#[derive(Default)]
pub struct HitRecord {
    pub hit_point: Point3,
    pub t_val: f64,
    pub color: Color,
    pub normal: Vec3,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = vec3::dot(ray.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, hit_record: &mut HitRecord) -> bool;
}
