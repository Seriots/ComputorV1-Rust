mod poly_d2;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    for arg in args.iter() {
        println!("{}", arg);
    }

    let poly = poly_d2::Polynome2S::new(0.0, 6.0, 0.0);

    println!("poly: {:?}", poly);

    let polyroots = poly.get_roots();

    println!("The polynomial degree: {}", polyroots.degree);
    println!("{}", polyroots);
}
