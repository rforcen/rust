mod pi_decimals;
use pi_decimals::generate_pi_decimals;

use std::env;

fn main() {
    for argument in env::args().skip(1) {
        println!(
            "{}",
            generate_pi_decimals(argument.parse::<usize>().expect("bad integer"))
        );
    }
}
