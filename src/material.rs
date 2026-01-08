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
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f32) -> Self {
        Self {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
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
        *scattered = Ray::new(
            hit_record.hit_point,
            reflected + self.fuzz * utils::random_vec3_in_unit_sphere(),
        );

        scattered.direction().dot(hit_record.normal) > 0.0
    }
}

pub struct Dielectric {
    index_of_refraction: f32,
}

impl Dielectric {
    pub fn new(index_of_refraction: f32) -> Self {
        Self {
            index_of_refraction,
        }
    }

    fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
        let mut r0 = (1.0 - ref_idx) / 1.0 + ref_idx;
        r0 = r0 * r0;
        r0 + (1.0 - r0) * f32::powf(1.0 - cosine, 5.0)
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        ray: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let refraction_ratio = if hit_record.front_face {
            1.0 / self.index_of_refraction
        } else {
            self.index_of_refraction
        };

        let unit_dir = ray.direction().normalize();
        let cos_theta = f32::min((-unit_dir).dot(hit_record.normal), 1.0);
        let sin_theta = f32::sqrt(1.0 - cos_theta * cos_theta);

        let dir = if (refraction_ratio * sin_theta > 1.0)
            || (Self::reflectance(cos_theta, refraction_ratio) > utils::random_float())
        {
            unit_dir.reflect(hit_record.normal)
        } else {
            unit_dir.refract(hit_record.normal, refraction_ratio)
        };

        *attenuation = Color::new(1.0, 1.0, 1.0);
        *scattered = Ray::new(hit_record.hit_point, dir);
        true
    }
}
