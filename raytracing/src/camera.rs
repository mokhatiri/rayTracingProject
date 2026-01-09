use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Camera {
    origin: Vec3,
    direction: Vec3,
    up: Vec3,
    fov: f64,
    width: u32,
    height: u32,
}

impl Camera {
    pub fn new(origin: Vec3, direction: Vec3, up: Vec3, fov: f64, width: u32, height: u32) -> Self {
        Camera {
            origin,
            direction: direction.normalize(),
            up: up.normalize(),
            fov,
            width,
            height,
        }
    }

    pub fn get_ray(&self, x: u32, y: u32) -> Ray {
        // Normalize pixel coordinates to -1..1
        let u = (x as f64) / (self.width as f64) * 2.0 - 1.0;
        let v = -((y as f64) / (self.height as f64) * 2.0 - 1.0);  // flip Y (top-left is 0)

        // Create a proper viewport
        // Right vector (perpendicular to direction and up)
        let right = self.direction.cross(&self.up).normalize();
        // Recalculate up to ensure orthogonal basis
        let up_norm = right.cross(&self.direction).normalize();

        // Field of view determines how wide the viewport is
        let fov_rad = self.fov * std::f64::consts::PI / 180.0;
        let viewport_height = 2.0 * (fov_rad / 2.0).tan();
        let viewport_width = viewport_height * (self.width as f64 / self.height as f64);

        // Create ray direction based on viewport position
        let ray_direction = (
            self.direction
            + right * (u * viewport_width / 2.0)
            + up_norm * (v * viewport_height / 2.0)
        ).normalize();

        Ray::new(self.origin, ray_direction)
    }
}