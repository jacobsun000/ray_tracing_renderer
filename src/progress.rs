use std::io::prelude::*;

pub fn show(percentage: f32) {
    print!("\x1B[?25l");
    print!("\r{:6.2}% complete", percentage);
    std::io::stdout().flush().unwrap();
    print!("\x1B[?25h");
}