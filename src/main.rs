mod poly;
mod parsing;

use std::env;

use poly::PolynomePart;

use crate::poly::PolyRoots;
use parsing::parse_input;



fn display_expression(poly_parts: &Vec<PolynomePart>, reduced_poly: &Vec<PolynomePart>, power_reduced: &Vec<u8>) {
    let mut is_opright = false;
    for elem in reduced_poly.iter() {
        print!("{}", elem)
    }

    for elem in poly_parts.iter() {
        if !power_reduced.contains(&elem.power) {
            if is_opright != elem.opright {
                is_opright = elem.opright;
                print!(" =")
            }
            print!("{}", elem);
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

    let mut iter = args.iter();
    iter.next();
    for arg in iter {
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

#[cfg(test)]
#[test]
fn general_test_polyparser() {
    assert_ne!(None, parse_input("1 * x + 54 * x ^ 1 - 40 * x ^ 2 = 3 * X ^ 2".to_string()));
    assert_eq!(None, parse_input("5+1*x^2 +45X^1 - 20X^2 + 30X + 20= X - 20X -40X^2 + 80 -80X9.5 + 20".to_string()));
    assert_ne!(None, parse_input("1x = 0".to_string()));
    assert_ne!(None, parse_input("1 = 0".to_string()));
    assert_ne!(None, parse_input("1x^3 = 0".to_string()));
    assert_eq!(None, parse_input("1* = 0".to_string()));
    assert_eq!(None, parse_input("^5 = 0".to_string()));
    assert_eq!(None, parse_input("1^5 = 0".to_string()));
    assert_eq!(None, parse_input("+1^5 = 0".to_string()));
    assert_ne!(None, parse_input("5+-+--1x = 0".to_string()));
    assert_ne!(None, parse_input("5+1x2 = 0".to_string()));
}

#[cfg(test)]
#[test]
fn sign_test_polyparser() {
    assert_eq!(None, parse_input("-+-+-+--".to_string()));
}

#[cfg(test)]
#[test]
fn float_test_polyparser() {
    assert_eq!(None, parse_input(". = 9".to_string()));
    assert_ne!(None, parse_input(".9 = 9".to_string()));
    assert_ne!(None, parse_input("9. = 9".to_string()));
    assert_ne!(None, parse_input("99 = 9".to_string()));
    assert_ne!(None, parse_input("99.99 = 9".to_string()));
}

#[cfg(test)]
#[test]
fn x_test_polyparser() {
    assert_ne!(None, parse_input("x = 9".to_string()));
    assert_eq!(None, parse_input("x*5 = 9".to_string()));
    assert_eq!(None, parse_input("xa5 = 9".to_string()));
    assert_ne!(None, parse_input("x**5 = 9".to_string()));
    assert_eq!(None, parse_input("x   * *5 = 9".to_string()));
    assert_ne!(None, parse_input("X^5 = 9".to_string()));
    assert_eq!(None, parse_input("X^^5 = 9".to_string()));
    assert_eq!(None, parse_input("X^ ^5 = 9".to_string()));
    assert_ne!(None, parse_input("1*X = 9".to_string()));
    assert_eq!(None, parse_input("1**X = 9".to_string()));
    assert_ne!(None, parse_input("98X = 9".to_string()));
    assert_ne!(None, parse_input("98 X = 9".to_string()));
}

