use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::shape::{Hittable, HitRecord};

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Self {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    // hit detection logic: check if a ray hits the sphere
    fn hit(&self, ray: &Ray) -> Option<HitRecord> {
    
        let mat = ray.origin - self.center;

        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * mat.dot(&ray.direction);
        let c = mat.dot(&mat) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            return None;
        }
        
        let sqrt_discriminant = discriminant.sqrt();
        let t = (-b - sqrt_discriminant) / (2.0 * a); // closer root
        // since we don't have a hollow sphere, we can ignore the second root
        
        // check for a hit
        if t < 0.0 {
            return None;
        }

        

        return Some(HitRecord {
            t,
            point: ray.at(t),
            normal: (ray.at(t) - self.center) * (1.0 / self.radius),
        });
    }

}