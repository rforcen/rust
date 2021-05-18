mod sh;
use sh::*;
mod wrl;
use wrl::Wrl;
use std::io::{self, Write};

fn generate_all() {
    for code in 0..SH_N_CODES {
        let sh = SpericalHarmonics::new(128, code as u32, 1).generate();
        Wrl::write_IndexedFaceSet(&*format!("wrl/{}.wrl", code), &sh.shape, &sh.faces);
        print!("{} ", code);
        io::stdout().flush().unwrap();
    }
}
fn gen1() {
    let sh = SpericalHarmonics::new(256, 1, 1).generate();
    Wrl::write_IndexedFaceSet("sh.wrl", &sh.shape, &sh.faces)
}

fn main() {
    generate_all()
}
