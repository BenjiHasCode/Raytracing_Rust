use std::{time::Instant, sync::{Mutex, Arc}, f64::INFINITY};

use hittable::{hittable_list::HittableList, Hittable, sphere::Sphere, xy_rect::XYRect, yz_rect::YZRect, xz_rect::XZRect, rotate_y::RotateY, translate::Translate, constant_medium::ConstantMedium, bvh::BvhNode, r#box::Box, moving_sphere::MovingSphere};
use material::{Material, lambertian::Lambertian, metal::Metal, dielectric::Dielectric, diffuse_light::DiffuseLight};
use ray::Ray;
use rayon::{slice::ParallelSliceMut, prelude::{IntoParallelIterator, IndexedParallelIterator, ParallelIterator}};
use texture::{Texture, image::ImageTexture, noise::NoiseTexture, checker::CheckerTexture};

use crate::{vec3::{Color, Point3, Vec3}, camera::Camera, util::{random_double, calculate_percentage}};

mod vec3;
mod util;
mod ray;
mod perlin;
mod hit_record;
mod camera;
mod aabb;
mod texture;
mod material;
mod hittable;

fn main() {
    // Image
    let mut aspect_ratio: f64 = 16.0 / 9.0;
    let mut width: u32 = 1920;
    let mut height: u32 = (width as f64 / aspect_ratio) as u32;
    const SAMPLES_PER_PIXEL: u32 = 1;
    const MAX_DEPTH: u32 = 50;
    const BYTES_PER_PIXEL: usize = 3;

    // World
    let world;

    // Camera
    let look_from;
    let look_at;
    let vfov;
    let mut aperture = 0.0;
    let background;

    let scene = 8;
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
        },
        7 => {
            world = cornell_smoke();
            aspect_ratio = 1.0;
            width = 600;
            height = (width as f64 / aspect_ratio) as u32;
            background = Color::new(0.0, 0.0, 0.0);
            look_from = Point3::new(278.0, 278.0, -800.0);
            look_at = Point3::new(278.0, 278.0, 0.0);
            vfov = 40.0;
        },
        8 => {
            world = final_scene();
            aspect_ratio = 1.0;
            width = 800;
            height = (width as f64 / aspect_ratio) as u32;
            background = Color::new(0.0, 0.0, 0.0);
            look_from = Point3::new(478.0, 278.0, -600.0);
            look_at = Point3::new(278.0, 278.0, 0.0);
            vfov = 40.0;
        }
        _ => {
            world = final_scene();
            aspect_ratio = 1.0;
            width = 800;
            height = (width as f64 / aspect_ratio) as u32;
            background = Color::new(0.0, 0.0, 0.0);
            look_from = Point3::new(478.0, 278.0, -600.0);
            look_at = Point3::new(278.0, 278.0, 0.0);
            vfov = 40.0;
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

    // room
    objects.push(Arc::new(YZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, &green)));
    objects.push(Arc::new(YZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, &red)));
    objects.push(Arc::new(XZRect::new(213.0, 343.0, 227.0, 332.0, 554.0, &light)));
    objects.push(Arc::new(XZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, &white)));
    objects.push(Arc::new(XZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, &white)));
    objects.push(Arc::new(XYRect::new(0.0, 555.0, 0.0, 555.0, 555.0, &white)));

    // boxes
    let box1: Arc<dyn Hittable> = Arc::new(Box::new(&Point3::new(0.0, 0.0, 0.0), &Point3::new(165.0, 330.0, 165.0), &white));
    let box1: Arc<dyn Hittable> = Arc::new(RotateY::new(&box1, 15.0));
    let box1 = Arc::new(Translate::new(&box1, &Vec3::new(265.0, 0.0, 295.0)));
    objects.push(box1);

    let box2: Arc<dyn Hittable> = Arc::new(Box::new(&Point3::new(0.0, 0.0, 0.0), &Point3::new(165.0, 165.0, 165.0), &white));
    let box2: Arc<dyn Hittable> = Arc::new(RotateY::new(&box2, -18.0));
    let box2 = Arc::new(Translate::new(&box2, &Vec3::new(130.0, 0.0, 65.0)));
    objects.push(box2);

    objects
}

fn cornell_smoke() -> HittableList {
    let mut objects = HittableList::new();

    let red: Arc<dyn Material> = Arc::new(Lambertian::new_color(Color::new(0.65, 0.05, 0.05)));
    let white: Arc<dyn Material> = Arc::new(Lambertian::new_color(Color::new(0.73, 0.73, 0.73)));
    let green: Arc<dyn Material> = Arc::new(Lambertian::new_color(Color::new(0.12, 0.45, 0.15)));
    
    let light: Arc<dyn Material> = Arc::new(DiffuseLight::new_color(Color::new(7.0, 7.0, 7.0)));

    // room
    objects.push(Arc::new(YZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, &green)));
    objects.push(Arc::new(YZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, &red)));
    objects.push(Arc::new(XZRect::new(113.0, 443.0, 127.0, 432.0, 554.0, &light)));
    objects.push(Arc::new(XZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, &white)));
    objects.push(Arc::new(XZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, &white)));
    objects.push(Arc::new(XYRect::new(0.0, 555.0, 0.0, 555.0, 555.0, &white)));

    // boxes
    let box1: Arc<dyn Hittable> = Arc::new(Box::new(&Point3::new(0.0, 0.0, 0.0), &Point3::new(165.0, 330.0, 165.0), &white));
    let box1: Arc<dyn Hittable> = Arc::new(RotateY::new(&box1, 15.0));
    let box1: Arc<dyn Hittable> = Arc::new(Translate::new(&box1, &Vec3::new(265.0, 0.0, 295.0)));

    let box2: Arc<dyn Hittable> = Arc::new(Box::new(&Point3::new(0.0, 0.0, 0.0), &Point3::new(165.0, 165.0, 165.0), &white));
    let box2: Arc<dyn Hittable> = Arc::new(RotateY::new(&box2, -18.0));
    let box2: Arc<dyn Hittable> = Arc::new(Translate::new(&box2, &Vec3::new(130.0, 0.0, 65.0)));
    
    objects.push(Arc::new(ConstantMedium::new(&box1, 0.01, Color::new(0.0, 0.0, 0.0))));
    objects.push(Arc::new(ConstantMedium::new(&box2, 0.01, Color::new(1.0, 1.0, 1.0))));

    objects
}

fn final_scene() -> HittableList {
    let mut boxes1 = HittableList::new();
    let ground: Arc<dyn Material> = Arc::new(Lambertian::new_color(Color::new(0.48, 0.83, 0.53)));

    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + i as f64*w;
            let z0 = -1000.0 + j as f64*w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = random_double(1.0, 101.0);
            let z1 = z0 + w;

            let p0 = Point3::new(x0, y0, z0);
            let p1 = Point3::new(x1, y1, z1);
            boxes1.push(Arc::new(Box::new(&p0, &p1, &ground)));
        }
    }

    let mut objects = HittableList::new();
    let len = boxes1.len();
    objects.push(Arc::new(BvhNode::new(&mut boxes1, 0, len, 0.0, 1.0)));

    let light: Arc<dyn Material> = Arc::new(DiffuseLight::new_color(Color::new(7.0, 7.0, 7.0)));
    objects.push(Arc::new(XZRect::new(123.0, 423.0, 147.0, 412.0, 554.0, &light)));

    let center1 = Point3::new(400.0, 400.0, 200.0);
    let center2 = center1 + Vec3::new(30.0, 0.0, 0.0);
    let moving_sphere_material: Arc<dyn Material> = Arc::new(Lambertian::new_color(Color::new(0.7, 0.3, 0.1)));
    objects.push(Arc::new(MovingSphere::new(center1, center2, 0.0, 1.0, 50.0, &moving_sphere_material)));

    let d_mat: Arc<dyn Material> = Arc::new(Dielectric::new(1.5));
    let m_mat: Arc<dyn Material> = Arc::new(Metal::new(Color::new(0.8, 0.8, 0.9), 1.0));
    objects.push(Arc::new(Sphere::new(Point3::new(260.0, 150.0, 45.0), 50.0, &d_mat)));
    objects.push(Arc::new(Sphere::new(Point3::new(0.0, 150.0, 145.0), 50.0, &m_mat)));

    let boundary: Arc<dyn Hittable> = Arc::new(Sphere::new(Point3::new(360.0, 150.0, 145.0), 70.0, &d_mat));
    objects.push(boundary.clone());
    objects.push(Arc::new(ConstantMedium::new(&boundary, 0.2, Color::new(0.2, 0.4, 0.9))));
    let boundary: Arc<dyn Hittable> = Arc::new(Sphere::new(Point3::new(0.0, 0.0, 0.0), 5000.0, &d_mat));
    objects.push(Arc::new(ConstantMedium::new(&boundary, 0.0001, Color::new(1.0, 1.0, 1.0))));

    let earth_text: Arc<dyn Texture> = Arc::new(ImageTexture::new("./resources/earthmap.jpg"));
    let earth_mat: Arc<dyn Material> = Arc::new(Lambertian::new_texture(&earth_text));
    objects.push(Arc::new(Sphere::new(Point3::new(400.0, 200.0, 400.0), 100.0, &earth_mat)));
    let perlin_text: Arc<dyn Texture> = Arc::new(NoiseTexture::new(0.1));
    let perlin_mat: Arc<dyn Material> = Arc::new(Lambertian::new_texture(&perlin_text));
    objects.push(Arc::new(Sphere::new(Point3::new(220.0, 280.0, 300.0), 80.0, &perlin_mat)));

    let mut boxes2 = HittableList::new();
    let white: Arc<dyn Material> = Arc::new(Lambertian::new_color(Color::new(0.73, 0.73, 0.73)));
    let ns = 1000;
    for _j in 0..ns {
        boxes2.push(Arc::new(Sphere::new(Point3::random(0.0, 165.0), 10.0, &white)));
    }

    let len = boxes2.len();
    let bvhlist: Arc<dyn Hittable> = Arc::new(BvhNode::new(&mut boxes2, 0, len, 0.0, 1.0));
    let r_y: Arc<dyn Hittable> = Arc::new(RotateY::new(&bvhlist, 15.0));

    objects.push(Arc::new(Translate::new(&r_y, &Vec3::new(-100.0, 270.0, 395.0))));

    objects
}