pub use crate::aabb::AABB;
pub use crate::object::{HitRecord, Hittable};
pub use crate::oneweekend::rand_int;
pub use crate::ray::Ray;
pub use crate::vec3::Vec3;
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
    fn pdf_value(&self, o: Vec3, v: Vec3) -> f64 {
        let weight = 1.0 / (self.objects.len() as f64);
        let mut sum = 0.0;
        for i in self.objects.iter() {
            sum += i.pdf_value(o, v);
        }
        sum * weight
    }
    fn random(&self, o: Vec3) -> Vec3 {
        let sz = self.objects.len() as i32;
        self.objects[rand_int(0, sz - 1) as usize].random(o)
    }
}
