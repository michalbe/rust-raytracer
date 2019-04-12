use vec3D::Vec3D;

use crate::{
    ray::Ray,
    hitable::{HitRecord, Hitable},
    material::Scatter
};

pub fn random_in_unit_sphere() -> Vec3D {
    let mut p: Vec3D;

    loop {
        p = Vec3D::new(rand::random::<f64>(), rand::random::<f64>(), rand::random::<f64>()) * 2.0 - Vec3D::new(1.0, 1.0, 1.0);

        if p.mag2() < 1.0 {
            break;
        }
    }
    p
}

pub fn color(ray: Ray, world: &Hitable, depth: i32) -> Vec3D {
    let mut rec = HitRecord::new();
    if world.hit(&ray, 0.001, std::f64::MAX, &mut rec) {
        let mut scattered = Ray::new(Vec3D::new(0.0, 0.0, 0.0), Vec3D::new(0.0, 0.0, 0.0));
        let mut attenuation = Vec3D::new(0.0, 0.0, 0.0);

        if depth < 50 && rec.material.scatter(&ray, &rec, &mut attenuation, &mut scattered) {
            let new_color = color(scattered, world, depth + 1);
            return Vec3D::new(
                new_color.x * attenuation.x,
                new_color.y * attenuation.y,
                new_color.z * attenuation.z,
            );
            // return new_color.cross(attenuation);
        } else {
            return Vec3D::new(0.0, 0.0, 0.0);
        }
    } else {
        let unit_direction = ray.direction.unit();
        let t = 0.5 * (unit_direction.y + 1.0);
        return (Vec3D::new(1.0, 1.0, 1.0) * (1.0 - t)) + (Vec3D::new(0.5, 0.7, 1.0) * t);
    }
}

pub fn reflect(v: Vec3D, n: Vec3D) -> Vec3D {
    v - n * v.dot(n) * 2.0
}
