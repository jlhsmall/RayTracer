pub use crate::oneweekend::{rand_double, rand_int};
pub use crate::vec3::Vec3;
const POINT_COUNT: usize = 256;
#[derive(Clone)]
pub struct Perlin {
    ranfloat: Vec<f64>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
}
impl Perlin {
    fn perlin_generate_perm() -> Vec<usize> {
        let mut ret: Vec<usize> = Vec::new();
        for i in 0..POINT_COUNT {
            ret.push(i);
        }
        let mut i = (POINT_COUNT as i32) - 1;
        while i >= 0 {
            let target = rand_int(0, i) as usize;
            ret.swap(i as usize, target);
            i -= 1;
        }
        ret
    }
    fn perlin_generate_ranfloat() -> Vec<f64> {
        let mut ret: Vec<f64> = Vec::new();
        for _i in 0..POINT_COUNT {
            ret.push(rand_double(0.0, 1.0));
        }
        ret
    }
    pub fn new() -> Self {
        Self {
            ranfloat: Perlin::perlin_generate_ranfloat(),
            perm_x: Perlin::perlin_generate_perm(),
            perm_y: Perlin::perlin_generate_perm(),
            perm_z: Perlin::perlin_generate_perm(),
        }
    }
    pub fn noise(&self, p: Vec3) -> f64 {
        let mut u = p.x - p.x.floor();
        let mut v = p.y - p.y.floor();
        let mut w = p.z - p.z.floor();
        u=u*u*(3.0-2.0*u);
        v=v*v*(3.0-2.0*v);
        w=w*w*(3.0-2.0*w);
        let xx = p.x.floor() as usize;
        let yy = p.y.floor() as usize;
        let zz = p.z.floor() as usize;
        let mut accum=0.0;
        for di in 0..2{
            for dj in 0..2{
                for dk in 0..2{
                    let c=self.ranfloat[(self.perm_x[((xx+di)&255)as usize]^self.perm_y[((yy+dj)&255)as usize]^self.perm_z[((zz+dk)&255)as usize])as usize];
                    let ii=di as f64;
                    let jj=dj as f64;
                    let kk=dk as f64;
                    accum+=(ii*u+(1.0-ii)*(1.0-u))*(jj*v+(1.0-jj)*(1.0-v))*(kk*w+(1.0-kk)*(1.0-w))*c;
                }
            }
        }
        accum
    }
}
