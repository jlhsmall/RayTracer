pub use crate::aabb::AABB;
pub use crate::material::Material;
pub use crate::oneweekend::degree_to_radian;
pub use crate::oneweekend::INF;
pub use crate::ray::Ray;
pub use crate::vec3::Vec3;
pub use std::sync::Arc;

#[derive(Clone)]
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub mat_ptr: Arc<dyn Material>,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
}
impl HitRecord {
    pub fn new(
        r: Ray,
        p: Vec3,
        out_normal: Vec3,
        t: f64,
        uu: f64,
        vv: f64,
        mat_ptr: Arc<dyn Material>,
    ) -> Self {
        let front_face = r.dir * out_normal < 0.0;
        let normal = if front_face { out_normal } else { -out_normal };
        Self {
            p,
            normal,
            mat_ptr,
            t,
            u: uu,
            v: vv,
            front_face,
        }
    }
}
pub trait Hittable: Send + Sync {
    fn hit(&self, r: Ray, tmin: f64, tmax: f64) -> Option<HitRecord>;
    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB>;
    fn pdf_value(&self,_o:Vec3,_v:Vec3)->f64{
        0.0
    }
    fn random(&self,_o:Vec3)->Vec3{
        Vec3::new(1.0,0.0,0.0)
    }
}
pub struct Translate {
    pub ptr: Arc<dyn Hittable>,
    pub offset: Vec3,
}
impl Translate {
    pub fn new(ptr: Arc<dyn Hittable>, offset: Vec3) -> Self {
        Self { ptr, offset }
    }
}
impl Hittable for Translate {
    fn hit(&self, r: Ray, tmin: f64, tmax: f64) -> Option<HitRecord> {
        let moved_r = Ray::new(r.orig - self.offset, r.dir);
        let opt = self.ptr.hit(moved_r, tmin, tmax);
        if let Option::Some(mut rec) = opt {
            rec.p += self.offset;
            Option::Some(rec)
        } else {
            Option::None
        }
    }
    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        let opt = self.ptr.bounding_box(t0, t1);
        if let Option::Some(mut boxx) = opt {
            boxx.mi = [
                self.offset.x + boxx.mi[0],
                self.offset.y + boxx.mi[1],
                self.offset.z + boxx.mi[2],
            ];
            boxx.mx = [
                self.offset.x + boxx.mx[0],
                self.offset.y + boxx.mx[1],
                self.offset.z + boxx.mx[2],
            ];
            Option::Some(boxx)
        } else {
            Option::None
        }
    }
}
pub struct RotateY {
    pub ptr: Arc<dyn Hittable>,
    pub sin_theta: f64,
    pub cos_theta: f64,
    pub box_opt: Option<AABB>,
}
impl RotateY {
    pub fn new(ptr: Arc<dyn Hittable>, angle: f64) -> Self {
        let radian = degree_to_radian(angle);
        let sin_theta = radian.sin();
        let cos_theta = radian.cos();
        let opt = ptr.bounding_box(0.0, 1.0);
        let box_opt: Option<AABB>;
        if let Option::Some(boxx) = opt {
            let mut mi = [INF, INF, INF];
            let mut mx = [-INF, -INF, -INF];
            for i in 0..2 {
                for j in 0..2 {
                    for k in 0..2 {
                        let x = if i == 0 { boxx.mi[0] } else { boxx.mx[0] };
                        let y = if j == 0 { boxx.mi[1] } else { boxx.mx[1] };
                        let z = if k == 0 { boxx.mi[2] } else { boxx.mx[2] };
                        let newx = cos_theta * x + sin_theta * z;
                        let newz = -sin_theta * x + cos_theta * z;
                        let tester = [newx, y, newz];
                        for c in 0..3 {
                            if tester[c] < mi[c] {
                                mi[c] = tester[c];
                            }
                            if tester[c] > mx[c] {
                                mx[c] = tester[c];
                            }
                        }
                    }
                }
            }
            box_opt = Option::Some(AABB::new(
                Vec3::new(mi[0], mi[1], mi[2]),
                Vec3::new(mx[0], mx[1], mx[2]),
            ))
        } else {
            box_opt = Option::None;
        }
        Self {
            ptr,
            sin_theta,
            cos_theta,
            box_opt,
        }
    }
}
impl Hittable for RotateY {
    fn hit(&self, r: Ray, tmin: f64, tmax: f64) -> Option<HitRecord> {
        let origin = Vec3::new(
            self.cos_theta * r.orig.x - self.sin_theta * r.orig.z,
            r.orig.y,
            self.sin_theta * r.orig.x + self.cos_theta * r.orig.z,
        );
        let direction = Vec3::new(
            self.cos_theta * r.dir.x - self.sin_theta * r.dir.z,
            r.dir.y,
            self.sin_theta * r.dir.x + self.cos_theta * r.dir.z,
        );
        let rotated_r = Ray::new(origin, direction);
        let opt = self.ptr.hit(rotated_r, tmin, tmax);
        if let Option::Some(mut rec) = opt {
            rec.p = Vec3::new(
                self.cos_theta * rec.p.x + self.sin_theta * rec.p.z,
                rec.p.y,
                -self.sin_theta * rec.p.x + self.cos_theta * rec.p.z,
            );
            let out_normal = Vec3::new(
                self.cos_theta * rec.normal.x + self.sin_theta * rec.normal.z,
                rec.normal.y,
                -self.sin_theta * rec.normal.x + self.cos_theta * rec.normal.z,
            );
            let front_face = rotated_r.dir * rec.normal < 0.0;
            rec.normal = if front_face { out_normal } else { -out_normal };
            Option::Some(rec)
        } else {
            Option::None
        }
    }
    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<AABB> {
        self.box_opt
    }
}
pub struct FlipFace{
    ptr:Arc<dyn Hittable>,
}
impl FlipFace{
    pub fn new(ptr:Arc<dyn Hittable>)->Self{
        Self{
            ptr
        }
    }
}
impl Hittable for FlipFace{
    fn hit(&self, r: Ray, tmin: f64, tmax: f64) -> Option<HitRecord> {
        let opt=self.ptr.hit(r,tmin,tmax);
        if let Option::Some(mut rec)=opt{
            rec.front_face=!rec.front_face;
            Option::Some(rec)
        }
        else{
            Option::None
        }
    }
    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        self.ptr.bounding_box(t0,t1)
    }
}