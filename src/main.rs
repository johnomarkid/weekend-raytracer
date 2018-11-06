mod vec;

use std::fs::File;
use std::io;
use std::io::Write;
use vec::Vec3;

fn main() -> io::Result<()> {
    let nx = 200;
    let ny = 100;

    let mut f = File::create("color").expect("Unable to open file");
    f.write_fmt(format_args!("P3\n{} {}\n255\n", nx, ny))?;

    for j in (0..ny - 1).rev() {
        for i in 0..nx {
            let col = Vec3(i as f32 / nx as f32, j as f32 / ny as f32, 0.2);

            let ir = (255.99 * col.0) as u32;
            let ig = (255.99 * col.1) as u32;
            let ib = (255.99 * col.2) as u32;

            f.write_fmt(format_args!("{} {} {}\n", ir, ig, ib))?;
        }
    }

    Ok(())
}
