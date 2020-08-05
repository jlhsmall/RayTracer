pub use crate::aabb::AABB;
pub use crate::object::{HitRecord, Hittable};
pub use crate::ray::Ray;
pub use std::sync::Arc;
pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
}
impl HittableList {
    pub fn new(objects: Vec<Arc<dyn Hittable>>) -> Self {
        Self { objects }
    }
    pub fn add(&mut self, x: Arc<dyn Hittable>) {
        self.objects.push(x);
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
    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        let mut output_box: Option<AABB> = Option::None;
        for i in self.objects.iter() {
            let tmp_box = i.bounding_box(t0, t1);
            if let Option::Some(box1) = tmp_box {
                if let Option::Some(box2) = output_box {
                    output_box = Option::Some(AABB::surrounding_box(box1, box2));
                } else {
                    output_box = tmp_box;
                }
            } else {
                return Option::None;
            }
        }
        output_box
    }
}
