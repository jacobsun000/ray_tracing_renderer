use std::fs::File;
use std::io::prelude::*;


fn main() {
    let mut file = File::create("test.ppm").unwrap();
    let image_width = 256;
    let image_height = 256;
    let mut ppm_file = format!("P3\n{} {}\n255\n", image_width, image_height);

    for j in (0..image_height).rev() {
        let progress = (1.0 - j as f64 / image_height as f64) * 100.0;
        println!("{:6.2}% complete", progress);
        for i in 0..image_width {
            let r: f64 = i as f64 / (image_width - 1) as f64;
            let g: f64 = j as f64 / (image_height - 1) as f64; 
            let b: f64 = 0.25;
            let ir: i32 = (255.999 * r) as i32;
            let ig: i32 = (255.999 * g) as i32;
            let ib: i32 = (255.999 * b) as i32;
            ppm_file += &format!("{} {} {}\n", ir, ig, ib);
        }
    }

    println!("Finished renderering");

    file.write(&ppm_file.as_bytes()).expect("Write error");
}
