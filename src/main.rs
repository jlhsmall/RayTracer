mod ray;
mod sphere;
#[allow(clippy::float_cmp)]
mod vec3;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
pub use ray::Ray;
pub use sphere::Sphere;
pub use vec3::Vec3;
fn get_color(r: Ray) -> Vec3 {
    if r.hit_sphere(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)) {
        return Vec3::new(1.0, 0.0, 0.0);
    }
    let v1 = Vec3::new(0.5, 0.7, 1.0);
    let v2 = Vec3::new(1.0, 1.0, 1.0);
    let t = (r.dir.y / r.dir.length() + 1.0) / 2.0;
    v2 * t + v1 * (1.0 - t)
}
fn main() {
    //blue to white gradient
    let aspect_ratio = 16.0 / 9.0;
    let image_width: u32 = 400;
    let image_height: u32 = (image_width as f64 / aspect_ratio) as u32;
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;
    let mut img: RgbImage = ImageBuffer::new(image_width, image_height);
    let ba = ProgressBar::new(256);
    let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);
    for x in 0..image_width {
        for y in 0..image_height {
            let pixel = img.get_pixel_mut(x, y);
            let r = Ray::new(
                origin,
                lower_left_corner
                    + Vec3::new(
                        (x as f64) * viewport_width / (image_width as f64),
                        (y as f64) * viewport_height / (image_height as f64),
                        0.0,
                    ),
            );
            let color = get_color(r) * 255.0;
            *pixel = image::Rgb([color.x as u8, color.y as u8, color.z as u8]);
        }
        ba.inc(1);
    }

    img.save("output/red sphere.png").unwrap();
    ba.finish();
}
