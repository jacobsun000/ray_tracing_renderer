use std::time::Instant;

mod geometry {
    mod hitable;
    pub use hitable::{HitRecord, Hitable};
    mod hitable_list;
    pub use hitable_list::HitableList;
    mod sphere;
    pub use sphere::Sphere;
}

mod camera;
mod color;
mod image;
mod math;
mod pixel;
mod progress;
mod ray;
mod renderer;
mod scene;
mod vector3;
use crate::{image::Image, math::random, renderer::Renderer};
use color::Color;
use pixel::Pixel;
use ray::Ray;
use scene::Scene;
use vector3::{Point3d, Vector3d};

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let width = 400;
    let height = (width as f64 / aspect_ratio) as usize;
    let mut image = Image::new("./image/test.ppm", width, height);

    // Scene
    let scene = Scene::sample();

    // Render
    let option = renderer::RenderOption {
        samples_per_pixel: 100,
        max_depth: 50,
    };
    let mut renderer = Renderer {
        scene: &scene,
        image: &mut image,
        option: option,
    };

    println!("Starts rendering");
    let time_start = Instant::now();
    renderer.render();
    let time_end = Instant::now();
    let duration = (time_end - time_start).as_secs_f64();
    println!("\nRendering is completed in {:.2}s", duration);
    image.output().expect("Image output error");
}
