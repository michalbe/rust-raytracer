use crate::ray::Ray;
use vec3D::Vec3D;

pub struct Camera {
    pub origin: Vec3D,
    pub lower_left_corner: Vec3D,
    pub horizontal: Vec3D,
    pub vertical: Vec3D,
}

impl Camera {
    pub fn new(look_from: Vec3D, look_at: Vec3D, up: Vec3D, vfov: f64, aspect: f64) -> Camera {
        let theta = vfov * std::f64::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;

        let w = (look_from - look_at).unit();
        let u = up.cross(w).unit();
        let v = w.cross(u);
        let origin = look_from;

        Camera {
            lower_left_corner: origin - u * half_width - v * half_height - w,
            horizontal: u * half_width * 2.0,
            vertical: v * half_height * 2.0,
            origin,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(self.origin, self.lower_left_corner + (self.horizontal * u) + (self.vertical * v) - self.origin)
    }
}
