use crate::Pixel;
use crate::vector3::Vector3;

pub type Color = Vector3<f64>;

impl Color {
    pub fn to_pixel(&self, samples_per_pixel: i32) -> Pixel {
        let ratio: f64 = 1.0 / samples_per_pixel as f64;
        let f = |a: &f64| (255.999 * (ratio * a).sqrt().clamp(0.0, 0.999)) as i32;
        self.data.iter().map(f).collect()
    }

    pub fn black() -> Self {
        Color::new([0.0, 0.0, 0.0])
    }

    pub fn white() -> Self {
        Color::new([1.0, 1.0, 1.0])
    }

    pub fn red() -> Self {
        Color::new([1.0, 0.0, 0.0])
    }

    pub fn green() -> Self {
        Color::new([0.0, 1.0, 1.0])
    }

    pub fn blue() -> Self {
        Color::new([0.0, 0.0, 1.0])
    }
}
