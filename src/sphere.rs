use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::hit::HitRecord;
use crate::hittable::Hittable;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub albedo: Vec3,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, albedo: Vec3) -> Self {
        Sphere { center, radius, albedo }
    }
}

impl Hittable for Sphere {
    fn hit_record(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = Vec3::dot(r.direction, r.direction);
        let half_b = Vec3::dot(oc, r.direction);
        let c = Vec3::dot(oc, oc) - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;

        if root < t_min || root > t_max {
            root = (-half_b + sqrtd) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }

        let point = r.at(root);
        let normal = (point - self.center) * (1.0 / self.radius);

        Some(HitRecord::new(point, normal, root, self.albedo))
    }
}