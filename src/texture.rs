pub use crate::vec3::Vec3;
pub use std::sync::Arc;
pub trait Texture :Send + Sync {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Vec3;
}
pub struct SolidColor {
    pub color_value: Vec3,
}
impl SolidColor {
    pub fn new(color_value: Vec3) -> Self {
        Self { color_value }
    }
}
impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: Vec3) -> Vec3 {
        self.color_value
    }
}
pub struct CheckerTexture {
    even: Arc<dyn Texture>,
    odd: Arc<dyn Texture>,
}
impl CheckerTexture {
    pub fn newa(even: Arc<dyn Texture>, odd: Arc<dyn Texture>) -> Self {
        Self { even, odd }
    }
    pub fn new(c1: Vec3, c2: Vec3) -> Self {
        Self {
            even: Arc::new(SolidColor::new(c1)),
            odd: Arc::new(SolidColor::new(c2)),
        }
    }
}
impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Vec3 {
        let sines = (10.0 * p.x).sin() * (10.0 * p.y).sin() * (10.0 * p.z).sin();
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}
