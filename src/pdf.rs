pub use crate::vec3::Vec3;
pub use crate::onb::ONB;
pub use std::f64::consts::PI;
pub use crate::oneweekend::rand_cosine_direction;
pub use std::sync::Arc;
pub use crate::object::Hittable;
pub trait PDF: Send + Sync{
    fn value(&self,direction:Vec3)->f64;
    fn generate(&self)->Vec3;
}
pub struct CosinePDF{
    pub uvw:ONB,
}
impl CosinePDF{
    pub fn new(w:Vec3)->Self{
        Self{
            uvw:ONB::new_from_w(w)
        }
    }
}
impl PDF for CosinePDF{
    fn value(&self,direction:Vec3)->f64{
        let cosine=direction.unit()*self.uvw.w();
        if cosine<0.0{0.0}else{cosine/PI}
    }
    fn generate(&self)->Vec3{
        self.uvw.local(rand_cosine_direction())
    }
}
pub struct HittablePDF{
    o:Vec3,
    ptr:Arc<dyn Hittable>,
}
impl HittablePDF{
    pub fn new(ptr:Arc<dyn Hittable>,o:Vec3)->Self{
        Self{
            o,
            ptr,
        }
    }
}
impl PDF for HittablePDF{
    fn value(&self,direction:Vec3)->f64{
        self.ptr.pdf_value(self.o,direction)
    }
    fn generate(&self)->Vec3{
        self.ptr.random(self.o)
    }
}