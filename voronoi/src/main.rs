mod voronoi;
use voronoi::*;
use std::time::Instant;

fn main() {
    let mf = 6;
    let (w, h, n_points) = (800*mf, 800*mf, 400*mf);
    let mut v = Voronoi::new(w, h, n_points);

    println!("generating voronoi for {}x{}={} pix, {} points...", w, h, w*h, n_points);
    let t = Instant::now();

    v.generate();
    
    println!("lap: {:?}", Instant::now()-t);

    v.write_png("voronoi.png");
}
