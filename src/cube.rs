use crate::hit::HitRecord;
use crate::hittable::Hittable;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Cube {
	pub min: Vec3,
	pub max: Vec3,
	pub albedo: Vec3,
}

impl Cube {
	pub fn new(center: Vec3, size: f64, albedo: Vec3) -> Self {
		let half = Vec3::new(size * 0.5, size * 0.5, size * 0.5);
		Self {
			min: center - half,
			max: center + half,
			albedo,
		}
	}
}

impl Hittable for Cube {
	fn hit_record(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
		let mut t0 = t_min;
		let mut t1 = t_max;

		for axis in 0..3 {
			let origin = match axis {
				0 => r.origin.x,
				1 => r.origin.y,
				_ => r.origin.z,
			};
			let direction = match axis {
				0 => r.direction.x,
				1 => r.direction.y,
				_ => r.direction.z,
			};
			let min_bound = match axis {
				0 => self.min.x,
				1 => self.min.y,
				_ => self.min.z,
			};
			let max_bound = match axis {
				0 => self.max.x,
				1 => self.max.y,
				_ => self.max.z,
			};

			let inv_d = 1.0 / direction;
			let mut near = (min_bound - origin) * inv_d;
			let mut far = (max_bound - origin) * inv_d;

			if inv_d < 0.0 {
				std::mem::swap(&mut near, &mut far);
			}

			if near > t0 {
				t0 = near;
			}
			if far < t1 {
				t1 = far;
			}

			if t1 <= t0 {
				return None;
			}
		}

		let t = t0;
		let point = r.at(t);
		let epsilon = 0.001;

		let normal = if (point.x - self.min.x).abs() < epsilon {
			Vec3::new(-1.0, 0.0, 0.0)
		} else if (point.x - self.max.x).abs() < epsilon {
			Vec3::new(1.0, 0.0, 0.0)
		} else if (point.y - self.min.y).abs() < epsilon {
			Vec3::new(0.0, -1.0, 0.0)
		} else if (point.y - self.max.y).abs() < epsilon {
			Vec3::new(0.0, 1.0, 0.0)
		} else if (point.z - self.min.z).abs() < epsilon {
			Vec3::new(0.0, 0.0, -1.0)
		} else {
			Vec3::new(0.0, 0.0, 1.0)
		};

		Some(HitRecord::new(point, normal, t, self.albedo))
	}
}
