pub use crate::ray::Ray;
pub use crate::vec3::Vec3;
#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64,
}
impl HitRecord {
    pub fn new(p: Vec3, normal: Vec3, t: f64) -> Self {
        Self { p, normal, t }
    }
}
pub trait Hittable {
    fn hit(&self, r: Ray, tmin: f64, tmax: f64) -> Option<HitRecord>;
}
