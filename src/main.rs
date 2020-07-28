extern crate rand;
mod aabb;
mod bvh;
mod camera;
mod hittablelist;
mod material;
mod object;
mod oneweekend;
mod ray;
mod sphere;
mod texture;
#[allow(clippy::float_cmp)]
mod vec3;
pub use hittablelist::HittableList;
use image::{ImageBuffer, RgbImage};
//use indicatif::ProgressBar;
pub use crate::material::DiffuseLight;
pub use bvh::BVHNode;
pub use camera::Camera;
pub use material::Dielectric;
pub use material::Lamertian;
pub use material::Material;
pub use material::Metal;
pub use object::HitRecord;
pub use object::Hittable;
pub use oneweekend::rand_double;
pub use oneweekend::rand_unit_vector;
pub use oneweekend::rand_vector;
pub use oneweekend::INF;
pub use oneweekend::PI;
pub use ray::Ray;
pub use sphere::Sphere;
pub use std::sync::Arc;
pub use texture::CheckerTexture;
pub use vec3::Vec3;

fn get_color(r: Ray, background: Vec3, world: Arc<BVHNode>, depth: i32) -> Vec3 {
    if depth <= 0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }
    let opt = world.hit(r, 0.001, INF);
    if opt.is_none() {
        return background;
    }
    let rec = opt.unwrap();
    let opt2 = rec.mat_ptr.scatter(r, rec.clone());
    let emitted = rec.mat_ptr.emitted(rec.u, rec.v, rec.p);
    if opt2.is_none() {
        return emitted;
    }
    let rec2 = opt2.unwrap();
    emitted
        + Vec3::elemul(
            rec2.attenuation,
            get_color(rec2.scattered, background, world, depth - 1),
        )
    /*match opt {
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
    }*/
}
fn random_scene() -> Arc<BVHNode> {
    let mut objects: Vec<Arc<dyn Hittable>> = Vec::new();
    let checker = Arc::new(CheckerTexture::new(
        Vec3::new(0.2, 0.3, 0.1),
        Vec3::new(0.9, 0.9, 0.9),
    ));
    objects.push(Arc::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        -1000.0,
        Arc::new(Lamertian::newa(checker)),
    )));
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rand_double(0.0, 1.0);
            let centre = Vec3::new(
                (a as f64) + rand_double(0.0, 0.9),
                0.2,
                (b as f64) + rand_double(0.0, 0.9),
            );
            if (centre - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Arc<dyn Material>;
                if choose_mat < 0.5 {
                    let albedo = rand_vector(0.5, 1.0);
                    sphere_material = Arc::new(DiffuseLight::new(albedo));
                    objects.push(Arc::new(Sphere::new(centre, 0.2, sphere_material)));
                } else if choose_mat < 0.8 {
                    let albedo = Vec3::elemul(rand_vector(0.0, 1.0), rand_vector(0.0, 1.0));
                    sphere_material = Arc::new(Lamertian::new(albedo));
                    objects.push(Arc::new(Sphere::new(centre, 0.2, sphere_material)));
                } else if choose_mat < 0.95 {
                    let albedo = rand_vector(0.5, 1.0);
                    let fuzz = rand_double(0.0, 0.5);
                    sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    objects.push(Arc::new(Sphere::new(centre, 0.2, sphere_material)));
                } else {
                    sphere_material = Arc::new(Dielectric::new(1.5));
                    objects.push(Arc::new(Sphere::new(centre, 0.2, sphere_material)));
                }
            }
        }
    }
    let material1 = Arc::new(DiffuseLight::new(Vec3::ones()));
    objects.push(Arc::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));
    let material2 = Arc::new(Lamertian::new(Vec3::new(0.4, 0.2, 0.1)));
    objects.push(Arc::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));
    let material3 = Arc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0));
    objects.push(Arc::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));
    let span = objects.len();
    Arc::new(BVHNode::new(objects, span, 0.001, INF))
    //HittableList::new(vec![tree])
}
fn main() {
    //image
    let aspect_ratio = 3.0 / 2.0;
    let image_width: u32 = 1200;
    let image_height: u32 = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel: u32 = 64;
    let max_depth = 50;
    //world
    let background = Vec3::new(0.0, 0.0, 0.0);
    let world = random_scene();
    //camera
    let lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let focus_dist = 10.0;
    let aperture = 0.1;
    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        aspect_ratio,
        aperture,
        focus_dist,
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
                color += get_color(r, background, world.clone(), max_depth);
            }
            color /= samples_per_pixel as f64;
            color = Vec3::new(color.x.sqrt(), color.y.sqrt(), color.z.sqrt()) * 255.0;
            *pixel = image::Rgb([color.x as u8, color.y as u8, color.z as u8]);
        }
        //ba.inc(1);
    }

    img.save("output/test.png").unwrap();
    //ba.finish();
}
