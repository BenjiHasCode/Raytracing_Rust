mod vec3;
mod ray;
mod hittable_list;
mod hittable;
mod sphere;
mod camera;
mod util;
mod material;
mod aabb;
mod bvh;
mod texture;
mod perlin;
mod xy_rect;
mod yz_rect;
mod xz_rect;

use std::f64::INFINITY;
use std::sync::{Arc, Mutex};
use std::time::Instant;

use hittable::Hittable;
use hittable_list::HittableList;
use material::diffuse_light::DiffuseLight;
use ray::Ray;
use rayon::prelude::*;
use texture::{CheckerTexture, Texture, NoiseTexture, ImageTexture, SolidColor};
use vec3::{Point3, Color};
use sphere::{Sphere, MovingSphere};
use material::Material;
use xy_rect::XYRect;
use xz_rect::XZRect;
use yz_rect::YZRect;

use crate::camera::Camera;
use crate::material::dielectric::Dielectric;
use crate::material::lambertian::Lambertian;
use crate::material::metal::Metal;
use crate::util::{random_double, calculate_percentage};
use crate::vec3::Vec3;


fn main() {
    // Image
    let mut aspect_ratio: f64 = 16.0 / 9.0;
    let mut width: u32 = 1920;
    let mut height: u32 = (width as f64 / aspect_ratio) as u32;
    const SAMPLES_PER_PIXEL: u32 = 1000;
    const MAX_DEPTH: u32 = 50;
    const BYTES_PER_PIXEL: usize = 3;

    // World
    let world;


    let look_from;
    let look_at;
    let vfov;
    let mut aperture = 0.0;
    let mut background = Color::new(0.0, 0.0, 0.0);
    // Camera
    let scene = 6;
    match scene {
        1 => {
            world = random_scene();
            background = Color::new(0.7, 0.8, 1.0);
            look_from = Point3::new(13.0, 2.0, 3.0);
            look_at = Point3::new(0.0, 0.0, 0.0);
            vfov = 20.0;
            aperture = 0.1;
        },
        2 => {
            world = two_spheres();
            background = Color::new(0.7, 0.8, 1.0);
            look_from = Point3::new(13.0, 2.0, 3.0);
            look_at = Point3::new(0.0, 0.0, 0.0);
            vfov = 20.0;
        },
        3 => {
            world = two_perlin_spheres();
            background = Color::new(0.7, 0.8, 1.0);
            look_from = Point3::new(13.0, 2.0, 3.0);
            look_at = Point3::new(0.0, 0.0, 0.0);
            vfov = 20.0;
        },
        4 => {
            world = earth();
            background = Color::new(0.7, 0.8, 1.0);
            look_from = Point3::new(13.0, 2.0, 3.0);
            look_at = Point3::new(0.0, 0.0, 0.0);
            vfov = 20.0;
        },
        5 => {
            world = simple_light();
            background = Color::new(0.0, 0.0, 0.0);
            look_from = Point3::new(26.0, 3.0, 6.0);
            look_at = Point3::new(0.0, 2.0, 0.0);
            vfov = 20.0;
        },
        6 => {
            world = cornell_box();
            aspect_ratio = 1.0;
            width = 600;
            height = (width as f64 / aspect_ratio) as u32;
            background = Color::new(0.0, 0.0, 0.0);
            look_from = Point3::new(278.0, 278.0, -800.0);
            look_at = Point3::new(278.0, 278.0, 0.0);
            vfov = 40.0;
        }
        _ => {
            world = random_scene();
            look_from = Point3::new(13.0, 2.0, 3.0);
            look_at = Point3::new(0.0, 0.0, 0.0);
            vfov = 20.0;
            aperture = 0.1;
        }
    }

    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let cam = Camera::new(
        look_from,
        look_at,
        vup,
        vfov,
        aspect_ratio,
        aperture,
        dist_to_focus,
        0.0,
        1.0
    );


    let start = Instant::now();
    // Render
    let mut bytes = vec![0u8; height as usize * width as usize * BYTES_PER_PIXEL];
    let whole = bytes.len();
    let current = Mutex::new(0);
    let percent = Mutex::new(0);
    bytes
        // take mutable chunk of three items
        .par_chunks_mut(BYTES_PER_PIXEL)
        // turn into a parralel iterator using Rayon
        .into_par_iter()
        // enumerate() changes the closure argument from |item| => |(index, item)|
        .enumerate()
        .for_each(|(idx, chunk)| {
            let y = (height as usize - idx / width as usize) as f64;
            let x = (idx % width as usize) as f64;
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);

            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (x + random_double(0.0, 1.0)) / (width-1) as f64;
                let v = (y + random_double(0.0, 1.0)) / (height-1) as f64;
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &background, &world, MAX_DEPTH);
            }
            let (r, g, b) = pixel_color.to_u8_rgb(SAMPLES_PER_PIXEL);
            chunk[0] = r;
            chunk[1] = g;
            chunk[2] = b;

            // calculate percentage done
            *current.lock().unwrap() += BYTES_PER_PIXEL;
            let temp_percent = calculate_percentage(whole, *current.lock().unwrap());
            if *percent.lock().unwrap() != temp_percent {
                *percent.lock().unwrap() = temp_percent;
                println!("{}%", temp_percent);
            }
        });

    // Print how long it took to render
    let duration = start.elapsed().as_secs();
    println!("Render took: {} seconds", duration);
    
    // Save image
    image::save_buffer("render.png", &bytes, width, height, image::ColorType::Rgb8).unwrap();
}

fn ray_color(r: &Ray, background: &Color, world: &HittableList, depth: u32) -> Color {
    if depth == 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if let Some(rec) = world.hit(r, 0.001, INFINITY) {
        let emitted = rec.material.emitted(rec.u, rec.v, &rec.p);

        if let Some((attenuation, scattered)) = rec.material.scatter(r, &rec) {
            emitted + attenuation * ray_color(&scattered, background, world, depth - 1)
        } else {
            emitted
        }
    } else {
        *background
    }
}

fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let check_tex: Arc<dyn Texture> = Arc::new(CheckerTexture::new(Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9)));
    let ground_material: Arc<dyn Material> = Arc::new(Lambertian::new_texture(&check_tex));
    world.push(Arc::new(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, &ground_material)));


    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double(0.0, 1.0);
            let center = Point3::new(a as f64 + 0.9*random_double(0.0, 1.0), 0.2, b as f64 + 0.9*random_double(0.0, 1.0));

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Arc<dyn Material>;

                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random(0.0, 1.0) * Color::random(0.0, 1.0);
                    sphere_material = Arc::new(Lambertian::new_color(albedo));
                    let center2 = center + Vec3::new(0.0, random_double(0.0, 0.5), 0.0);
                    world.push(Arc::new(MovingSphere::new(center, center2, 0.0, 1.0, 0.2, &sphere_material)))
                }
                else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random(0.5, 1.0);
                    let fuzz = random_double(0.0, 0.5);
                    sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    world.push(Arc::new(Sphere::new(center, 0.2, &sphere_material)))
                }
                else {
                    // glass
                    sphere_material = Arc::new(Dielectric::new(1.5));
                    world.push(Arc::new(Sphere::new(center, 0.2, &sphere_material)))
                }
            }
        }
    }

    let material1: Arc<dyn Material> = Arc::new(Dielectric::new(1.5));
    world.push(Arc::new(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, &material1)));

    let material2: Arc<dyn Material> = Arc::new(Lambertian::new_color(Color::new(0.4, 0.2, 0.1)));
    world.push(Arc::new(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, &material2)));

    let material3: Arc<dyn Material> = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.05));
    world.push(Arc::new(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, &material3)));

    world
}

fn two_spheres() -> HittableList {
    let mut objects = HittableList::new();

    let checker_texture: Arc<dyn Texture> = Arc::new(CheckerTexture::new(Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9)));
    let checker_material: Arc<dyn Material> = Arc::new(Lambertian::new_texture(&checker_texture));
    objects.push(Arc::new(Sphere::new(Point3::new(0.0, -10.0, 0.0), 10.0, &checker_material)));
    objects.push(Arc::new(Sphere::new(Point3::new(0.0, 10.0, 0.0), 10.0, &checker_material)));

    objects
}

fn two_perlin_spheres() -> HittableList {
    let mut objects = HittableList::new();

    let pertext: Arc<dyn Texture> = Arc::new(NoiseTexture::new(4.0));
    let permat: Arc<dyn Material> = Arc::new(Lambertian::new_texture(&pertext));
    objects.push(Arc::new(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, &permat)));
    objects.push(Arc::new(Sphere::new(Point3::new(0.0, 2.0, 0.0), 2.0, &permat)));

    objects
}

fn earth() -> HittableList {
    let mut objects = HittableList::new();

    let earth_texture: Arc<dyn Texture> = Arc::new(ImageTexture::new("./resources/earthmap.jpg"));
    let earth_material: Arc<dyn Material> = Arc::new(Lambertian::new_texture(&earth_texture));
    objects.push(Arc::new(Sphere::new(Point3::new(0.0, 0.0, 0.0), 2.0, &earth_material)));

    objects
}

fn simple_light() -> HittableList {
    let mut objects = HittableList::new();

    let pertext: Arc<dyn Texture> = Arc::new(NoiseTexture::new(4.0));
    let permat: Arc<dyn Material> = Arc::new(Lambertian::new_texture(&pertext));
    objects.push(Arc::new(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, &permat)));
    objects.push(Arc::new(Sphere::new(Point3::new(0.0, 2.0, 0.0), 2.0, &permat)));

    let color = Color::new(4.0, 4.0, 4.0);
    let difflight: Arc<dyn Material> = Arc::new(DiffuseLight::new_color(color));
    objects.push(Arc::new(XYRect::new(3.0, 5.0, 1.0, 3.0, -2.0, &difflight)));

    objects
}

fn cornell_box() -> HittableList {
    let mut objects = HittableList::new();

    let red: Arc<dyn Material> = Arc::new(Lambertian::new_color(Color::new(0.65, 0.05, 0.05)));
    let white: Arc<dyn Material> = Arc::new(Lambertian::new_color(Color::new(0.73, 0.73, 0.73)));
    let green: Arc<dyn Material> = Arc::new(Lambertian::new_color(Color::new(0.12, 0.45, 0.15)));
    
    let light: Arc<dyn Material> = Arc::new(DiffuseLight::new_color(Color::new(15.0, 15.0, 15.0)));

    objects.push(Arc::new(YZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, &green)));
    objects.push(Arc::new(YZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, &red)));
    objects.push(Arc::new(XZRect::new(213.0, 343.0, 227.0, 332.0, 554.0, &light)));
    objects.push(Arc::new(XZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, &white)));
    objects.push(Arc::new(XZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, &white)));
    objects.push(Arc::new(XYRect::new(0.0, 555.0, 0.0, 555.0, 555.0, &white)));

    objects
}