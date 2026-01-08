use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::utils;

pub trait Material {
    fn scatter(
        &self,
        ray: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        return Self { albedo };
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        ray: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = hit_record.normal * utils::random_unit_vec3();

        if utils::is_near_zero(scatter_direction) {
            scatter_direction = hit_record.normal;
        }

        *attenuation = self.albedo;
        *scattered = Ray::new(hit_record.hit_point, scatter_direction);
        true
    }
}

pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        ray: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = ray.direction().normalize().reflect(hit_record.normal);

        *attenuation = self.albedo;
        *scattered = Ray::new(hit_record.hit_point, reflected);

        scattered.direction().dot(hit_record.normal) > 0.0
    }
}
