use crate::camera::Camera;
use crate::geometry::*;
use crate::Point3d;

pub struct Scene {
    pub objects: HitableList,
    pub camera: Camera,
}

impl Scene {
    pub fn sample() -> Self {
        let mut world: Vec<Box<dyn Hitable + Sync>> = Vec::new();
        let sphere0: Sphere = Sphere {
            center: Point3d::new([0.0, 0.0, -1.0]),
            radius: 0.5,
        };
        let sphere1 = Sphere {
            center: Point3d::new([0.0, -100.5, -1.0]),
            radius: 100.0,
        };
        world.push(Box::new(sphere0));
        world.push(Box::new(sphere1));

        let aspect_ratio = 16.0 / 9.0;
        let viewport_width = 3.5;
        let focal_length = 1.0;

        return Scene {
            objects: HitableList { hitables: world },
            camera: Camera::new(aspect_ratio, viewport_width, focal_length),
        };
    }
}
