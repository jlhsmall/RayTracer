pub use std::f64::consts::PI;
pub const INF: f64 = 100000000000000000.0;

use rand::prelude::*;
/*fn degree_to_radian(degree:f64)->f64{
    degree*PI/180.0
}*/
pub fn rand_double(low: f64, high: f64) -> f64 {
    let mut rng = rand::thread_rng();
    let x: f64 = rng.gen();
    x * (high - low) + low
}
/*pub fn clamp(x:f64,low:f64,high:f64)->f64{
    if x<low{low}
    else if x>high{high}
    else {x}
}*/
