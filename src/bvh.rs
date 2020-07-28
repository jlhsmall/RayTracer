pub use crate::aabb::AABB;
pub use crate::object::HitRecord;
pub use crate::object::Hittable;
pub use crate::oneweekend::rand_int;
pub use crate::ray::Ray;
pub use std::sync::Arc;

pub struct BVHNode {
    pub left: Arc<dyn Hittable>,
    pub right: Arc<dyn Hittable>,
    pub bbox: AABB,
}
impl BVHNode {
    pub fn new(mut objects: Vec<Arc<dyn Hittable>>, span: usize, time0: f64, time1: f64) -> Self {
        let axis = rand_int(0, 2) as usize;
        let left: Arc<dyn Hittable>;
        let right: Arc<dyn Hittable>;
        if span == 1 {
            left = objects.remove(0);
            right = left.clone();
        } else if span == 2 {
            left = objects.remove(0);
            right = objects.remove(1);
        } else {
            objects.sort_by(|a, b| {
                let x = a.bounding_box(time0, time1).unwrap().mi[axis];
                let y = b.bounding_box(time0, time1).unwrap().mi[axis];
                x.partial_cmp(&y).unwrap()
            });
            let mid = span / 2;
            let (objects1, objects2) = objects.split_at_mut(mid);
            left = Arc::new(BVHNode::new(objects1.to_vec(), mid, time0, time1));
            right = Arc::new(BVHNode::new(objects2.to_vec(), mid, time0, time1));
        }
        let box1 = left.bounding_box(time0, time1).unwrap();
        let box2 = right.bounding_box(time0, time1).unwrap();
        Self {
            left,
            right,
            bbox: AABB::surrounding_box(box1, box2),
        }
    }
}
impl Hittable for BVHNode {
    fn hit(&self, r: Ray, tmin: f64, tmax: f64) -> Option<HitRecord> {
        if self.bbox.hit(r, tmin, tmax) {
            Option::None
        } else {
            let hit_left = self.left.hit(r, tmin, tmax);
            let hit_right = self.right.hit(r, tmin, tmax);
            if hit_left.is_some() {
                hit_left
            } else {
                hit_right
            }
        }
    }
    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<AABB> {
        Option::Some(self.bbox)
    }
}
