pub use crate::object::HitRecord;
pub use crate::object::Hittable;
pub use crate::ray::Ray;
pub use crate::vec3::Vec3;
#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct Sphere {
    pub centre: Vec3,
    pub radius: f64,
}
impl Sphere {
    pub fn new(centre: Vec3, radius: f64) -> Self {
        Self { centre, radius }
    }
}
impl Hittable for Sphere {
    fn hit(&self, r: Ray, tmin: f64, tmax: f64) -> Option<HitRecord> {
        let co_a = r.dir.squared_length();
        let co_c = (r.orig - self.centre).squared_length() - self.radius * self.radius;
        let co_b = (r.orig - self.centre) * r.dir * 2.0;
        let delta = co_b * co_b - 4.0 * co_a * co_c;
        if delta < 0.0 {
            return Option::None;
        }
        let t1 = (-co_b - delta.sqrt()) / co_a / 2.0;
        if t1 >= tmin && t1 <= tmax {
            let p = r.at(t1);
            return Option::Some(HitRecord::new(p, (p - self.centre).unit(), t1));
        }
        let t2 = (-co_b + delta.sqrt()) / co_a / 2.0;
        if t2 >= tmin && t2 <= tmax {
            let p = r.at(t2);
            return Option::Some(HitRecord::new(p, (p - self.centre).unit(), t2));
        }
        Option::None
    }
}
