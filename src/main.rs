use std::fs::File;
use std::io::Write;

fn main() {
    let nx = 200;
    let ny = 100;

    let mut f = File::create("color").expect("Unable to open file");
    f.write_fmt(format_args!("P3\n{} {}\n255\n", nx, ny));

    for j in (0..ny - 1).rev() {
        for i in 0..nx {
            let r: f32 = i as f32 / nx as f32;
            let g: f32 = j as f32 / ny as f32;
            let b: f32 = 0.2;

            let ir = (255.99 * r) as u32;
            let ig = (255.99 * g) as u32;
            let ib = (255.99 * b) as u32;

            f.write_fmt(format_args!("{} {} {}\n", ir, ig, ib));
        }
    }
}
