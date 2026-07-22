use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Camera {
	origin: Vec3,
	lower_left_corner: Vec3,
	horizontal: Vec3,
	vertical: Vec3,
}

impl Camera {
	pub fn new(lookfrom: Vec3, lookat: Vec3, vup: Vec3, vfov: f64, aspect_ratio: f64) -> Self {
		let theta = vfov.to_radians();
		let h = (theta / 2.0).tan();
		let viewport_height = 2.0 * h;
		let viewport_width = aspect_ratio * viewport_height;

		let w = (lookfrom - lookat).normalize();
		let u = Vec3::cross(vup, w).normalize();
		let v = Vec3::cross(w, u);

		let origin = lookfrom;
		let horizontal = viewport_width * u;
		let vertical = viewport_height * v;
		let lower_left_corner = origin - horizontal * 0.5 - vertical * 0.5 - w;

		Self {
			origin,
			lower_left_corner,
			horizontal,
			vertical,
		}
	}

	pub fn get_ray(&self, s: f64, t: f64) -> Ray {
		Ray::new(
			self.origin,
			self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin,
		)
	}
}
