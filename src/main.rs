mod vec3;
mod ray; // TODO how to import in Rust??

use ray::Ray;
use vec3::Color;
use vec3::Point3;

use crate::vec3::Vec3;


fn main() {
    // Image
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const WIDTH: u32 = 1920;
    const HEIGHT: u32 = (WIDTH as f32 / ASPECT_RATIO) as u32;
    let mut imgbuf = image::ImageBuffer::new(WIDTH, HEIGHT);

    // Camera
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width as f64, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height as f64, 0.0);
    let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - Vec3::new(0.0, 0.0, focal_length);

    // Render
    for j in 0..HEIGHT {
        println!("Lines remaining: {}", HEIGHT - j);
        for i in 0..WIDTH {
            let u = i as f64 / (WIDTH-1) as f64;
            let v = j as f64 / (HEIGHT -1) as f64;
            let r = Ray::new(origin, lower_left_corner + u*horizontal + v*vertical - origin);
            let color = ray_color(&r).translate();

            imgbuf[(i, HEIGHT - j - 1)] = image::Rgb([color.x as u8, color.y as u8, color.z as u8]);
        }
    }

    // Save image
    imgbuf.save("render.png").unwrap();
}

fn ray_color(r: &Ray) -> Color {
    if hit_sphere(&Point3::new(0.0, 0.0, -1.0), 0.5, &r) {
        Color::new(1.0, 0.0, 0.0)
    }
    else {
        let unit_direction = r.direction().unit_vector();
        let t = 0.5 * (unit_direction.y + 1.0);
    
        (1.0-t)*Color::new(1.0, 1.0, 1.0) + t*Color::new(0.5, 0.7, 1.0)
    }
}

fn hit_sphere(center: &Point3, radius: f64, r: &Ray) -> bool {
    let oc = r.origin() - *center;
    let a = r.direction().dot(&r.direction());
    let b = 2.0 * oc.dot(&r.direction());
    let c = oc.dot(&oc) - radius*radius;
    let discriminant = b*b - 4.0*a*c;

    discriminant > 0.0
}