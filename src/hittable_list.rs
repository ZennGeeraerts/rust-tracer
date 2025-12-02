use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;

#[derive(Default)]
pub struct HittableList {
    hittables: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn push(&mut self, hittable: Box<dyn Hittable>) {
        self.hittables.push(hittable);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, hit_record: &mut HitRecord, t_min: f32, t_max: f32) -> bool {
        let mut temp_record = HitRecord::default();
        let mut closest_so_far = t_max;
        let mut is_hit = false;

        for hittable in &self.hittables {
            if hittable.hit(ray, &mut temp_record, t_min, closest_so_far) {
                is_hit = true;
                closest_so_far = temp_record.t_val;
                *hit_record = temp_record.clone();
            }
        }

        is_hit
    }
}
