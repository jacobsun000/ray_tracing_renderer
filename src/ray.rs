use crate::vector3::{Point3d, Vector3d};

#[derive(Clone, Copy)]
pub struct Ray {
    pub origin: Point3d,
    pub direction: Vector3d,
}

impl Ray {
    pub fn at(&self, t: f64) -> Point3d {
        self.origin + self.direction * t
    }
}
