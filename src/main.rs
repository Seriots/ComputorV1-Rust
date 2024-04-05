mod poly;
mod parsing;
mod reduce;
mod test;

use std::env;

use crate::poly::PolyRoots;
use parsing::parse_input;
use reduce::reduce_expression;

fn main() {
    let args: Vec<String> = env::args().collect();

    // skip first argument
    let mut iter = args.iter();
    iter.next();
    let mut first = true;
    for arg in iter {
        if first {
            first = false;
        } else {
            println!();
        }
        if let Some(user_input) = parse_input(arg.to_string()) {
            let reduced_poly = reduce_expression(user_input);
             
            if let Some(poly2s) = poly::Polynome2S::from_polypart(&reduced_poly) {
                let polyroots = poly2s.get_roots();
                    println!("The polynomial degree: {}", polyroots.degree);
                    println!("{}", polyroots);
            }
            else {
                let max_power = reduced_poly.iter().max_by(|a, b| a.power.cmp(&b.power)).unwrap().power;
                let polyroots = PolyRoots::new(None, false, max_power, 0.0);
                println!("The polynomial degree: {}", polyroots.degree);
                println!("{}", polyroots);
            }
        }
    }
}



