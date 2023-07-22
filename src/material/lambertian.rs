use crate::{Color, geometry::HitRecord, Ray, vector3::Vector3};

pub struct Lambertian {}

impl Lambertian {
    pub fn scatter(albedo: &Color, rec: &HitRecord) -> Option<(Color, Ray)> {
        let scatter_direction = rec.normal + Vector3::random_in_unit_sphere();
        let scattered = Ray{origin: rec.point, direction: scatter_direction};
        let attenuation = albedo.clone();
        Some((attenuation, scattered))
    }
}