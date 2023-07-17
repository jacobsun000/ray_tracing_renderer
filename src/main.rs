use std::fs::File;
use std::io::prelude::*;

mod color;
mod vector3;
mod pixel;
mod progress;
mod ray;
use color::Color;

fn main() {
    let image_width = 256;
    let image_height = 256;

    let mut ppm_file = String::new();

    ppm_file.push_str(&format!("P3\n{} {}\n255\n", image_width, image_height));
    println!("Starts rendering");

    for j in (0..image_height).rev() {
        let percentage = (1.0 - j as f32 / image_height as f32) * 100.0;
        progress::show(percentage);
        for i in 0..image_width {
            let r = i as f64 / (image_width - 1) as f64;
            let g = j as f64 / (image_height - 1) as f64;
            let b = 0.25;
            let pixel = Color::new([r, g, b]).to_pixel().to_string();
            ppm_file.push_str(&format!("{}\n", pixel));
        }
    }

    println!("\nFinished rendering");
    let mut file = File::create("test.ppm").expect("Failed to create file");
    file.write_all(ppm_file.as_bytes()).expect("Write error");
}
