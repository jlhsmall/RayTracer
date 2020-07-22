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
    pub fn hit_sphere(&self, s: Sphere) -> bool {
        let a = self.dir.squared_length();
        let c = (s.centre - self.orig).squared_length() - s.radius * s.radius;
        let b = (s.centre - self.orig) * self.dir * 2.0;
        b * b > 4.0 * a * c
    }
}
