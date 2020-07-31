pub use crate::aabb::AABB;
pub use crate::hittablelist::HittableList;
pub use crate::material::Material;
pub use crate::object::{HitRecord, Hittable};
pub use crate::ray::Ray;
pub use crate::vec3::Vec3;
pub use crate::{XYRect, XZRect, YZRect};
pub use std::sync::Arc;
pub struct CBox {
    pub box_min: Vec3,
    pub box_max: Vec3,
    pub sides: HittableList,
}
impl CBox {
    pub fn new(p0: Vec3, p1: Vec3, ptr: Arc<dyn Material>) -> Self {
        let mut objects: Vec<Arc<dyn Hittable>> = Vec::new();
        objects.push(Arc::new(XYRect::new(
            p0.x,
            p1.x,
            p0.y,
            p1.y,
            p1.z,
            ptr.clone(),
        )));
        objects.push(Arc::new(XYRect::new(
            p0.x,
            p1.x,
            p0.y,
            p1.y,
            p0.z,
            ptr.clone(),
        )));
        objects.push(Arc::new(XZRect::new(
            p0.x,
            p1.x,
            p0.z,
            p1.z,
            p1.y,
            ptr.clone(),
        )));
        objects.push(Arc::new(XZRect::new(
            p0.x,
            p1.x,
            p0.z,
            p1.z,
            p0.y,
            ptr.clone(),
        )));
        objects.push(Arc::new(YZRect::new(
            p0.y,
            p1.y,
            p0.z,
            p1.z,
            p1.x,
            ptr.clone(),
        )));
        objects.push(Arc::new(YZRect::new(
            p0.y,
            p1.y,
            p0.z,
            p1.z,
            p0.x,
            ptr.clone(),
        )));
        Self {
            box_min: p0,
            box_max: p1,
            sides: HittableList::new(objects),
        }
    }
}

impl Hittable for CBox {
    fn hit(&self, r: Ray, tmin: f64, tmax: f64) -> Option<HitRecord> {
        self.sides.hit(r, tmin, tmax)
    }
    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<AABB> {
        Option::Some(AABB::new(self.box_min, self.box_max))
    }
}
