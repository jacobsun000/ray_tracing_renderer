use crate::Ray;
use crate::Color;
use crate::Scene;
use crate::geometry::Hitable;

pub fn ray_color(ray: &Ray, scene: &Scene) -> Color {
    if let Some(rec) = scene.objects.hit(ray) {
        return (rec.normal + Color::new([1.0,1.0,1.0])) * 0.5;
    }
    let unit_direction = ray.direction.unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    return Color::new([1.0, 1.0, 1.0]) * (1.0 - t) + Color::new([0.5, 0.7, 1.0]) * t;
}
