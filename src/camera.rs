use crate::{vec3::{Vec3, Point3}, ray::Ray, util::degrees_to_radians};

pub struct Camera {
    origin: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3,
    u: Vec3,
    v: Vec3, 
   // w: Vec3,    // not used?
    lens_radius: f64
}

impl Camera {
    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * Vec3::random_in_unit_disk();
        let offset = self.u*rd.x + self.v*rd.y;
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s*self.horizontal + t*self.vertical - self.origin - offset
        )
    }

    pub fn new(
        look_from: Point3,
        look_at: Point3,
        vup: Vec3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64
    ) -> Camera {
        let theta = degrees_to_radians(vfov);
        let h = f64::tan(theta/2.0);
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from - look_at).unit_vector();
        let u = (Vec3::cross(&vup, &w)).unit_vector();
        let v = Vec3::cross(&w, &u);

        let origin = look_from;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - focus_dist*w;
        let lens_radius = aperture / 2.0;
        
        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            u, v, // w,
            lens_radius
        }
    }
}