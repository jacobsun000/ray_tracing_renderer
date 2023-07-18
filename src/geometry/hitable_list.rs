use crate::geometry::hitable::{HitRecord, Hitable};
use crate::ray::Ray;

pub struct HitableList {
    pub hitables: std::vec::Vec<Box<dyn Hitable + Sync>>,
}

impl Hitable for HitableList {
    fn hit(&self, r: &Ray) -> Option<HitRecord> {
        self.hitables
            .iter()
            .map(|a| a.hit(r))
            .filter(|x| x.is_some())
            .min()
            .flatten()
    }
}
