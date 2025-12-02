use crate::color::Color;
use crate::ray::Ray;
use crate::vec3::Point3;

pub const T_MIN: f64 = 0.00001;
pub const T_MAX: f64 = f64::MAX;

#[derive(Default)]
pub struct HitRecord {
    pub hit_point: Point3,
    pub t_val: f64,
    pub color: Color,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, hit_record: &mut HitRecord) -> bool;
}
