use crate::vec3::Vec3;

pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub albedo: Vec3,
}

impl HitRecord {
    pub fn new(point: Vec3, normal: Vec3, t: f64, albedo: Vec3) -> Self {
        Self { point, normal, t, albedo }
    }
}
