mod vec3;
mod ray;
mod shape;
mod sphere;
mod plane;
mod camera;

use vec3::Vec3;
use sphere::Sphere;
use plane::Plane;
use camera::Camera;
use shape::Hittable;
use std::fs::File;
use std::io::Write;

fn main() {
    // Image settings
    let width: u32 = 400;
    let height: u32 = 300;

    // Create camera
    let camera = Camera::new(
        Vec3::new(0.0, 0.0, -5.0),      // position
        Vec3::new(0.0, 0.0, 1.0),       // looking forward
        Vec3::new(0.0, 1.0, 0.0),       // up direction
        90.0,                           // fov
        width,
        height,
    );

    // Create scene with shapes (stored as trait objects)
    let shapes: Vec<Box<dyn Hittable>> = vec![
        // Spheres
        Box::new(Sphere::new(Vec3::new(0.0, 0.0, 2.0), 1.0)),      // center sphere
        Box::new(Sphere::new(Vec3::new(-2.5, 0.0, 3.0), 0.8)),     // left sphere
        Box::new(Sphere::new(Vec3::new(2.5, 0.0, 3.0), 0.8)),      // right sphere
        Box::new(Sphere::new(Vec3::new(-1.0, -1.0, 3.5), 0.5)),    // small sphere
        
        // Ground plane
        Box::new(Plane::new(
            Vec3::new(0.0, -2.0, 0.0),              // point on plane
            Vec3::new(0.0, 1.0, 0.0),               // normal (pointing up)
        )),
        
        // Back wall
        Box::new(Plane::new(
            Vec3::new(0.0, 0.0, 5.0),               // point on plane
            Vec3::new(0.0, 0.0, -1.0),              // normal (pointing toward camera)
        )),
    ];

    // Render image
    let mut pixels: Vec<Vec<(u8, u8, u8)>> = vec![vec![(0, 0, 0); width as usize]; height as usize];

    for y in 0..height {
        for x in 0..width {
            let ray = camera.get_ray(x, y);

            // Find closest hit
            let mut closest_t = f64::INFINITY;
            let mut hit_color = (100, 150, 200); // sky blue background

            for shape in &shapes {
                if let Some(hit) = shape.hit(&ray) {
                    if hit.t < closest_t {
                        closest_t = hit.t;

                        // Simple lighting: use normal to calculate color
                        let light_dir = Vec3::new(1.0, 1.0, -1.0).normalize();
                        let brightness = hit.normal.dot(&light_dir).max(0.1); // at least 10% brightness

                        // Color based on surface normal
                        let r = ((hit.normal.x + 1.0) * 0.5 * 255.0 * brightness) as u8;
                        let g = ((hit.normal.y + 1.0) * 0.5 * 255.0 * brightness) as u8;
                        let b = ((hit.normal.z + 1.0) * 0.5 * 255.0 * brightness) as u8;

                        hit_color = (r, g, b);
                    }
                }
            }

            pixels[y as usize][x as usize] = hit_color;
        }

        // Progress indicator
        if y % 50 == 0 {
            println!("Rendered {} lines...", y);
        }
    }

    // Save as PPM file
    let mut file = File::create("output.ppm").expect("Failed to create file");
    
    // PPM header
    writeln!(file, "P3").expect("Failed to write");
    writeln!(file, "{} {}", width, height).expect("Failed to write");
    writeln!(file, "255").expect("Failed to write");

    // Write pixels
    for row in pixels {
        for (r, g, b) in row {
            write!(file, "{} {} {} ", r, g, b).expect("Failed to write");
        }
        writeln!(file).expect("Failed to write");
    }

    println!("Image saved to output.ppm!");
}
