mod vec3;
mod ray;
mod hittable_list;
mod hittable;
mod sphere;
mod camera;
mod util;
mod material;


use std::f64::INFINITY;
use std::sync::Arc;
use std::time::Instant;

use hittable::HitRecord;
use hittable::Hittable;
use hittable_list::HittableList;
use ray::Ray;
use rayon::prelude::*;
use vec3::{Point3, Color};
use sphere::Sphere;
use material::Material;

use crate::camera::Camera;
use crate::material::dielectric::Dielectric;
use crate::material::lambertian::Lambertian;
use crate::material::metal::Metal;
use crate::util::random_double;
use crate::vec3::Vec3;


fn main() {
    // Image
    const ASPECT_RATIO: f64 = 3.0 / 2.0;
    const WIDTH: u32 = 1200;
    const HEIGHT: u32 = (WIDTH as f64 / ASPECT_RATIO) as u32;
    const SAMPLES_PER_PIXEL: u32 = 50;
    const MAX_DEPTH: u32 = 50;
   
    let mut pixels = vec![];
    for y in (0..HEIGHT).rev() {
        for x in 0..WIDTH {
            pixels.push((y as f64, x as f64));
        }
    }

    // World
    let world = random_scene();

    // Camera
    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let cam = Camera::new(look_from, look_at, vup, 20.0, ASPECT_RATIO, aperture, dist_to_focus);

    let start = Instant::now();
    // Render
    let bytes: Vec<u8> = pixels.par_iter().flat_map(|(y, x)| {
        let mut pixel_color = Color::new(0.0, 0.0, 0.0);

        for _ in 0..SAMPLES_PER_PIXEL {
            let u = (x + random_double(0.0, 1.0)) / (WIDTH-1) as f64;
            let v = (y + random_double(0.0, 1.0)) / (HEIGHT-1) as f64;
            let r = cam.get_ray(u, v);
            pixel_color += ray_color(&r, &world, MAX_DEPTH);
        }

        let c = pixel_color.translate(SAMPLES_PER_PIXEL);
        vec![c.x as u8, c.y as u8, c.z as u8]
    }).collect();

    // Print how long it took to render
    let duration = start.elapsed().as_secs();
    println!("Render took: {} seconds", duration);
    
    // Save image
    image::save_buffer("render.png", &bytes, WIDTH, HEIGHT, image::ColorType::Rgb8).unwrap();
}

fn ray_color(r: &Ray, world: &HittableList, depth: u32) -> Color {
    if depth == 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    let mut rec = HitRecord::default();

    if world.hit(r, 0.001, INFINITY, &mut rec) {
        let mut scattered = Ray::default();
        let mut attenuation = Color::default();
        if rec.material.scatter(r, &rec, &mut attenuation, &mut scattered) {
            return attenuation * ray_color(&scattered, world, depth-1);
        }
        return Color::new(0.0, 0.0, 0.0);
    }

    let unit_direction = r.direction().unit_vector();
    let t = 0.5*(unit_direction.y + 1.0);
    (1.0-t)*Color::new(1.0, 1.0, 1.0) + t*Color::new(0.5, 0.7, 1.0)
}

fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let ground_material = Arc::new(Material::Lambertian(Lambertian::new(Color::new(0.5, 0.5, 0.5))));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, &ground_material)));


    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double(0.0, 1.0);
            let center = Point3::new(a as f64 + 0.9*random_double(0.0, 1.0), 0.2, b as f64 + 0.9*random_double(0.0, 1.0));

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material;

                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random(0.0, 1.0) * Color::random(0.0, 1.0);
                    sphere_material = Arc::new(Material::Lambertian(Lambertian::new(albedo)));
                    world.add(Box::new(Sphere::new(center, 0.2, &sphere_material)))
                }
                else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random(0.5, 1.0);
                    let fuzz = random_double(0.0, 0.5);
                    sphere_material = Arc::new(Material::Metal(Metal::new(albedo, fuzz)));
                    world.add(Box::new(Sphere::new(center, 0.2, &sphere_material)))
                }
                else {
                    // glass
                    sphere_material = Arc::new(Material::Dielectric(Dielectric::new(1.5)));
                    world.add(Box::new(Sphere::new(center, 0.2, &sphere_material)))
                }
            }
        }
    }

    let material1 = Arc::new(Material::Dielectric(Dielectric::new(1.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, &material1)));

    let material2 = Arc::new(Material::Lambertian(Lambertian::new(Color::new(0.4, 0.2, 0.1))));
    world.add(Box::new(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, &material2)));

    let material3 = Arc::new(Material::Metal(Metal::new(Color::new(0.7, 0.6, 0.5), 0.1)));
    world.add(Box::new(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, &material3)));

    world
}