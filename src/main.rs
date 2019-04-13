extern crate vec3D;
extern crate rand;
extern crate rayon;

use vec3D::Vec3D;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

mod utils;
mod ray;
mod hitable;
mod sphere;
mod camera;
mod material;

use hitable::HitableList;
use sphere::Sphere;
use camera::Camera;
use material::{ Material, Lambert, Metal, Dielectric };
use utils::color;

// fn scene(world: &mut HitableList) -> bool {


//     true
// }

fn main() {
    let nx = 600;
    let ny = 300;
    let ns = 100;

    println!("P3\n{} {} 255", nx, ny);


    let mut world = HitableList {
        list: vec![]
    };

    world.list.push(Sphere::new(
        Vec3D::new(0.0, -1000.0, 0.0),
        1000.0,
        Material::Lambert(Lambert::new(Vec3D::new(0.5, 0.5, 0.5)))
    ));

    for a in -11..11 {
        for b in -11..11 {
            let material = rand::random::<f64>();
            let center = Vec3D::new((a as f64) + 0.9 * rand::random::<f64>(), 0.2, (b as f64) + 0.9 * rand::random::<f64>());
            if (center - Vec3D::new(4.0, 0.2, 0.0)).mag() > 0.9 {
                if material < 0.8 {
                    world.list.push(Sphere::new(
                        center,
                        0.2,
                        Material::Metal(Metal::new(Vec3D::new(
                            0.5 * (1.0 + rand::random::<f64>()),
                            0.5 * (1.0 + rand::random::<f64>()),
                            0.5 * (1.0 + rand::random::<f64>())
                        ), 0.5 * rand::random::<f64>()))
                    ));
                } else {
                    world.list.push(Sphere::new(
                        center,
                        0.2,
                        Material::Lambert(Lambert::new(Vec3D::new(
                            rand::random::<f64>() * rand::random::<f64>(),
                            rand::random::<f64>() * rand::random::<f64>(),
                            rand::random::<f64>() * rand::random::<f64>()
                        )))
                    ));
                }
            }
        }
    }
    world.list.push(Sphere::new(
        Vec3D::new(0.0, 1.0, 0.0),
        1.0,
        Material::Dielectric(Dielectric::new(1.5))
    ));

    world.list.push(Sphere::new(
        Vec3D::new(-4.0, 1.0, 0.0),
        1.0,
        Material::Lambert(Lambert::new(Vec3D::new(0.4, 0.2, 0.1)))
    ));

    world.list.push(Sphere::new(
        Vec3D::new(4.0, 1.0, 0.0),
        1.0,
        Material::Metal(Metal::new(Vec3D::new(0.7, 0.6, 0.5), 0.0))
    ));


    let look_from = Vec3D::new(13.0, 2.0, 3.0);
    let look_at = Vec3D::new(0.0, 0.0, 0.0);
    let distance_to_focus = 10.0;
    let aperture = 0.1;

    let camera = Camera::new(
        look_from,
        look_at,
        Vec3D::new(0.0, 1.0, 0.0),
        20.0,
        nx as f64 / ny as f64,
        aperture,
        distance_to_focus
    );

    let pixels: Vec<Vec<(f64, f64, f64)>> =  (0..ny).into_par_iter().map(|j| {
        let j = ny as f64 - j as f64 - 1.0;
        let row: Vec<(f64, f64, f64)> = (0..nx).into_par_iter().map(|i| {
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

            (r, g, b)
        }).collect();

        row

    }).collect();

    for row in pixels {
        for (r, g, b) in row {
            println!("{} {} {}", r, g, b);
        }
    }
}
