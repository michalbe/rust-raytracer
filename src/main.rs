extern crate vec3D;

use vec3D::Vec3D;

fn main() {
    let max_i = 200;
    let max_j = 200;

    println!("P3\n{} {} 255", max_i, max_j);
    for i in 0..max_i {
        for j in 0..max_j {

            let col = Vec3D::new(i as f64 / max_i as f64, j as f64 / max_j as f64, 0.2);
            let r = (255.00 * col.x).round();
            let g = (255.00 * col.y).round();
            let b = (255.00 * col.z).round();
            println!("{} {} {}", r, g, b);
        }
    }
}
