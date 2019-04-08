fn main() {
    let max_i = 200;
    let max_j = 200;

    println!("P3\n{} {} 255", max_i, max_j);
    for i in 0..max_i {
        for j in 0..max_j {
            let r: f32 = i as f32 / max_i as f32;
            let g: f32 = j as f32 / max_j as f32;
            let b: f32 = 0.2;

            let ir = (255.00 * r).round();
            let ig = (255.00 * g).round();
            let ib = (255.00 * b).round();
            println!("{} {} {}", ir, ig, ib);
        }
    }
}
