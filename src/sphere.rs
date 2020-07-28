pub use crate::aabb::AABB;
pub use crate::material::Material;
pub use crate::object::HitRecord;
pub use crate::object::Hittable;
pub use crate::ray::Ray;
pub use crate::vec3::Vec3;
pub use std::f64::consts::PI;
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
pub fn get_sphere_uv(p: Vec3) -> (f64, f64) {
    let phi = p.z.atan2(p.x);
    let theta = p.y.asin();
    let u = 1.0 - (PI + phi) / (2.0 * PI);
    let v = (theta + PI / 2.0) / PI;
    (u, v)
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
            let (u, v) = get_sphere_uv(p - self.centre);
            return Option::Some(HitRecord::new(
                r,
                p,
                (p - self.centre) / self.radius,
                t1,
                u,
                v,
                self.mat_ptr.clone(),
            ));
        }
        let t2 = (-co_bd2 + deltad4.sqrt()) / co_a;
        if t2 >= tmin && t2 <= tmax {
            let p = r.at(t2);
            let (u, v) = get_sphere_uv(p - self.centre);
            return Option::Some(HitRecord::new(
                r,
                p,
                (p - self.centre) / self.radius,
                t2,
                u,
                v,
                self.mat_ptr.clone(),
            ));
        }
        Option::None
    }
    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<AABB> {
        let tmp = Vec3::ones() * self.radius;
        Option::Some(AABB::new(self.centre - tmp, self.centre + tmp))
    }
}
