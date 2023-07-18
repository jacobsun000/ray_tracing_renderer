use crate::pixel::Pixel;
use crate::vector3::Vector3;

pub type Color = Vector3<f64>;

impl Color {
    pub fn to_pixel(&self) -> Pixel {
        let f = |a| (255.999 * a) as i32;
        self.data.iter().map(f).collect()
    }
}
