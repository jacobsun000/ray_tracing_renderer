use crate::geometry::hitable::{HitRecord, Hitable};
use crate::ray::Ray;
use crate::vector3::Point3d;

pub struct Sphere {
    pub center: Point3d,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: Point3d, radius: f64) -> Self {
        Sphere { center, radius }
    }
}

impl Hitable for Sphere {
    fn hit(&self, ray: &Ray) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(&ray.direction);
        let c = oc.length_squared() - self.radius.powi(2);
        let discriminant = half_b.powi(2) - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut t = (-half_b - sqrtd) / a;
        if t < 0.0 || 1e10 < t {
            t = (-half_b + sqrtd) / a;
            if t < 0.0 || 1e10 < t {
                return None;
            }
        }
        let point = ray.at(t);
        let normal = (point - self.center) / self.radius;
        return Some(HitRecord::new(ray, point, normal, t));
    }
}
