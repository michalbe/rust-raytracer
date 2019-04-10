extern crate vec3D;

mod ray;

use vec3D::Vec3D;
use ray::Ray;

fn hit_sphere(center: Vec3D, radius: f64, ray: &Ray) -> bool {
    let oc = ray.origin - center;
    let a = ray.direction.dot(ray.direction);
    let b = oc.dot(ray.direction) * 2.0;
    let c = oc.dot(oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    discriminant > 0.0
}

fn color(ray: Ray) -> Vec3D {
    if hit_sphere(Vec3D::new(0.0, 0.0, -1.0), 0.5, &ray) {
        return Vec3D::new(1.0, 0.0, 0.0);
    }

    let unit_direction = ray.direction.unit();
    let t = 0.5 * (unit_direction.y + 1.0);
    (Vec3D::new(1.0, 1.0, 1.0) * (1.0 - t)) + (Vec3D::new(0.5, 0.7, 1.0) * t)
}


fn main() {
    let nx = 200;
    let ny = 100;

    println!("P3\n{} {} 255", nx, ny);

    let lower_left_corner = Vec3D::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3D::new(4.0, 0.0, 0.0);
    let vertical = Vec3D::new(0.0, 2.0, 0.0);
    let origin = Vec3D::new(0.0, 0.0, 0.0);

    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = i as f64 / nx as f64;
            let v = j as f64 / ny as f64;
            let r = Ray::new(origin, lower_left_corner + (horizontal * u) + (vertical * v));
            let col = color(r);
            // println!("{}, t", col);
            let r = (255.00 * col.x).round();
            let g = (255.00 * col.y).round();
            let b = (255.00 * col.z).round();
            println!("{} {} {}", r, g, b);
        }
    }
}
