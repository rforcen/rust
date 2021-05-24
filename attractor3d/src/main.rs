mod attr3d;
mod color_interp;
mod vector3d;
use attr3d::*;

fn all() {
    // 'mkdir wrl' before running 
    for n_eval in 0..=1 {
        for preset in 0..17 {
            let attr = Attr3d::new(preset, n_eval);
            attr.write_wrl(&*format!("wrl/attr{}-{}.wrl", n_eval, preset));
        }
    }
}

fn main() {
    all()
}
