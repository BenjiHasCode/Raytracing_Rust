mod vec3;
mod ray; // TODO how to import in Rust??
mod hittable_list;
mod hittable;
mod sphere;
mod camera;
mod util;

use std::f64::INFINITY;

use hittable::HitRecord;
use hittable::Hittable;
use hittable_list::HittableList;
use ray::Ray;
use vec3::{Vec3, Point3, Color};
use sphere::Sphere;

use crate::camera::Camera;
use crate::util::random_double;


fn main() {
    // Image
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const WIDTH: u32 = 256;//1920;
    const HEIGHT: u32 = (WIDTH as f32 / ASPECT_RATIO) as u32;
    const SAMPLES_PER_PIXEL: u32 = 10000;
    const MAX_DEPTH: u32 = 50;
    let mut imgbuf = image::ImageBuffer::new(WIDTH, HEIGHT);

    // World
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let cam = Camera::new();

    // Render
    for j in 0..HEIGHT {
        println!("Lines remaining: {}", HEIGHT - j);
        for i in 0..WIDTH {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);

            for _s in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + random_double(0.0, 1.0)) / (WIDTH-1) as f64;
                let v = (j as f64 + random_double(0.0, 1.0)) / (HEIGHT-1) as f64;
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world, MAX_DEPTH);
            }

            let c = pixel_color.translate(SAMPLES_PER_PIXEL);
            imgbuf[(i, HEIGHT - j - 1)] = image::Rgb([c.x as u8, c.y as u8, c.z as u8]);
        }
    }

    // Save image
    imgbuf.save("render.png").unwrap();
}

fn ray_color(r: &Ray, world: &HittableList, depth: u32) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    let mut rec = HitRecord::default();

    if world.hit(r, 0.001, INFINITY, &mut rec) {
        let target = rec.p + rec.normal + Vec3::random_in_unit_sphere();
        return 0.5 * ray_color(&Ray::new(rec.p, target - rec.p), world, depth-1);
    }

    let unit_direction = r.direction().unit_vector();
    let t = 0.5*(unit_direction.y + 1.0);
    return (1.0-t)*Color::new(1.0, 1.0, 1.0) + t*Color::new(0.5, 0.7, 1.0);
}