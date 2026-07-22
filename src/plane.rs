use crate::hit::HitRecord;
use crate::hittable::Hittable;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Plane {
	pub point: Vec3,
	pub normal: Vec3,
	pub albedo: Vec3,
}

impl Plane {
	pub fn new(point: Vec3, normal: Vec3, albedo: Vec3) -> Self {
		Self { point, normal, albedo }
	}
}

impl Hittable for Plane {
	fn hit_record(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
		let denom = Vec3::dot(self.normal, r.direction);

		if denom.abs() < 1e-8 {
			return None;
		}

		let t = Vec3::dot(self.point - r.origin, self.normal) / denom;

		if t < t_min || t > t_max {
			return None;
		}

		let point = r.at(t);
		Some(HitRecord::new(point, self.normal, t, self.albedo))
	}
}
