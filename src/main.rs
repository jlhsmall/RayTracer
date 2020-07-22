#[allow(clippy::float_cmp)]
mod vec3;
use image::{ImageBuffer, RgbImage};
//use indicatif::ProgressBar;
pub use vec3::Vec3;
fn fire_flower() {
    let mut img: RgbImage = ImageBuffer::new(400, 400);
    //let ba = ProgressBar::new(400);
    let mut i: f64 = -1.0;
    let mut cnt = 0;
    while i < 0.9 {
        let j = (1.0 - i * i).sqrt();
        for k in 0..200 {
            let kk = k as f64;
            let pixel = img.get_pixel_mut((200.0 + i * kk) as u32, (200.0 + j * kk) as u32);
            let color = if cnt % 2 == 0 { 0 } else { 255 };
            *pixel = image::Rgb([255, 255, color]);
        }
        i += 0.1;
        cnt += 1;
        //ba.inc(1);
    }
    i = -0.9;
    while i < 1.0 {
        let j = (1.0 - i * i).sqrt();
        for k in 0..200 {
            let kk = k as f64;
            let pixel = img.get_pixel_mut((200.0 + i * kk) as u32, (200.0 - j * kk) as u32);
            let color = if cnt % 2 == 1 { 0 } else { 255 };
            *pixel = image::Rgb([255, 255, color]);
        }
        i += 0.1;
        cnt += 1;
        //ba.inc(1);
    }
    img.save("output/fire flower.png").unwrap();
    //ba.finish();
}
fn main() {
    let x = Vec3::new(1.0, 0.0, -1.0) * Vec3::ones();
    println!("{}", x);
    let x = Vec3::new(1.0, 1.0, 1.0);
    println!("{:?}", x);
    fire_flower();
}
