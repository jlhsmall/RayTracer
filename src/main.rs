#![allow(clippy::float_cmp)]
extern crate rand;
mod aabb;
mod aarect;
mod bvh;
mod camera;
mod cbox;
mod hittablelist;
mod material;
mod object;
mod onb;
mod oneweekend;
mod pdf;
mod ray;
mod sphere;
mod texture;
const AUTHOR: &str = "jlhsmall";
mod vec3;
pub use aarect::{XYRect, XZRect, YZRect};
pub use bvh::BVHNode;
pub use camera::Camera;
pub use cbox::CBox;
pub use hittablelist::HittableList;
use image::{ImageBuffer, Rgb, RgbImage};
use indicatif::ProgressBar;
pub use material::{
    Dielectric, DiffuseLight, Lamertian, Material, Metal, NoMaterial, ScatterRecord, ScatterType,
};
pub use object::{FlipFace, HitRecord, Hittable, RotateY, Translate};
pub use oneweekend::{rand_double, rand_unit_vector, rand_vector, INF, PI};
pub use pdf::{CosinePDF, HittablePDF, MixturePDF, PDF};
pub use ray::Ray;
use rusttype::Font;
pub use sphere::Sphere;
use std::sync::mpsc::channel;
pub use std::sync::Arc;
pub use texture::CheckerTexture;
use threadpool::ThreadPool;
pub use vec3::Vec3;

fn get_color(
    r: Ray,
    background: Vec3,
    world: Arc<BVHNode>,
    lights: Arc<dyn Hittable>,
    depth: i32,
) -> Vec3 {
    if depth <= 0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }
    let opt = world.hit(r, 0.001, INF);
    if opt.is_none() {
        return background;
    }
    let rec = opt.unwrap();
    let opt2 = rec.mat_ptr.scatter(r, rec.clone());
    let emitted = rec.mat_ptr.emitted(r, rec.clone(), rec.u, rec.v, rec.p);
    if opt2.is_none() {
        return emitted;
    }
    let rec2 = opt2.unwrap();
    match rec2.tp {
        ScatterType::Specular(specular_ray) => Vec3::elemul(
            rec2.attenuation,
            get_color(specular_ray, background, world, lights, depth - 1),
        ),
        ScatterType::Pdf(pdf_ptr) => {
            let light_ptr = Arc::new(HittablePDF::new(lights.clone(), rec.p));
            let p = MixturePDF::new(light_ptr, pdf_ptr);
            let scattered = Ray::new(rec.p, p.generate());
            let pdf_val = p.value(scattered.dir);
            emitted
                + Vec3::elemul(
                    rec2.attenuation,
                    get_color(scattered, background, world, lights, depth - 1)
                        * rec.mat_ptr.scattering_pdf(r, rec.clone(), scattered)
                        / pdf_val,
                )
        }
    }
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
fn simple_light() -> Arc<BVHNode> {
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
    let difflight = Arc::new(DiffuseLight::new(Vec3::new(4.0, 4.0, 4.0)));
    objects.push(Arc::new(XYRect::new(3.0, 5.0, 1.0, 3.0, -2.0, difflight)));
    let span = objects.len();
    Arc::new(BVHNode::new(objects, span, 0.001, INF))
}
fn cornell_box() -> Arc<BVHNode> {
    let mut objects: Vec<Arc<dyn Hittable>> = Vec::new();
    let red = Arc::new(Lamertian::new(Vec3::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lamertian::new(Vec3::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lamertian::new(Vec3::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::new(Vec3::new(15.0, 15.0, 15.0)));
    objects.push(Arc::new(YZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green)));
    objects.push(Arc::new(YZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red)));
    objects.push(Arc::new(FlipFace::new(Arc::new(XZRect::new(
        213.0, 343.0, 227.0, 332.0, 554.0, light,
    )))));
    objects.push(Arc::new(XZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        white.clone(),
    )));
    objects.push(Arc::new(XZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));
    objects.push(Arc::new(XYRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));
    let aluminum = Arc::new(Metal::new(Vec3::new(0.8, 0.85, 0.88), 0.0));
    let mut box1: Arc<dyn Hittable> = Arc::new(CBox::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(165.0, 330.0, 165.0),
        aluminum,
    ));
    box1 = Arc::new(RotateY::new(box1, 15.0));
    box1 = Arc::new(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)));
    objects.push(box1);
    let mut box2: Arc<dyn Hittable> = Arc::new(CBox::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(165.0, 165.0, 165.0),
        white,
    ));
    box2 = Arc::new(RotateY::new(box2, -18.0));
    box2 = Arc::new(Translate::new(box2, Vec3::new(130.0, 0.0, 65.0)));
    objects.push(box2);
    let span = objects.len();
    Arc::new(BVHNode::new(objects, span, 0.001, INF))
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
    let (image_width, samples_per_pixel): (u32, u32) = if is_ci { (1200, 500) } else { (300, 50) };

    println!(
        "CI: {}, using {} width and {} samples",
        is_ci, image_width, samples_per_pixel
    );
    let aspect_ratio: f64;
    let image_height: u32;
    let max_depth: i32;
    //image
    //world
    let background = Vec3::new(0.0, 0.0, 0.0);
    let world: Arc<BVHNode>;
    //camera

    let lookfrom: Vec3;
    let lookat: Vec3;
    let vfov: f64;
    let vup: Vec3;
    let focus_dist = 10.0;
    let aperture = 0.0;
    let x = 2;
    if x == 0 {
        world = random_scene();
        aspect_ratio = 3.0 / 2.0;
        lookfrom = Vec3::new(13.0, 2.0, 3.0);
        lookat = Vec3::new(0.0, 0.0, 0.0);
        vup = Vec3::new(0.0, 1.0, 0.0);
        vfov = 20.0;
    } else if x == 1 {
        world = simple_light();
        aspect_ratio = 3.0 / 2.0;
        lookfrom = Vec3::new(26.0, 3.0, 6.0);
        lookat = Vec3::new(0.0, 2.0, 0.0);
        vup = Vec3::new(0.0, 1.0, 0.0);
        vfov = 20.0;
    } else {
        world = cornell_box();
        aspect_ratio = 1.0;
        lookfrom = Vec3::new(278.0, 278.0, -800.0);
        lookat = Vec3::new(278.0, 278.0, 0.0);
        vup = Vec3::new(0.0, 1.0, 0.0);
        vfov = 40.0;
    }
    let mut light_obj: Vec<Arc<dyn Hittable>> = Vec::new();
    light_obj.push(Arc::new(XZRect::new(
        213.0,
        343.0,
        227.0,
        332.0,
        554.0,
        Arc::new(NoMaterial),
    )));
    //light_obj.push(Arc::new(Sphere::new(Vec3::new(190.0, 90.0, 190.0),90.0,Arc::new(NoMaterial))));
    let lights = Arc::new(HittableList::new(light_obj));
    image_height = (image_width as f64 / aspect_ratio) as u32;
    max_depth = 50;
    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        vfov,
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
        let lights_ptr = lights.clone();
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
                        color += get_color(
                            r,
                            background,
                            world_ptr.clone(),
                            lights_ptr.clone(),
                            max_depth,
                        );
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
