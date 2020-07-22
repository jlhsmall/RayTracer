pub use crate::sphere::Sphere;
pub use crate::vec3::Vec3;
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Ray {
    pub orig: Vec3,
    pub dir: Vec3,
}
impl Ray {
    pub fn new(orig: Vec3, dir: Vec3) -> Self {
        Self { orig, dir }
    }
    pub fn at(&self, t: f64) -> Vec3 {
        self.orig + self.dir * t
    }
    pub fn hit_sphere(&self, s: Sphere) -> f64 {
        let a = self.dir.squared_length();
        let c = (self.orig - s.centre).squared_length() - s.radius * s.radius;
        let b = (self.orig - s.centre) * self.dir * 2.0;
        let delta = b * b - 4.0 * a * c;
        if delta < 0.0 {
            return -1.0;
        }
        (-b - delta.sqrt()) / a / 2.0
    }
}
