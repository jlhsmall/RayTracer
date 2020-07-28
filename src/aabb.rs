pub use crate::oneweekend::get_max;
pub use crate::oneweekend::get_min;
pub use crate::ray::Ray;
pub use crate::vec3::Vec3;
#[derive(Copy, Clone)]
pub struct AABB {
    pub mi: [f64; 3],
    pub mx: [f64; 3],
}
impl AABB {
    pub fn new(mi: Vec3, mx: Vec3) -> Self {
        Self {
            mi: mi.get_array(),
            mx: mx.get_array(),
        }
    }
    pub fn get_mi(&self) -> Vec3 {
        Vec3::new(self.mi[0], self.mi[1], self.mi[2])
    }
    pub fn get_mx(&self) -> Vec3 {
        Vec3::new(self.mx[0], self.mx[1], self.mx[2])
    }
    pub fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> bool {
        let origin = r.orig.get_array();
        let direction = r.dir.get_array();
        let mut t_mi = t_min;
        let mut t_mx = t_max;
        for a in 0..3 {
            let inv_d = 1.0 / direction[a];
            let mut t0 = (self.mi[a] - origin[a]) * inv_d;
            let mut t1 = (self.mx[a] - origin[a]) * inv_d;
            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }
            if t0 > t_mi {
                t_mi = t0;
            }
            if t1 < t_mx {
                t_mx = t1;
            }
            if t1 < t0 {
                return false;
            }
        }
        true
    }
    pub fn surrounding_box(box0: Self, box1: Self) -> Self {
        Self {
            mi: [
                get_min(box0.mi[0], box1.mi[0]),
                get_min(box0.mi[1], box1.mi[1]),
                get_min(box0.mi[2], box1.mi[2]),
            ],
            mx: [
                get_max(box0.mx[0], box1.mx[0]),
                get_min(box0.mx[1], box1.mx[1]),
                get_min(box0.mx[2], box1.mx[2]),
            ],
        }
    }
}
