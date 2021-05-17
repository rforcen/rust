mod lorentz;
use lorentz::Lorentz;

fn cli() {
    Lorentz::new().write_wrl("lorentz.wrl");
}

fn main() {
    cli();
}
