use std::fs::File;
use std::io::prelude::*;

pub mod geometry {
    mod hitable;
    pub use hitable::{HitRecord, Hitable};
    mod hitable_list;
    pub use hitable_list::HitableList;
    mod sphere;
    pub use sphere::Sphere;
}
pub mod color;
pub mod pixel;
pub mod progress;
pub mod ray;
pub mod renderer;
pub mod vector3;
pub mod math;
pub mod scene;
pub use ray::Ray;
pub use color::Color;
pub use pixel::Pixel;
pub use scene::Scene;
pub use vector3::{Point3d, Vector3d};

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;
    let origin = Point3d::new([0.0, 0.0, 0.0]);
    let horizontal = Vector3d::new([viewport_width, 0.0, 0.0]);
    let vertical = Vector3d::new([0.0, viewport_height, 0.0]);
    let half_horizontal = horizontal / 2.0;
    let half_vertical = vertical / 2.0;
    let adjustment = Vector3d::new([0.0, 0.0, focal_length]);
    let lower_left_corner = origin - half_horizontal - half_vertical - adjustment;

    // Scene
    let scene = Scene::sample();

    // Render
    let mut ppm_file = String::new();

    ppm_file.push_str(&format!("P3\n{} {}\n255\n", image_width, image_height));
    println!("Starts rendering");

    for j in (0..image_height).rev() {
        let percentage = (1.0 - j as f32 / image_height as f32) * 100.0;
        progress::show(percentage);
        for i in 0..image_width {
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;
            let direction = lower_left_corner + horizontal * u + vertical * v - origin;
            let r = Ray { origin, direction };
            let color = renderer::ray_color(&r, &scene);
            let pixel = color.to_pixel().to_string();
            ppm_file.push_str(&format!("{}\n", pixel));
        }
    }

    println!("\nFinished rendering");
    let mut file = File::create("./image/test.ppm").expect("Failed to create file");
    file.write_all(ppm_file.as_bytes()).expect("Write error");
}
