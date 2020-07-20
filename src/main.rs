#[allow(clippy::float_cmp)]
mod vec3;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;

pub use vec3::Vec3;
fn main() {
    let x = Vec3::new(1.0, 0.0, -1.0) * Vec3::ones();
    println!("{}", x);
    let x = Vec3::new(1.0, 1.0, 1.0);
    println!("{:?}", x);

    let mut img: RgbImage = ImageBuffer::new(1024, 512);
    let bar = ProgressBar::new(1024);

    for x in 0..255 {
        for y in 0..255 {
            let pixel = img.get_pixel_mut(x, y);
            let color = x as u8;
            let color2 = y as u8;
            *pixel = image::Rgb([color, color2, 0]);
        }
        bar.inc(1);
    }

    img.save("output/test.png").unwrap();
    bar.finish();
}
