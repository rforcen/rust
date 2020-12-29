
mod mandelbrot;
use mandelbrot::*;
use std::time::Instant;

fn main() {
    let mf = 2;
    let (w, h, iters) = (mf*1024, mf*1024, 200);

    println!("Generating mandelbrot {} x {} = {} pix, {} iters...", w, h, w*h, iters);

    let mut mnd = Mandelbrot::new(w, h, ComplexF32::new(0.5, 0.0), ComplexF32::new(-2.0, 2.0), iters);

    let t = Instant::now();

    mnd.generate();

    println!("lap: {:?}", Instant::now() - t);

    mnd.write_png("mandel.png");
}
