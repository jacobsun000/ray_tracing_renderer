use crate::geometry::*;
use crate::Point3d;

pub struct Scene {
    pub objects: HitableList,
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
        return Scene {
            objects: HitableList { hitables: world },
        };
    }
}
