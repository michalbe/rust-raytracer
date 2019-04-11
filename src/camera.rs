use crate::ray::Ray;
use vec3D::Vec3D;

pub struct Camera {
    pub origin: Vec3D,
    pub lower_left_corner: Vec3D,
    pub horizontal: Vec3D,
    pub vertical: Vec3D,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            lower_left_corner: Vec3D::new(-2.0, -1.0, -1.0),
            horizontal :Vec3D::new(4.0, 0.0, 0.0),
            vertical: Vec3D::new(0.0, 2.0, 0.0),
            origin: Vec3D::new(0.0, 0.0, 0.0),
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(self.origin, self.lower_left_corner + (self.horizontal * u) + (self.vertical * v))
    }
}
