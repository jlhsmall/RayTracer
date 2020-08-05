pub use crate::object::Hittable;
pub use crate::onb::ONB;
pub use crate::oneweekend::rand_cosine_direction;
use crate::rand_double;
pub use crate::vec3::Vec3;
pub use std::f64::consts::PI;
pub use std::sync::Arc;

pub trait PDF: Send + Sync {
    fn value(&self, direction: Vec3) -> f64;
    fn generate(&self) -> Vec3;
}
pub struct CosinePDF {
    pub uvw: ONB,
}
impl CosinePDF {
    pub fn new(w: Vec3) -> Self {
        Self {
            uvw: ONB::new_from_w(w),
        }
    }
}
impl PDF for CosinePDF {
    fn value(&self, direction: Vec3) -> f64 {
        let cosine = direction.unit() * self.uvw.w();
        if cosine <= 0.0 {
            0.0
        } else {
            cosine / PI
        }
    }
    fn generate(&self) -> Vec3 {
        self.uvw.local(rand_cosine_direction())
    }
}
pub struct HittablePDF {
    o: Vec3,
    ptr: Arc<dyn Hittable>,
}
impl HittablePDF {
    pub fn new(ptr: Arc<dyn Hittable>, o: Vec3) -> Self {
        Self { o, ptr }
    }
}
impl PDF for HittablePDF {
    fn value(&self, direction: Vec3) -> f64 {
        self.ptr.pdf_value(self.o, direction)
    }
    fn generate(&self) -> Vec3 {
        self.ptr.random(self.o)
    }
}
pub struct MixturePDF {
    p: [Arc<dyn PDF>; 2],
}
impl MixturePDF {
    pub fn new(p0: Arc<dyn PDF>, p1: Arc<dyn PDF>) -> Self {
        Self { p: [p0, p1] }
    }
}
impl PDF for MixturePDF {
    fn value(&self, direction: Vec3) -> f64 {
        (self.p[0].value(direction) + self.p[1].value(direction)) / 2.0
    }
    fn generate(&self) -> Vec3 {
        if rand_double(0.0, 1.0) < 0.5 {
            self.p[0].generate()
        } else {
            self.p[1].generate()
        }
    }
}
