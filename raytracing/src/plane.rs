use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::shape::{Hittable, HitRecord};

pub struct Plane {
    pub point: Vec3,
    pub normal: Vec3,
}

impl Plane {
    pub fn new(point: Vec3, normal: Vec3) -> Self {
        Plane {
            point,
            normal: normal.normalize(),
        }
    }
}

impl Hittable for Plane {
    fn hit(&self, ray: &Ray) -> Option<HitRecord> {
        // Plane equation: (P - point) · normal = 0
        // Ray equation: P(t) = origin + t * direction
        // Substitute: (origin + t * direction - point) · normal = 0
        // Solve for t: t = ((point - origin) · normal) / (direction · normal)
        
        let denom = ray.direction.dot(&self.normal);
        
        // If denom is 0, ray is parallel to plane
        if denom.abs() < 1e-6 {
            return None;
        }
        
        let t = (self.point - ray.origin).dot(&self.normal) / denom;
        
        // t must be positive (hit is ahead of ray origin)
        if t < 0.0 {
            return None;
        }
        
        Some(HitRecord {
            t,
            point: ray.at(t),
            normal: if denom > 0.0 { -self.normal } else { self.normal },
        })
    }
}
