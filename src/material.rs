pub use crate::object::HitRecord;
pub use crate::onb::ONB;
pub use crate::oneweekend::{
    rand_cosine_direction, rand_double, /*rand_in_hemisphere,*/ rand_in_unit_sphere,
    rand_unit_vector,
};
pub use crate::pdf::{CosinePDF, PDF};
pub use crate::ray::Ray;
use crate::texture::SolidColor;
pub use crate::texture::Texture;
pub use crate::vec3::reflect;
pub use crate::vec3::refract;
pub use crate::vec3::Vec3;
pub use std::f64::consts::PI;
pub use std::sync::Arc;
#[derive(Clone)]
pub enum ScatterType {
    Specular(Ray),
    Pdf(Arc<dyn PDF>),
}
#[derive(Clone)]
pub struct ScatterRecord {
    pub attenuation: Vec3,
    pub tp: ScatterType,
}
impl ScatterRecord {
    pub fn new(attenuation: Vec3, tp: ScatterType) -> Self {
        Self { attenuation, tp }
    }
}
pub trait Material: Send + Sync {
    fn emitted(&self, _r: Ray, _rec: HitRecord, _u: f64, _v: f64, _p: Vec3) -> Vec3 {
        Vec3::new(0.0, 0.0, 0.0)
    }
    fn scatter(&self, _r: Ray, _rec: HitRecord) -> Option<ScatterRecord> {
        Option::None
    }
    fn scattering_pdf(&self, _r: Ray, _rec: HitRecord, _scattered: Ray) -> f64 {
        0.0
    }
}
#[derive(Default)]
pub struct NoMaterial;
impl Material for NoMaterial {}
pub struct Lamertian {
    pub albedo: Arc<dyn Texture>,
}
impl Lamertian {
    pub fn new(albedo: Vec3) -> Self {
        Self {
            albedo: Arc::new(SolidColor::new(albedo)),
        }
    }
    pub fn newa(albedo: Arc<dyn Texture>) -> Self {
        Self { albedo }
    }
}
impl Material for Lamertian {
    fn scatter(&self, _r: Ray, rec: HitRecord) -> Option<ScatterRecord> {
        Option::Some(ScatterRecord::new(
            self.albedo.value(rec.u, rec.v, rec.p),
            ScatterType::Pdf(Arc::new(CosinePDF::new(rec.normal))),
        ))
    }
    fn scattering_pdf(&self, _r: Ray, rec: HitRecord, scattered: Ray) -> f64 {
        let cosine = rec.normal * scattered.dir.unit();
        if cosine < 0.0 {
            0.0
        } else {
            cosine / PI
        }
    }
}
pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f64,
}
impl Metal {
    pub fn new(albedo: Vec3, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}
impl Material for Metal {
    fn scatter(&self, r: Ray, rec: HitRecord) -> Option<ScatterRecord> {
        let reflected = reflect(r.dir.unit(), rec.normal);
        Option::Some(ScatterRecord::new(
            self.albedo,
            ScatterType::Specular(Ray::new(
                rec.p,
                reflected + rand_in_unit_sphere() * self.fuzz,
            )),
        ))
    }
}
fn schlick(cosine: f64, ref_idx: f64) -> f64 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * ((1.0 - cosine).powf(5.0))
}
pub struct Dielectric {
    pub ref_idx: f64,
}
impl Dielectric {
    pub fn new(ref_idx: f64) -> Self {
        Self { ref_idx }
    }
}
impl Material for Dielectric {
    fn scatter(&self, r: Ray, rec: HitRecord) -> Option<ScatterRecord> {
        let attenuation = Vec3::ones();
        let eta_i_over_t = if rec.front_face {
            1.0 / self.ref_idx
        } else {
            self.ref_idx
        };
        let uv = r.dir.unit();
        let mut cos_theta: f64 = -uv * rec.normal;
        if cos_theta > 1.0 {
            cos_theta = 1.0;
        }
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let scattered: Ray;
        if eta_i_over_t * sin_theta > 1.0 {
            let reflected = reflect(uv, rec.normal);
            scattered = Ray::new(rec.p, reflected);
        } else {
            let reflect_prob = schlick(cos_theta, eta_i_over_t);
            if rand_double(0.0, 1.0) < reflect_prob {
                let reflected = reflect(uv, rec.normal);
                scattered = Ray::new(rec.p, reflected);
            } else {
                let refracted = refract(uv, rec.normal, eta_i_over_t);
                scattered = Ray::new(rec.p, refracted);
            }
        }
        Option::Some(ScatterRecord::new(
            attenuation,
            ScatterType::Specular(scattered),
        ))
    }
}
pub struct DiffuseLight {
    pub emit: Arc<dyn Texture>,
}
impl DiffuseLight {
    pub fn new(c: Vec3) -> Self {
        Self {
            emit: Arc::new(SolidColor::new(c)),
        }
    }
    pub fn newt(emit: Arc<dyn Texture>) -> Self {
        Self { emit }
    }
}
impl Material for DiffuseLight {
    fn emitted(&self, _r: Ray, rec: HitRecord, u: f64, v: f64, p: Vec3) -> Vec3 {
        if rec.front_face {
            self.emit.value(u, v, p)
        } else {
            Vec3::zero()
        }
    }
    fn scatter(&self, _r: Ray, _rec: HitRecord) -> Option<ScatterRecord> {
        Option::None
    }
}
