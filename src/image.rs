use crate::Pixel;

use std::fs::File;
use std::io::{Result, Write};

pub struct Image {
    pub canvas: Vec<Vec<Pixel>>,
    pub filename: String,
    pub width: usize,
    pub height: usize,
}

impl Image {
    pub fn new(filename: &str, width: usize, height: usize) -> Self {
        let canvas = vec![vec![Default::default(); width]; height];
        let filename = filename.to_string();
        Image {
            canvas,
            filename,
            width,
            height,
        }
    }

    pub fn output(&self) -> Result<()> {
        let mut file = File::create(&self.filename)?;
        let ppm_header = format!("P3\n{} {}\n255\n", self.width, self.height);
        file.write(ppm_header.as_bytes())?;
        let mut pixels_str: String = self
            .canvas
            .iter()
            .flat_map(|row| row.iter())
            .map(|pixel| pixel.to_string())
            .collect::<Vec<String>>()
            .join("\n");
        pixels_str.push('\n');
        file.write_all(pixels_str.as_bytes())?;
        Ok(())
    }
}
