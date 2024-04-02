mod poly_d2;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    for arg in args.iter() {
        println!("{}", arg);
    }

    let poly = poly_d2::Polynome2S::new(1.0, 2.0, 3.0);

    println!("poly: {:?}", poly);
}
