use crate::hit::HitRecord;
use crate::hittable::Hittable;
use crate::ray::Ray;

pub struct World {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl World {
    pub fn new() -> Self {
        Self { objects: Vec::new() }
    }

    pub fn add<T: Hittable + 'static>(&mut self, object: T) {
        self.objects.push(Box::new(object));
    }

    pub fn hit_record(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut hit_result = None;

        for object in &self.objects {
            if let Some(hit) = object.hit_record(r, t_min, closest_so_far) {
                closest_so_far = hit.t;
                hit_result = Some(hit);
            }
        }

        hit_result
    }
}