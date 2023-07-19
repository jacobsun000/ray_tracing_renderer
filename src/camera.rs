use crate::{Point3d, Ray, Vector3d};

pub struct Camera {
    origin: Point3d,
    lower_left_corner: Point3d,
    horizontal: Vector3d,
    vertical: Vector3d,
}

impl Camera {
    pub fn new(aspect_ratio: f64, viewport_width: f64, focal_length: f64) -> Self {
        let viewport_height = viewport_width / aspect_ratio;
        let origin = Point3d::new([0.0, 0.0, 0.0]);
        let horizontal = Vector3d::new([viewport_width, 0.0, 0.0]);
        let vertical = Vector3d::new([0.0, viewport_height, 0.0]);
        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - Vector3d::new([0.0, 0.0, focal_length]);
        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let origin = self.origin;
        let direction = self.lower_left_corner + self.horizontal * u + self.vertical * v - origin;
        Ray { origin, direction }
    }
}
