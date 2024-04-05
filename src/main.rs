mod poly;
mod parsing;
mod test;

use std::env;

use poly::PolynomePart;

use crate::poly::PolyRoots;
use parsing::parse_input;



fn display_expression(poly_parts: &Vec<PolynomePart>, reduced_poly: &Vec<PolynomePart>, power_reduced: &Vec<u8>) {
    let mut is_opright = false;
    let mut first = 1;

    for elem in reduced_poly.iter() {
        print!("{:.prec$}", elem, prec=first);
        if first == 1 {
            first = 0;
        }
    }

    for elem in poly_parts.iter() {
        if !power_reduced.contains(&elem.power) {
            if is_opright != elem.opright {
                is_opright = elem.opright;
                print!(" = ");
                first = 1;
            }
            print!("{:.prec$}", elem, prec=first);
            if first == 1 {
                first = 0;
            }
        }
    }
    if is_opright == false {
        print!(" = 0");
    }
    println!();
}

fn reduce_expression(poly_parts: Vec<PolynomePart>) -> Vec<PolynomePart> {
    let mut reduced_poly: Vec<PolynomePart> = Vec::new();
    let mut power_reduced: Vec<u8> = Vec::new();
    let mut steps = 1;
    print!("Step 0:       ");
    display_expression(&poly_parts, &reduced_poly, &power_reduced);
    for i in 0..poly_parts.len() {
        let power = poly_parts[i].power;
        if !power_reduced.contains(&power) {
            let mut display = false;
            let mut value = if poly_parts[i].opright == false { poly_parts[i].clone() } else { -poly_parts[i].clone() };
            for j in (i + 1)..poly_parts.len() {
                if poly_parts[j].power == power {
                    display = true;
                    value = if poly_parts[j].opright == false { value + poly_parts[j] } else { value - poly_parts[j] };
                }
            }
            reduced_poly.push(value);
            power_reduced.push(power);
            if display {
                print!("Step {}:       ", steps);
                steps += 1;
                display_expression(&poly_parts, &reduced_poly, &power_reduced);
            }
        }
    }

    reduced_poly.sort_by(|a, b| a.power.cmp(&b.power));
    print!("Reduced form: ");
    display_expression(&poly_parts, &reduced_poly, &power_reduced);
    return reduced_poly;
}


fn main() {
    let args: Vec<String> = env::args().collect();

    // skip first argument
    let mut iter = args.iter();
    iter.next();
    for arg in iter {
        if let Some(user_input) = parse_input(arg.to_string()) {
            let reduced_poly = reduce_expression(user_input);
             
            if let Some(poly2s) = poly::Polynome2S::from_polypart(&reduced_poly) {
                let polyroots = poly2s.get_roots();
                    println!("The polynomial degree: {}", polyroots.degree);
                    println!("{}\n", polyroots);
            }
            else {
                let max_power = reduced_poly.iter().max_by(|a, b| a.power.cmp(&b.power)).unwrap().power;
                let polyroots = PolyRoots::new(None, false, max_power, 0.0);
                println!("The polynomial degree: {}", polyroots.degree);
                println!("{}\n", polyroots);
            }
        }
    }
}



