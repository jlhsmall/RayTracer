extern crate rand;
mod camera;
mod hittablelist;
mod material;
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
pub use material::Dielectric;
pub use material::Lamertian;
pub use material::Metal;
pub use object::HitRecord;
pub use object::Hittable;
pub use oneweekend::rand_double;
pub use oneweekend::rand_unit_vector;
pub use oneweekend::INF;
pub use oneweekend::PI;
pub use ray::Ray;
pub use sphere::Sphere;
pub use std::sync::Arc;
pub use vec3::Vec3;

fn get_color(r: Ray, world: &HittableList, depth: i32) -> Vec3 {
    if depth <= 0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }
    let opt = world.hit(r, 0.001, INF);
    match opt {
        Option::Some(rec) => {
            let opt2 = rec.mat_ptr.scatter(r, rec.clone());
            match opt2 {
                Option::Some(rec2) => Vec3::elemul(
                    get_color(rec2.scattered, world, depth - 1),
                    rec2.attenuation,
                ),
                Option::None => Vec3::new(0.0, 0.0, 0.0),
            }
        }
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
    let max_depth = 50;
    //world
    let mat_ground = Arc::new(Lamertian::new(Vec3::new(0.8, 0.8, 0.0)));
    let mat_centre = Arc::new(Lamertian::new(Vec3::new(0.1, 0.2, 0.5)));
    let mat_left = Arc::new(Dielectric::new(1.5));
    let mat_right = Arc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.0));
    let world = HittableList::new(vec![
        Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, mat_ground)),
        Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, mat_centre)),
        Box::new(Sphere::new(
            Vec3::new(-1.0, 0.0, -1.0),
            0.5,
            mat_left.clone(),
        )),
        Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), -0.45, mat_left)),
        Box::new(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, mat_right)),
    ]);
    /*let R=(PI/4.0).cos();

    let mat_left = Arc::new(Lamertian::new(Vec3::new(0.0, 0.0, 1.0)));
    let mat_right = Arc::new(Lamertian::new(Vec3::new(1.0, 0.0, 0.0)));
    let world = HittableList::new(vec![
        Box::new(Sphere::new(Vec3::new(-R, 0.0, -1.0), R, mat_left)),
        Box::new(Sphere::new(Vec3::new(R, 0.0, -1.0), R, mat_right)),
    ]);*/
    //camera
    let cam = Camera::new(
        Vec3::new(-2.0, 2.0, 1.0),
        Vec3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
        90.0,
        aspect_ratio,
    );
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
                color += get_color(r, &world, max_depth);
            }
            color /= samples_per_pixel as f64;
            color = Vec3::new(color.x.sqrt(), color.y.sqrt(), color.z.sqrt()) * 255.0;
            *pixel = image::Rgb([color.x as u8, color.y as u8, color.z as u8]);
        }
        //ba.inc(1);
    }

    img.save("output/red sphere.png").unwrap();
    //ba.finish();
}
