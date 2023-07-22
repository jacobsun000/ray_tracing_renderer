use crate::geometry::HitRecord;
use crate::Color;
use crate::Ray;
use crate::material::Lambertian;

#[derive(Clone, Copy, Debug)]
pub enum Material {
    Lambertian(Color),
}

impl Material {
    pub fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        match self {
            Material::Lambertian(albedo) => Lambertian::scatter(albedo, rec),

        }
    }
}
