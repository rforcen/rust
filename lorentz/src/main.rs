mod lorentz;
use lorentz::Lorentz;
mod color_interp;
mod vector3d;

fn cli() {
    Lorentz::new().write_wrl("lorentz.wrl");
}

fn main() {
    cli();
}
