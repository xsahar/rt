use crate::hit::HitRecord;
use crate::ray::Ray;

pub trait Hittable {
    fn hit_record(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}