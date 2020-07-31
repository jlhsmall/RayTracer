pub use crate::aabb::AABB;
pub use crate::material::Material;
pub use crate::object::{HitRecord, Hittable};
pub use crate::ray::Ray;
pub use crate::vec3::Vec3;
pub use std::sync::Arc;
pub struct XYRect {
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    k: f64,
    mp: Arc<dyn Material>,
}
impl XYRect {
    pub fn new(x0: f64, x1: f64, y0: f64, y1: f64, k: f64, mp: Arc<dyn Material>) -> Self {
        Self {
            x0,
            x1,
            y0,
            y1,
            k,
            mp,
        }
    }
}
impl Hittable for XYRect {
    fn hit(&self, r: Ray, tmin: f64, tmax: f64) -> Option<HitRecord> {
        let t = (self.k - r.orig.z) / r.dir.z;
        if t < tmin || t > tmax {
            return Option::None;
        }
        let xx = r.orig.x + r.dir.x * t;
        let yy = r.orig.y + r.dir.y * t;
        if xx < self.x0 || xx > self.x1 || yy < self.y0 || yy > self.y1 {
            return Option::None;
        }
        let uu = (xx - self.x0) / (self.x1 - self.x0);
        let vv = (yy - self.y0) / (self.y1 - self.y0);
        let out_normal = Vec3::new(0.0, 0.0, 1.0);
        let p = r.at(t);
        Option::Some(HitRecord::new(r, p, out_normal, t, uu, vv, self.mp.clone()))
    }
    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<AABB> {
        Option::Some(AABB::new(
            Vec3::new(self.x0, self.y0, self.k - 0.0001),
            Vec3::new(self.x1, self.y1, self.k + 0.0001),
        ))
    }
}
pub struct YZRect {
    y0: f64,
    y1: f64,
    z0: f64,
    z1: f64,
    k: f64,
    mp: Arc<dyn Material>,
}
impl YZRect {
    pub fn new(y0: f64, y1: f64, z0: f64, z1: f64, k: f64, mp: Arc<dyn Material>) -> Self {
        Self {
            y0,
            y1,
            z0,
            z1,
            k,
            mp,
        }
    }
}
impl Hittable for YZRect {
    fn hit(&self, r: Ray, tmin: f64, tmax: f64) -> Option<HitRecord> {
        let t = (self.k - r.orig.x) / r.dir.x;
        if t < tmin || t > tmax {
            return Option::None;
        }
        let yy = r.orig.y + r.dir.y * t;
        let zz = r.orig.z + r.dir.z * t;
        if yy < self.y0 || yy > self.y1 || zz < self.z0 || zz > self.z1 {
            return Option::None;
        }
        let uu = (yy - self.y0) / (self.y1 - self.y0);
        let vv = (zz - self.z0) / (self.z1 - self.z0);
        let out_normal = Vec3::new(1.0, 0.0, 0.0);
        let p = r.at(t);
        Option::Some(HitRecord::new(r, p, out_normal, t, uu, vv, self.mp.clone()))
    }
    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<AABB> {
        Option::Some(AABB::new(
            Vec3::new(self.k - 0.0001, self.y0, self.z0),
            Vec3::new(self.k + 0.0001, self.y1, self.z1),
        ))
    }
}
pub struct XZRect {
    x0: f64,
    x1: f64,
    z0: f64,
    z1: f64,
    k: f64,
    mp: Arc<dyn Material>,
}
impl XZRect {
    pub fn new(x0: f64, x1: f64, z0: f64, z1: f64, k: f64, mp: Arc<dyn Material>) -> Self {
        Self {
            x0,
            x1,
            z0,
            z1,
            k,
            mp,
        }
    }
}
impl Hittable for XZRect {
    fn hit(&self, r: Ray, tmin: f64, tmax: f64) -> Option<HitRecord> {
        let t = (self.k - r.orig.y) / r.dir.y;
        if t < tmin || t > tmax {
            return Option::None;
        }
        let xx = r.orig.x + r.dir.x * t;
        let zz = r.orig.z + r.dir.z * t;
        if xx < self.x0 || xx > self.x1 || zz < self.z0 || zz > self.z1 {
            return Option::None;
        }
        let uu = (xx - self.x0) / (self.x1 - self.x0);
        let vv = (zz - self.z0) / (self.z1 - self.z0);
        let out_normal = Vec3::new(0.0, 1.0, 0.0);
        let p = r.at(t);
        Option::Some(HitRecord::new(r, p, out_normal, t, uu, vv, self.mp.clone()))
    }
    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<AABB> {
        Option::Some(AABB::new(
            Vec3::new(self.x0, self.k - 0.0001, self.z0),
            Vec3::new(self.x1, self.k + 0.0001, self.z1),
        ))
    }
}
