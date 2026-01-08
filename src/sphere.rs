use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::point3::Point3;
use crate::ray::Ray;

use std::sync::Arc;

pub struct Sphere {
    origin: Point3,
    radius: f32,
    material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(origin: Point3, radius: f32, material: Arc<dyn Material>) -> Self {
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
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin() - self.origin;

        let a = ray.direction().length_squared();
        let half_b = oc.dot(ray.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        let mut t = (-half_b - sqrtd) / a;
        if !(t_min < t && t < t_max) {
            t = (-half_b + sqrtd) / a;
            if !(t_min < t && t < t_max) {
                return None;
            }
        }

        let mut hit_record = HitRecord {
            t_val: t,
            hit_point: ray.sample(t),
            normal: Default::default(),
            material: self.material.clone(),
            front_face: Default::default(),
        };

        let outward_normal = (hit_record.hit_point - self.origin) / self.radius;
        hit_record.set_face_normal(ray, outward_normal);

        Some(hit_record)
    }
}
