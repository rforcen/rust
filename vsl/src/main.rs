

mod vsl;
use vsl::*;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() == 2 {   
        let mut vsl = Vsl::from_file(&args[1][..]);
        vsl.play();
     }
    else {
        println!("usage: vsl file.vsl\n plays vls file");
    }
}

