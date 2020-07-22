pub use crate::vec3::Vec3;
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Sphere {
    pub centre: Vec3,
    pub radius: f64,
}
impl Sphere {
    pub fn new(centre: Vec3, radius: f64) -> Self {
        Self { centre, radius }
    }
}
