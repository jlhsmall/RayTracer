#![allow(clippy::float_cmp)]
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
const AUTHOR: &str = "@jlhsmall";
mod vec3;
pub use crate::material::DiffuseLight;
pub use bvh::BVHNode;
pub use camera::Camera;
pub use hittablelist::HittableList;
use image::{ImageBuffer, Rgb, RgbImage};
use indicatif::ProgressBar;
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
use rusttype::Font;
pub use sphere::Sphere;
use std::sync::mpsc::channel;
pub use std::sync::Arc;
pub use texture::CheckerTexture;
use threadpool::ThreadPool;
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
fn get_text() -> String {
    // GITHUB_SHA is the associated commit ID
    // only available on GitHub Action
    let github_sha = option_env!("GITHUB_SHA")
        .map(|x| "@".to_owned() + &x[0..6])
        .unwrap_or_default();
    format!("{}{}", AUTHOR, github_sha)
}
fn is_ci() -> bool {
    option_env!("CI").unwrap_or_default() == "true"
}
fn render_text(image: &mut RgbImage, msg: &str) {
    let font_file = if is_ci() {
        "EncodeSans-Regular.ttf"
    } else {
        "C:/Windows/Fonts/Arial.ttf"
    };
    let font_path = std::env::current_dir().unwrap().join(font_file);
    let data = std::fs::read(&font_path).unwrap();
    let font: Font = Font::try_from_vec(data).unwrap_or_else(|| {
        panic!(format!(
            "error constructing a Font from data at {:?}",
            font_path
        ));
    });

    imageproc::drawing::draw_text_mut(
        image,
        Rgb([255, 255, 255]),
        10,
        10,
        rusttype::Scale::uniform(24.0),
        &font,
        msg,
    );
}
fn main() {
    let is_ci = is_ci();

    // jobs: split image into how many parts
    // workers: maximum allowed concurrent running threads
    let (image_width, samples_per_pixel): (u32, u32) = if is_ci { (1200, 64) } else { (300, 16) };

    println!(
        "CI: {}, using {} width and {} samples",
        is_ci, image_width, samples_per_pixel
    );
    //image
    let aspect_ratio = 3.0 / 2.0;
    let image_height: u32 = (image_width as f64 / aspect_ratio) as u32;
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
    let (tx, rx) = channel();
    let n_jobs: usize = 32;
    let n_workers = 4;
    let pool = ThreadPool::new(n_workers);
    let ba = ProgressBar::new(n_jobs as u64);
    //render
    for i in 0..n_jobs {
        let tx = tx.clone();
        let world_ptr = world.clone();
        pool.execute(move || {
            let row_begin = image_height as usize * i / n_jobs;
            let row_end = image_height as usize * (i + 1) / n_jobs;
            let render_height = row_end - row_begin;
            let mut img: RgbImage = ImageBuffer::new(image_width, render_height as u32);
            for x in 0..image_width {
                for (img_y, y) in (row_begin..row_end).enumerate() {
                    let y = image_height - (y as u32) - 1;
                    let pixel = img.get_pixel_mut(x, img_y as u32);
                    let mut color = Vec3::new(0.0, 0.0, 0.0);
                    for _i in 0..samples_per_pixel {
                        let r = cam.get_ray(
                            ((x as f64) + rand_double(0.0, 1.0)) / ((image_width - 1) as f64),
                            ((y as f64) + rand_double(0.0, 1.0)) / ((image_height - 1) as f64),
                        );
                        color += get_color(r, background, world_ptr.clone(), max_depth);
                    }
                    color /= samples_per_pixel as f64;
                    color = Vec3::new(color.x.sqrt(), color.y.sqrt(), color.z.sqrt()) * 255.0;
                    *pixel = image::Rgb([color.x as u8, color.y as u8, color.z as u8]);
                }
            }
            tx.send((row_begin..row_end, img))
                .expect("failed to send result");
        });
    }
    let mut result: RgbImage = ImageBuffer::new(image_width, image_height);
    for (rows, data) in rx.iter().take(n_jobs) {
        for (idx, row) in rows.enumerate() {
            for col in 0..image_width {
                let row = row as u32;
                let idx = idx as u32;
                *result.get_pixel_mut(col, row) = *data.get_pixel(col, idx);
            }
        }
        ba.inc(1);
    }
    /*for y in 0..image_height {
            for x in 0..image_width {
                let pixel = img.get_pixel_mut(x, image_height - y - 1);

            }
            ba.inc(1);
        }
    */
    ba.finish();
    let msg = get_text();
    println!("Extra Info: {}", msg);
    render_text(&mut result, msg.as_str());
    result.save("output/test.png").unwrap();
}
