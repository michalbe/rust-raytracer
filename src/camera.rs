use crate::ray::Ray;
use vec3D::Vec3D;

fn random_in_unit_disk() -> Vec3D {
    let mut p: Vec3D;

    loop {
        p = Vec3D::new(rand::random::<f64>(), rand::random::<f64>(), 0.0) * 2.0 - Vec3D::new(1.0, 1.0, 0.0);

        if p.dot(p) < 1.0 {
            break;
        }
    }
    p
}

pub struct Camera {
    pub origin: Vec3D,
    pub lower_left_corner: Vec3D,
    pub horizontal: Vec3D,
    pub vertical: Vec3D,
    pub lens_radius: f64,
    u: Vec3D,
    v: Vec3D,
}

impl Camera {
    pub fn new(look_from: Vec3D, look_at: Vec3D, up: Vec3D, vfov: f64, aspect: f64, aperture: f64, focus_distance: f64) -> Camera {
        let theta = vfov * std::f64::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;

        let w = (look_from - look_at).unit();
        let u = up.cross(w).unit();
        let v = w.cross(u);
        let origin = look_from;

        Camera {
            lower_left_corner: origin - u * focus_distance * half_width - v * focus_distance * half_height - w * focus_distance,
            horizontal: u * half_width * 2.0 * focus_distance,
            vertical: v * half_height * 2.0 * focus_distance,
            origin,
            lens_radius: aperture / 2.0,
            u,
            v,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = random_in_unit_disk() * self.lens_radius;
        let offset = self.u * rd.x + self.v * rd.y;
        Ray::new(self.origin + offset, self.lower_left_corner + (self.horizontal * s) + (self.vertical * t) - self.origin - offset)
    }
}
