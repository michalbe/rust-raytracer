extern crate vec3D;
extern crate rand;

mod ray;
mod hitable;
mod sphere;
mod camera;

use vec3D::Vec3D;
use ray::Ray;
use hitable::{HitRecord, Hitable, HitableList};
use sphere::Sphere;
use camera::Camera;

fn color(ray: Ray, world: &Hitable) -> Vec3D {
    let mut rec = HitRecord::new();
    if world.hit(&ray, 0.0, std::f64::MAX, &mut rec) {
        return Vec3D::new(rec.normal.x + 1.0, rec.normal.y + 1.0, rec.normal.z + 1.0) * 0.5;
    } else {
        let unit_direction = ray.direction.unit();
        let t = 0.5 * (unit_direction.y + 1.0);
        return (Vec3D::new(1.0, 1.0, 1.0) * (1.0 - t)) + (Vec3D::new(0.5, 0.7, 1.0) * t);
    }
}


fn main() {
    let nx = 400;
    let ny = 200;
    let ns = 100;

    // let mut rng = rand::thread_rng();

    println!("P3\n{} {} 255", nx, ny);

    let mut world = HitableList {
        list: vec![]
    };

    world.list.push(Box::new(Sphere {
        center: Vec3D::new(0.0, 0.0, -1.0),
        radius: 0.5,
    }));

    world.list.push(Box::new(Sphere {
        center: Vec3D::new(0.0, -100.5, -1.0),
        radius: 100.0,
    }));

    let camera = Camera::new();

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3D::new(0.0, 0.0, 0.0);

            for _ in 0..ns {
                let u = (i as f64 + rand::random::<f64>()) / nx as f64;
                let v = (j as f64 + rand::random::<f64>()) / ny as f64;
                let ray = camera.get_ray(u, v);
                col += color(ray, &world);
            }


            col /= ns as f64;

            // println!("{}, t", col);
            let r = (255.00 * col.x).round();
            let g = (255.00 * col.y).round();
            let b = (255.00 * col.z).round();
            println!("{} {} {}", r, g, b);
        }
    }
}
