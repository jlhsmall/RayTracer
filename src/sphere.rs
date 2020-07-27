pub use crate::material::Material;
pub use crate::object::HitRecord;
pub use crate::object::Hittable;
pub use crate::ray::Ray;
pub use crate::vec3::Vec3;
pub use std::sync::Arc;
pub struct Sphere {
    pub centre: Vec3,
    pub radius: f64,
    pub mat_ptr: Arc<dyn Material>,
}
impl Sphere {
    pub fn new(centre: Vec3, radius: f64, mat_ptr: Arc<dyn Material>) -> Self {
        Self {
            centre,
            radius,
            mat_ptr: mat_ptr.clone(),
        }
    }
}
impl Hittable for Sphere {
    fn hit(&self, r: Ray, tmin: f64, tmax: f64) -> Option<HitRecord> {
        let co_a = r.dir.squared_length();
        let co_c = (r.orig - self.centre).squared_length() - self.radius * self.radius;
        let co_bd2 = (r.orig - self.centre) * r.dir;
        let deltad4 = co_bd2 * co_bd2 - co_a * co_c;
        if deltad4 < 0.0 {
            return Option::None;
        }
        let t1 = (-co_bd2 - deltad4.sqrt()) / co_a;
        if t1 >= tmin && t1 <= tmax {
            let p = r.at(t1);
            return Option::Some(HitRecord::new(
                r,
                p,
                (p - self.centre)/self.radius,
                t1,
                self.mat_ptr.clone(),
            ));
        }
        let t2 = (-co_bd2 + deltad4.sqrt()) / co_a;
        if t2 >= tmin && t2 <= tmax {
            let p = r.at(t2);
            return Option::Some(HitRecord::new(
                r,
                p,
                (p - self.centre)/self.radius,
                t2,
                self.mat_ptr.clone(),
            ));
        }
        Option::None
    }
}
