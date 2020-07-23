extern crate rand;
mod hittablelist;
mod object;
mod ray;
mod sphere;
#[allow(clippy::float_cmp)]
mod vec3;
pub use hittablelist::HittableList;
use image::{ImageBuffer, RgbImage};
//use indicatif::ProgressBar;
pub use object::HitRecord;
pub use object::Hittable;
pub use ray::Ray;
pub use sphere::Sphere;
pub use vec3::Vec3;

pub use std::f64::consts::PI;
const INF: f64 = 100000000000000000.0;

//use rand::prelude::*;
/*fn degree_to_radian(degree:f64)->f64{
    degree*PI/180.0
}
fn rand_double(low:f64,high:f64)->f64{
    let mut rng=rand::thread_rng();
    let x:f64=rng.gen();
    x*(high-low)+low
}*/
fn get_color(r: Ray, world: &HittableList) -> Vec3 {
    let opt = world.hit(r, 0.0, INF);
    match opt {
        Option::Some(rec) => (rec.normal + Vec3::ones()) / 2.0,
        Option::None => {
            let v1 = Vec3::new(0.5, 0.7, 1.0);
            let v2 = Vec3::new(1.0, 1.0, 1.0);
            let t = (r.dir.y / r.dir.length() + 1.0) / 2.0;
            v1 * t + v2 * (1.0 - t)
        }
    }
}
fn main() {
    //image
    let aspect_ratio = 16.0 / 9.0;
    let image_width: u32 = 400;
    let image_height: u32 = (image_width as f64 / aspect_ratio) as u32;
    //world
    let world = HittableList::new(vec![
        Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)),
        Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)),
    ]);
    //camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;
    let mut img: RgbImage = ImageBuffer::new(image_width, image_height);
    //let ba = ProgressBar::new(256);

    let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);
    //render
    for y in 0..image_height {
        for x in 0..image_width {
            let pixel = img.get_pixel_mut(x, image_height - y - 1);
            let r = Ray::new(
                origin,
                lower_left_corner
                    + Vec3::new(
                        (x as f64) * viewport_width / ((image_width - 1) as f64),
                        (y as f64) * viewport_height / ((image_height - 1) as f64),
                        0.0,
                    ),
            );
            let color = get_color(r, &world) * 255.0;
            *pixel = image::Rgb([color.x as u8, color.y as u8, color.z as u8]);
        }
        //ba.inc(1);
    }

    img.save("output/red sphere.png").unwrap();
    //ba.finish();
}
