use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Clone)]
pub struct HitRecord {
    pub t: f64,
    pub point: Vec3,
    pub normal: Vec3,
}

/// Trait for any shape that can be hit by a ray
pub trait Hittable {
    fn hit(&self, ray: &Ray) -> Option<HitRecord>;
}
