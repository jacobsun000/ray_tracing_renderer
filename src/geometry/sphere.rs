use crate::geometry::hitable::{HitRecord, Hitable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vector3::Point3d;

pub struct Sphere {
    pub center: Point3d,
    pub radius: f64,
    pub material: Material
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
        if t < 0.001 || 1e10 < t {
            t = (-half_b + sqrtd) / a;
            if t < 0.0 || 1e10 < t {
                return None;
            }
        }
        let point = ray.at(t);
        let normal = (point - self.center) / self.radius;
        let material = self.material;
        return Some(HitRecord::new(ray, point, normal, t, material));
    }
}
