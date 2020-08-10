pub use crate::oneweekend::{rand_double, rand_int, rand_vector};
pub use crate::vec3::Vec3;
const POINT_COUNT: usize = 256;
#[derive(Clone)]
pub struct Perlin {
    ranvec: Vec<Vec3>,
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
    fn perlin_generate_ranvec() -> Vec<Vec3> {
        let mut ret: Vec<Vec3> = Vec::new();
        for _i in 0..POINT_COUNT {
            ret.push(rand_vector(-1.0, 1.0).unit());
        }
        ret
    }
    pub fn new() -> Self {
        Self {
            ranvec: Perlin::perlin_generate_ranvec(),
            perm_x: Perlin::perlin_generate_perm(),
            perm_y: Perlin::perlin_generate_perm(),
            perm_z: Perlin::perlin_generate_perm(),
        }
    }
    pub fn noise(&self, p: Vec3) -> f64 {
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        let xx = p.x.floor() as i32;
        let yy = p.y.floor() as i32;
        let zz = p.z.floor() as i32;
        let mut accum = 0.0;
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let ii = di as f64;
                    let jj = dj as f64;
                    let kk = dk as f64;
                    let weight_v = Vec3::new(u - ii, v - jj, w - kk);
                    let cc = self.ranvec[(self.perm_x[((xx + di) & 255) as usize]
                        ^ self.perm_y[((yy + dj) & 255) as usize]
                        ^ self.perm_z[((zz + dk) & 255) as usize])
                        as usize];
                    accum += cc
                        * weight_v
                        * (ii * uu + (1.0 - ii) * (1.0 - uu))
                        * (jj * vv + (1.0 - jj) * (1.0 - vv))
                        * (kk * ww + (1.0 - kk) * (1.0 - ww));
                }
            }
        }
        accum
    }
    pub fn turb(&self, mut p: Vec3, depth: i32) -> f64 {
        let mut accum = 0.0;
        let mut weight = 1.0;
        for _i in 0..depth {
            accum += weight * self.noise(p);
            weight *= 0.5;
            p *= 2.0;
        }
        accum.abs()
    }
}
