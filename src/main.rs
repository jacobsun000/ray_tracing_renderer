use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;

pub mod geometry {
    mod hitable;
    pub use hitable::{HitRecord, Hitable};
    mod hitable_list;
    pub use hitable_list::HitableList;
    mod sphere;
    pub use sphere::Sphere;
}
pub mod camera;
pub mod color;
pub mod math;
pub mod pixel;
pub mod progress;
pub mod ray;
pub mod renderer;
pub mod scene;
pub mod vector3;
pub use color::Color;
pub use pixel::Pixel;
pub use ray::Ray;
pub use scene::Scene;
pub use vector3::{Point3d, Vector3d};

use crate::{camera::Camera, math::random};

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 100;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 50;

    // Camera
    let viewport_width = 3.5;
    let focal_length = 1.0;
    let camera = Camera::new(aspect_ratio, viewport_width, focal_length);

    // Scene
    let scene = Scene::sample();

    // Render
    let mut ppm_file = String::new();

    ppm_file.push_str(&format!("P3\n{} {}\n255\n", image_width, image_height));
    println!("Starts rendering");
    let time_start = Instant::now();

    for j in (0..image_height).rev() {
        let percentage = (1.0 - j as f32 / image_height as f32) * 100.0;
        progress::show(percentage);
        for i in 0..image_width {
            let r = || -> f64 { random() };
            let u = || (i as f64 + r()) / (image_width - 1) as f64;
            let v = || (j as f64 + r()) / (image_height - 1) as f64;
            let color: Color = (0..samples_per_pixel)
                .map(|_| camera.get_ray(u(), v()))
                .map(|r| renderer::ray_color(&r, &scene))
                .sum();
            let pixel = color.to_pixel(samples_per_pixel).to_string();
            ppm_file.push_str(&format!("{}\n", pixel));
        }
    }
    let time_end = Instant::now();
    let duration = (time_end - time_start).as_secs_f64();
    println!("\nRendering is completed in {:.2}s", duration);
    let mut file = File::create("./image/test.ppm").expect("Failed to create file");
    file.write_all(ppm_file.as_bytes()).expect("Write error");
}
