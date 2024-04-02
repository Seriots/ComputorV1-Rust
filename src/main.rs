mod poly_d2;

use std::env;

use poly_d2::PolynomePart;

/// Parse the input arguments
/// 5 * X^0 + 4 * X^1 - 9.3 * X^2 = 1 * X^0
/// format: [+/-:?][whitespace:?][0-9][[.:?][0-9]:?][whitespace:?][*:?][whitespace:?][x/X][whitespace:?][^/**][whitespace:?][0-9] (repeat) [whitespace:?][=][whitespace:?][0-9][[.:?][0-9]:?][whitespace:?][*:?][whitespace:?][x/X][whitespace:?][^/**][whitespace:?][0-9 
/// split block on +/-/= and parse each block

/// Parse an integer
/// @param input: the string to parse
/// @return a tuple with the integer and the length of the parsed string
fn parse_int(input: &String) -> (i32, i32) {
    return (0, input.len() as i32)
}

/// Parse a float
/// @param input: the string to parse
/// @return a tuple with the float and the length of the parsed string
fn parse_float(input: &String) -> (f32, i32) {
    return (0.0, input.len() as i32)
}

/// Parse a polynome part
/// @param input: the string to parse
/// @param opright: true if the polynome part is on the right side of the equation
/// @return a tuple with the polynome part and the length of the parsed string
fn parse_polypart(input: &String, opright: bool) -> (PolynomePart, i32) {
    return (PolynomePart { sign: '+', coef: 0.0, power: 0, opright: opright }, 1)
}

/// Parse the input arguments
/// We suppose that all block are polynome part so we run parse_polypart on each block
fn parse_input(input: String) {

    let mut all_part: Vec<PolynomePart> = Vec::new();
    let mut buffer = input.trim().to_string();
    let mut opright = false;

    while buffer.len() > 0 {
        let (new_part, end) = parse_polypart(&buffer, opright);
        
        buffer = buffer[end as usize..].trim().to_string();
        all_part.push(new_part);

        if buffer.starts_with("=") {
            buffer = buffer[1..].trim().to_string();
            opright = true;
        }
    }
    for elem in all_part.iter() {
        println!("{:?}", elem);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut iter = args.iter();
    iter.next();
    for arg in iter {
        parse_input(arg.to_string())
    }

    let poly = poly_d2::Polynome2S::new(0.0, 6.0, 0.0);

    println!("poly: {:?}", poly);

    let polyroots = poly.get_roots();

    println!("The polynomial degree: {}", polyroots.degree);
    println!("{}", polyroots);
}
