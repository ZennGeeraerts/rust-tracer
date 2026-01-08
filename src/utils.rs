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
