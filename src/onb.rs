pub use crate::vec3::Vec3;
pub struct ONB {
    pub axis: [Vec3; 3],
}
impl ONB {
    pub fn get(&self, i: usize) -> Vec3 {
        self.axis[i]
    }
    pub fn u(&self) -> Vec3 {
        self.axis[0]
    }
    pub fn v(&self) -> Vec3 {
        self.axis[1]
    }
    pub fn w(&self) -> Vec3 {
        self.axis[2]
    }
    pub fn local(&self, a: Vec3) -> Vec3 {
        self.u() * a.x + self.v() * a.y + self.w() * a.z
    }
    pub fn new_from_w(n: Vec3) -> Self {
        let axis2 = n.unit();
        let a = if axis2.x.abs() < 0.9 {
            Vec3::new(1.0, 0.0, 0.0)
        } else {
            Vec3::new(0.0, 1.0, 0.0)
        };
        let axis1 = Vec3::cross(axis2, a).unit();
        let axis0 = Vec3::cross(axis2, axis1);
        Self {
            axis: [axis0, axis1, axis2],
        }
    }
}
