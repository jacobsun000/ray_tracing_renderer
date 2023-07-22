use crate::material::Material;
use crate::ray::Ray;
use crate::vector3::{Point3d, Vector3d};
use std::cmp::Ordering;

#[derive(Clone, Copy, Debug)]
pub struct HitRecord {
    pub point: Point3d,
    pub normal: Vector3d,
    pub t: f64,
    pub front_face: bool,
    pub material: Material
}

pub trait Hitable {
    fn hit(&self, ray: &Ray) -> Option<HitRecord>;
}

impl HitRecord {
    pub fn new(ray: &Ray, point: Point3d, normal: Vector3d, t: f64, material: Material) -> Self {
        let front_face = ray.direction.dot(&normal) < 0.0;
        let normal = if front_face { normal } else { -normal };
        HitRecord {
            point,
            normal,
            t,
            front_face,
            material
        }
    }
}
impl PartialEq for HitRecord {
    fn eq(&self, other: &Self) -> bool {
        self.t.eq(&other.t)
    }
}

impl Eq for HitRecord {}

impl PartialOrd for HitRecord {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.t.total_cmp(&other.t))
    }
}

impl Ord for HitRecord {
    fn cmp(&self, other: &Self) -> Ordering {
        self.t.total_cmp(&other.t)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vector3::{Point3d, Vector3d};
    use crate::Color;

    #[test]
    fn test_hitrecord_new() {
        let ray = Ray {
            origin: Point3d::new([0.0, 0.0, 0.0]),
            direction: Vector3d::new([1.0, 0.0, 0.0]),
        };
        let point = Point3d::new([1.0, 2.0, 3.0]);
        let normal = Vector3d::new([-1.0, 0.0, 0.0]);
        let t = 2.0;
        let hit_record = HitRecord::new(&ray, point, normal, t, Material::Lambertian(Color::white()));

        assert_eq!(hit_record.point, point);
        assert_eq!(hit_record.normal, normal);
        assert_eq!(hit_record.t, t);
        assert_eq!(hit_record.front_face, true);
    }

    #[test]
    fn test_hitrecord_new_back_face() {
        let ray = Ray {
            origin: Point3d::new([0.0, 0.0, 0.0]),
            direction: Vector3d::new([1.0, 0.0, 0.0]),
        };
        let point = Point3d::new([1.0, 2.0, 3.0]);
        let normal = Vector3d::new([1.0, 1.0, 0.0]);
        let t = 2.0;
        let material = Material::Lambertian(Color::white());
        let hit_record = HitRecord::new(&ray, point, normal, t, material);

        assert_eq!(hit_record.point, point);
        assert_eq!(hit_record.normal, -normal);
        assert_eq!(hit_record.t, t);
        assert_eq!(hit_record.front_face, false);
    }

    #[test]
    fn test_hitrecord_eq() {
        let hit_record1 = HitRecord {
            point: Point3d::new([1.0, 5.0, 3.0]),
            normal: Vector3d::new([3.0, 1.0, 0.0]),
            t: 2.0,
            front_face: true,
            material: Material::Lambertian(Color::black())
        };
        let hit_record2 = HitRecord {
            point: Point3d::new([0.0, 2.0, 6.0]),
            normal: Vector3d::new([1.0, 4.0, 3.0]),
            t: 2.0,
            front_face: true,
            material: Material::Lambertian(Color::white())
        };

        assert_eq!(hit_record1, hit_record2);
    }

    #[test]
    fn test_hitrecord_cmp() {
        let hit_record1 = HitRecord {
            point: Point3d::new([1.0, 2.0, 3.0]),
            normal: Vector3d::new([0.0, 1.0, 0.0]),
            t: 2.0,
            front_face: true,
            material: Material::Lambertian(Color::white())
        };
        let hit_record2 = HitRecord {
            point: Point3d::new([1.0, 2.0, 3.0]),
            normal: Vector3d::new([0.0, 1.0, 0.0]),
            t: 3.0,
            front_face: true,
            material: Material::Lambertian(Color::white())
        };

        assert!(hit_record1 < hit_record2);
        assert!(hit_record2 > hit_record1);
    }
}
