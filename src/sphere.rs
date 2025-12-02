use crate::color::Color;
use crate::hittable;
use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3;
use crate::vec3::Point3;

pub struct Sphere {
    origin: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(origin: Point3, radius: f64) -> Self {
        Self { origin, radius }
    }

    pub fn origin(&self) -> Point3 {
        self.origin
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, hit_record: &mut HitRecord) -> bool {
        let oc = ray.origin() - self.origin;

        let a = vec3::dot(ray.direction(), ray.direction());
        let half_b = vec3::dot(oc, ray.direction());
        let c = vec3::dot(oc, oc) - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if !(hittable::T_MIN < root && root < hittable::T_MAX) {
            root = (-half_b + sqrtd) / a;
            if !(hittable::T_MIN < root && root < hittable::T_MAX) {
                return false;
            }
        }

        hit_record.t_val = root;

        hit_record.hit_point = ray.origin() + hit_record.t_val * ray.direction();
        hit_record.color = Color::new(1.0, 0.0, 0.0);
        true
    }
}
