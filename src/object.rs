pub use crate::material::Material;
pub use crate::ray::Ray;
pub use crate::vec3::Vec3;
pub use std::sync::Arc;
#[derive(Clone)]
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub mat_ptr: Arc<dyn Material>,
    pub t: f64,
    pub front_face: bool,
}
impl HitRecord {
    pub fn new(r: Ray, p: Vec3, out_normal: Vec3, t: f64, mat_ptr: Arc<dyn Material>) -> Self {
        let front_face = r.dir * out_normal < 0.0;
        let normal = if front_face { out_normal } else { -out_normal };
        Self {
            p,
            normal,
            mat_ptr,
            t,
            front_face,
        }
    }
}
pub trait Hittable {
    fn hit(&self, r: Ray, tmin: f64, tmax: f64) -> Option<HitRecord>;
}
