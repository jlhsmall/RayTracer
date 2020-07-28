pub use std::f64::consts::PI;
pub const INF: f64 = f64::INFINITY;

use crate::vec3::Vec3;
use rand::prelude::*;
pub fn degree_to_radian(degree: f64) -> f64 {
    degree * PI / 180.0
}
/*pub fn rand_int(low: i32, high: i32) -> i32 {
    let mut rng = rand::thread_rng();
    let x: i32 = rng.gen();
    x % (high - low + 1) + low
}*/
pub fn rand_double(low: f64, high: f64) -> f64 {
    let mut rng = rand::thread_rng();
    let x: f64 = rng.gen();
    x * (high - low) + low
}
pub fn rand_vector(low: f64, high: f64) -> Vec3 {
    Vec3::new(
        rand_double(low, high),
        rand_double(low, high),
        rand_double(low, high),
    )
}
pub fn rand_unit_vector() -> Vec3 {
    let a = rand_double(0.0, 2.0 * PI);
    let z = rand_double(-1.0, 1.0);
    let r = (1.0 - z * z).sqrt();
    Vec3::new(a.cos() * r, a.sin() * r, z)
}
pub fn rand_in_unit_sphere() -> Vec3 {
    let mut x: f64 = rand_double(-1.0, 1.0);
    let mut y: f64 = rand_double(-1.0, 1.0);
    let mut z: f64 = rand_double(-1.0, 1.0);
    while x * x + y * y + z * z > 1.0 {
        x = rand_double(-1.0, 1.0);
        y = rand_double(-1.0, 1.0);
        z = rand_double(-1.0, 1.0);
    }
    Vec3::new(x, y, z)
}
pub fn rand_in_unit_disk() -> Vec3 {
    let mut x: f64 = rand_double(-1.0, 1.0);
    let mut y: f64 = rand_double(-1.0, 1.0);
    while x * x + y * y > 1.0 {
        x = rand_double(-1.0, 1.0);
        y = rand_double(-1.0, 1.0);
    }
    Vec3::new(x, y, 0.0)
}
pub fn get_min(x: f64, y: f64) -> f64 {
    if x < y {
        x
    } else {
        y
    }
}
pub fn get_max(x: f64, y: f64) -> f64 {
    if x > y {
        x
    } else {
        y
    }
}
/*
pub fn clamp(x:f64,low:f64,high:f64)->f64{
    if x<low{low}
    else if x>high{high}
    else {x}
}*/
