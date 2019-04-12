extern crate vec3D;
extern crate rand;

mod utils;
mod ray;
mod hitable;
mod sphere;
mod camera;
mod material;

use vec3D::Vec3D;
use hitable::HitableList;
use sphere::Sphere;
use camera::Camera;
use material::{ Material, Lambert, Metal, Dielectric };
use utils::color;

fn main() {
    let nx = 600;
    let ny = 300;
    let ns = 100;

    println!("P3\n{} {} 255", nx, ny);

    let mut world = HitableList {
        list: vec![]
    };

    world.list.push(Box::new(Sphere::new(
        Vec3D::new(0.0, 0.0, -1.0),
        0.5,
        Material::Lambert(Lambert::new(Vec3D::new(0.8, 0.3, 0.3)))
    )));

    world.list.push(Box::new(Sphere::new(
        Vec3D::new(0.0, -100.5, -1.0),
        100.0,
        Material::Lambert(Lambert::new(Vec3D::new(0.8, 0.8, 0.0)))
    )));

    world.list.push(Box::new(Sphere::new(
        Vec3D::new(1.0, 0.0, -1.0),
        0.5,
        Material::Metal(Metal::new(Vec3D::new(0.8, 0.6, 0.2), 0.0001))
    )));

    world.list.push(Box::new(Sphere::new(
        Vec3D::new(-1.0, 0.0, -1.0),
        0.5,
        Material::Dielectric(Dielectric::new(1.5))
    )));

    let camera = Camera::new();

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3D::new(0.0, 0.0, 0.0);

            for _ in 0..ns {
                let u = (i as f64 + rand::random::<f64>()) / nx as f64;
                let v = (j as f64 + rand::random::<f64>()) / ny as f64;
                let ray = camera.get_ray(u, v);
                col += color(ray, &world, 0);
            }

            col /= ns as f64;

            let r = (255.00 * col.x.sqrt()).round();
            let g = (255.00 * col.y.sqrt()).round();
            let b = (255.00 * col.z.sqrt()).round();
            println!("{} {} {}", r, g, b);
        }
    }
}
