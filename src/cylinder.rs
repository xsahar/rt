use crate::hit::HitRecord;
use crate::hittable::Hittable;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Cylinder{
   pub center: Vec3,
   pub radius: f64,
   pub y_min: f64,
   pub y_max: f64,
   pub albedo: Vec3,
}

impl Cylinder {
	pub fn new(center: Vec3, radius: f64, y_min:f64, y_max:f64, albedo: Vec3) -> Self {
		Cylinder{ center, radius, y_min , y_max , albedo }
	}
}

impl Hittable for Cylinder {
    fn hit_record(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let min_y = self.center.y + self.y_min;
        let max_y = self.center.y + self.y_max;

        let mut best_hit: Option<HitRecord> = None;

        let oc = r.origin - self.center;
        let a = r.direction.x * r.direction.x + r.direction.z * r.direction.z;

        if a.abs() >= 1e-8 {
            let half_b = oc.x * r.direction.x + oc.z * r.direction.z;
            let c = oc.x * oc.x + oc.z * oc.z - self.radius * self.radius;
            let discriminant = half_b * half_b - a * c;

            if discriminant >= 0.0 {
                let sqrtd = discriminant.sqrt();

                for root in [(-half_b - sqrtd) / a, (-half_b + sqrtd) / a] {
                    if root < t_min || root > t_max {
                        continue;
                    }

                    let point = r.at(root);
                    if point.y < min_y || point.y > max_y {
                        continue;
                    }

                    let normal = Vec3::new(point.x - self.center.x, 0.0, point.z - self.center.z).normalize();
                    let candidate = HitRecord::new(point, normal, root, self.albedo);

                    match &best_hit {
                        Some(current) if current.t <= candidate.t => {}
                        _ => best_hit = Some(candidate),
                    }
                }
            }
        }

        if r.direction.y.abs() >= 1e-8 {
            for (cap_y, normal_y) in [(min_y, -1.0), (max_y, 1.0)] {
                let root = (cap_y - r.origin.y) / r.direction.y;

                if root < t_min || root > t_max {
                    continue;
                }

                let point = r.at(root);
                let dx = point.x - self.center.x;
                let dz = point.z - self.center.z;

                if dx * dx + dz * dz > self.radius * self.radius {
                    continue;
                }

                let normal = Vec3::new(0.0, normal_y, 0.0);
                let candidate = HitRecord::new(point, normal, root, self.albedo);

                match &best_hit {
                    Some(current) if current.t <= candidate.t => {}
                    _ => best_hit = Some(candidate),
                }
            }
        }

        best_hit
    }
}