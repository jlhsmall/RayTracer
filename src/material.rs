pub use crate::vec3::Vec3;
pub use crate::ray::Ray;
pub use crate::object::HitRecord;
pub use crate::oneweekend::rand_unit_vector;
pub use crate::vec3::reflect;
pub use crate::vec3::refract;
pub use crate::oneweekend::rand_in_unit_sphere;

#[derive(Clone)]
pub struct ScatterRecord{
    pub attenuation:Vec3,
    pub scattered:Ray,
}
impl ScatterRecord{
    pub fn new(attenuation:Vec3,scattered:Ray)->Self{
        Self{
            attenuation,
            scattered
        }
    }
}
pub trait Material{
    fn scatter(&self,r:Ray,rec:HitRecord)->Option<ScatterRecord>;
}
pub struct Lamertian{
    pub albedo:Vec3,
}
impl Lamertian{
    pub fn new(albedo:Vec3)->Self{
        Self{
            albedo,
        }
    }
}
impl Material for Lamertian{
    fn scatter(&self,_r:Ray,rec:HitRecord)->Option<ScatterRecord>{
        let scatter_direction = rec.normal+rand_unit_vector();
        Option::Some(ScatterRecord::new(self.albedo,Ray::new(rec.p,scatter_direction)))
    }
}
pub struct Metal{
    pub albedo:Vec3,
    pub fuzz:f64,
}
impl Metal{
    pub fn new(albedo:Vec3,fuzz:f64)->Self{
        Self{
            albedo,
            fuzz:if fuzz<1.0{fuzz}else{1.0}
        }
    }
}
impl Material for Metal{
    fn scatter(&self,r:Ray,rec:HitRecord)->Option<ScatterRecord>{
        let reflected=reflect(r.dir.unit(),rec.normal);
        let scattered=Ray::new(rec.p,reflected+rand_in_unit_sphere()*self.fuzz);
        if scattered.dir*rec.normal<0.0{
            return Option::None;
        }
        Option::Some(ScatterRecord::new(self.albedo,scattered))
    }
}
pub struct Dielectric{
    pub ref_idx:f64,
}
impl Dielectric{
    pub fn new(ref_idx:f64)->Self{
        Self{
            ref_idx
        }
    }
}
impl Material for Dielectric{
    fn scatter(&self,r:Ray,rec:HitRecord)->Option<ScatterRecord>{
        let attenuation=Vec3::ones();
        let eta_i_over_t=if rec.front_face{1.0/self.ref_idx}else{self.ref_idx};
        let uv=r.dir.unit();
        let mut cos_theta:f64=-uv*rec.normal;
        if cos_theta>1.0{cos_theta=1.0;}
        let sin_theta=(1.0-cos_theta*cos_theta).sqrt();
        let scattered:Ray;
        if eta_i_over_t*sin_theta>1.0{
            let reflected=reflect(uv,rec.normal);
            scattered=Ray::new(rec.p,reflected);
        }
        else{
            let refracted=refract(uv,rec.normal,eta_i_over_t);
            scattered=Ray::new(rec.p,refracted);
        }
        Option::Some(ScatterRecord::new(attenuation,scattered))
    }
}