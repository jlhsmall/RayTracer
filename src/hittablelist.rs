pub use crate::object::HitRecord;
pub use crate::object::Hittable;
pub use crate::ray::Ray;
pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}
impl HittableList {
    pub fn new(objects: Vec<Box<dyn Hittable>>) -> Self {
        Self { objects }
    }
}
impl Hittable for HittableList {
    fn hit(&self, r: Ray, tmin: f64, tmax: f64) -> Option<HitRecord> {
        let mut ret: Option<HitRecord> = Option::None;
        let mut closest = tmax;
        for i in self.objects.iter() {
            if let Option::Some(rec) = i.hit(r, tmin, closest) {
                closest = rec.t;
                ret = Option::Some(rec);
            }
        }
        ret
    }
}
