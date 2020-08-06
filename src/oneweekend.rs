pub use std::f64::consts::PI;
pub const INF: f64 = f64::INFINITY;

use crate::vec3::Vec3;
use rand::prelude::*;
pub fn degree_to_radian(degree: f64) -> f64 {
    degree * PI / 180.0
}
pub fn rand_double(low: f64, high: f64) -> f64 {
    let mut rng = rand::thread_rng();
    let x: f64 = rng.gen();
    x * (high - low) + low
}
pub fn rand_int(low: i32, high: i32) -> i32 {
    rand_double(low as f64, (high + 1) as f64) as i32
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
pub fn rand_to_sphere(radius: f64, distance_squared: f64) -> Vec3 {
    let r1 = rand_double(0.0, 1.0);
    let r2 = rand_double(0.0, 1.0);
    let z = 1.0 + r2 * ((1.0 - radius * radius / distance_squared).sqrt() - 1.0);
    let phi = 2.0 * PI * r1;
    let x = phi.cos() * (1.0 - z * z).sqrt();
    let y = phi.sin() * (1.0 - z * z).sqrt();
    Vec3::new(x, y, z)
}
/*  pub fn rand_in_hemisphere(normal: Vec3) -> Vec3 {
    let in_unit_sphere = rand_in_unit_sphere();
    if in_unit_sphere * normal < 0.0 {
        -in_unit_sphere
    } else {
        in_unit_sphere
    }
}*/
pub fn rand_in_unit_disk() -> Vec3 {
    let mut x: f64 = rand_double(-1.0, 1.0);
    let mut y: f64 = rand_double(-1.0, 1.0);
    while x * x + y * y > 1.0 {
        x = rand_double(-1.0, 1.0);
        y = rand_double(-1.0, 1.0);
    }
    Vec3::new(x, y, 0.0)
}
pub fn rand_cosine_direction() -> Vec3 {
    let r1 = rand_double(0.0, 1.0);
    let r2 = rand_double(0.0, 1.0);
    let z = (1.0 - r2).sqrt();
    let phi = 2.0 * PI * r1;
    let x = phi.cos() * r2.sqrt();
    let y = phi.sin() * r2.sqrt();
    Vec3::new(x, y, z)
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
