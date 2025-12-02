use crate::color::Color;
use crate::hittable;
use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3;
use crate::vec3::{Point3, Vec3};

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
    fn hit(&self, ray: &Ray, hit_record: &mut HitRecord, t_min: f64, t_max: f64) -> bool {
        let oc = ray.origin() - self.origin;

        let a = ray.direction().magnitude_sqr();
        let half_b = vec3::dot(oc, ray.direction());
        let c = oc.magnitude_sqr() - self.radius * self.radius;

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
        hit_record.color = 0.5
            * Color::new(
                hit_record.normal.x() + 1.0,
                hit_record.normal.y() + 1.0,
                hit_record.normal.z() + 1.0,
            );
        true
    }
}
