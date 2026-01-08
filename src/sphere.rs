use crate::color::Color;
use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::point3::Point3;
use crate::ray::Ray;

use std::rc::Rc;

pub struct Sphere {
    origin: Point3,
    radius: f32,
    material: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(origin: Point3, radius: f32, material: Rc<dyn Material>) -> Self {
        Self {
            origin,
            radius,
            material,
        }
    }

    pub fn origin(&self) -> Point3 {
        self.origin
    }

    pub fn radius(&self) -> f32 {
        self.radius
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, hit_record: &mut HitRecord, t_min: f32, t_max: f32) -> bool {
        let oc = ray.origin() - self.origin;

        let a = ray.direction().length_squared();
        let half_b = oc.dot(ray.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();

        let mut t = (-half_b - sqrtd) / a;
        if !(t_min < t && t < t_max) {
            t = (-half_b + sqrtd) / a;
            if !(t_min < t && t < t_max) {
                return false;
            }
        }

        hit_record.t_val = t;
        hit_record.hit_point = ray.sample(hit_record.t_val);
        let outward_normal = (hit_record.hit_point - self.origin) / self.radius;
        hit_record.set_face_normal(ray, outward_normal);
        hit_record.material = Some(self.material.clone());
        true
    }
}
