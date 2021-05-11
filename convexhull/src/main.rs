mod convexhull;
use convexhull::ConvexHull;
mod test;
mod waterman;

fn main() {
    let rad = 32.;
    let v = waterman::gen_waterman_flat(rad);
    let ch = ConvexHull::new(v);
    // println!("{:.1?}\n\nok={}", ch, ch.check());
    ch.write_obj(&*format!("{}.obj", rad))
}
