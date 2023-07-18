use crate::color::Color;
use crate::ray::Ray;

pub fn ray_color(ray: &Ray) -> Color {
    let unit_direction = ray.direction.unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    Color::new([1.0, 1.0, 1.0]) * (1.0 - t) + Color::new([0.5, 0.7, 1.0]) * t
}
