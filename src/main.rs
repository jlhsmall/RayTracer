extern crate rand;
mod camera;
mod hittablelist;
mod object;
mod oneweekend;
mod ray;
mod sphere;
#[allow(clippy::float_cmp)]
mod vec3;
pub use hittablelist::HittableList;
use image::{ImageBuffer, RgbImage};
//use indicatif::ProgressBar;
pub use camera::Camera;
pub use object::HitRecord;
pub use object::Hittable;
pub use oneweekend::rand_double;
pub use oneweekend::INF;
pub use ray::Ray;
pub use sphere::Sphere;
pub use vec3::Vec3;

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
    let samples_per_pixel: u32 = 100;
    //world
    let world = HittableList::new(vec![
        Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)),
        Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)),
    ]);
    //camera
    let cam = Camera::new(aspect_ratio);
    //let ba = ProgressBar::new(256);
    let mut img: RgbImage = ImageBuffer::new(image_width, image_height);
    //render
    for y in 0..image_height {
        for x in 0..image_width {
            let pixel = img.get_pixel_mut(x, image_height - y - 1);
            let mut color = Vec3::new(0.0, 0.0, 0.0);
            for _i in 0..samples_per_pixel {
                let r = cam.get_ray(
                    ((x as f64) + rand_double(0.0, 1.0)) / ((image_width - 1) as f64),
                    ((y as f64) + rand_double(0.0, 1.0)) / ((image_height - 1) as f64),
                );
                color += get_color(r, &world);
            }
            color *= 255.0 / (samples_per_pixel as f64);
            *pixel = image::Rgb([color.x as u8, color.y as u8, color.z as u8]);
        }
        //ba.inc(1);
    }

    img.save("output/red sphere.png").unwrap();
    //ba.finish();
}
