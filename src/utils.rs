use glam::Vec3;
use rand::Rng;

pub fn random_float() -> f32 {
    rand::rng().random()
}

pub fn random_float_in_range(min: f32, max: f32) -> f32 {
    min + (max - min) * random_float()
}

pub fn clamp(x: f32, min: f32, max: f32) -> f32 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

pub fn random_vec3() -> Vec3 {
    Vec3::new(random_float(), random_float(), random_float())
}

pub fn random_vec3_in_range(min: f32, max: f32) -> Vec3 {
    Vec3::new(
        random_float_in_range(min, max),
        random_float_in_range(min, max),
        random_float_in_range(min, max),
    )
}

pub fn random_vec3_in_unit_sphere() -> Vec3 {
    loop {
        let v = random_vec3_in_range(-1.0, 1.0);
        if v.length_squared() >= 1.0 {
            continue;
        }
        return v;
    }
}

pub fn random_unit_vec3() -> Vec3 {
    random_vec3_in_unit_sphere().normalize()
}
