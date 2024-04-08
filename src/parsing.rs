
use crate::poly::PolynomePart;

/// Parse an integer
/// @param input: the string to parse
/// @param is_exponent: true if there is an exponent symbol before
/// @param is_x: true if there is a x symbol before
/// @return a tuple with the integer and the length of the parsed string and an error code if error has occured
fn parse_power(input: String, is_exponent: bool, is_x: bool) -> (Option<u8>, usize, i8) {
    let mut i = 0;

    if is_x == false {
        return (Some(0), i, 0);
    }
    for char in input.chars() {
        if char == '.' {
            return (None, i, -1);
        }
        if char < '0' || char > '9' {
            break;
        }
        i += 1;
    }

    if i == 0 {
        if is_exponent {
            return (None, i, -1);
        } else {
            return (Some(1), i, 0);
        }
    }
    let power = input[..i].parse::<u8>();
    if power.is_err() {
        return (None, i, -1);
    }
    return (Some(power.unwrap()), i, 0);

}

/// Parse a float
/// @param input: the string to parse
/// @param is_signed: true if there is a sign before
/// @return a tuple with the float and the length of the parsed string and an error code if error has occured
fn parse_float(input: String) -> (Option<f32>, usize, i8) {
    let mut i = 0;
    let mut dot = 0;
    for char in input.chars() {
        if char == '.' && dot == 0 {
            dot += 1;
        }
        else if char < '0' || char > '9' {
            break;
        }
        i += 1;
    }
    if i == 0 {
        return (Some(1.0), i, 0);
    }
    let float = input[..i].parse::<f32>();
    if float.is_err() {
        return (None, i, -1);
    }
    return (Some(float.unwrap()), i, 0)
}

///Parse the sign, we can have multiple sign (+/-) and the result is equivalent to the multiplication between all of them
/// @param input: the string to parse
/// @return a tuple with the sign and the length of the parsed string
fn parse_sign(input: String, first: bool) -> (Option<char>, usize, i8) {
    let mut i = 0;
    let mut sign_count = 0;
    for char in input.chars() {
        if char == '-' {
            sign_count += 1;
        } else if char != '+' {
            break ;
        }
        i += 1;
    }
    if !first && i == 0 {
        return (None, i, -1);
    } 
    (Some(if sign_count % 2 == 0 {'+'} else {'-'}), i, 0)
}

/// Parse a polynome part
/// @param input: the string to parse
/// @param opright: true if the polynome part is on the right side of the equation
/// @return a tuple with the polynome part and the length of the parsed string
fn parse_polypart(input: String, opright: bool, first: bool) -> (Option<PolynomePart>, usize, i8) {
    let input_base_length = input.len();
    let mut buffer = input;
    let mut is_multiplier = false;
    let mut is_x = false;
    let mut is_exponent = false;
    let (sign, end, error) = parse_sign(buffer.clone(), first);
    if error < 0 {
        return (None, input_base_length - buffer.len(), error)
    }
    buffer = buffer[end..].trim_start().to_string();
    if buffer.len() <= 0 {
        return (None, input_base_length - buffer.len(), -1);
    }
    let (coef, end, error) = parse_float(buffer.clone());
    if error < 0 {
        return (None, input_base_length - buffer.len(), error)
    }
    let signed_coef = coef.unwrap() * PolynomePart::sign_to_value(sign.unwrap());
    buffer = buffer[end..].trim_start().to_string();
    if buffer.starts_with("*") {
        buffer = buffer[1..].trim().to_string();
        is_multiplier = true;
    }
    if buffer.starts_with("x") || buffer.starts_with("X") {
        buffer = buffer[1..].trim().to_string();
        is_x = true;
    } else if is_multiplier == true {
        return (None, input_base_length - buffer.len(), -1);
    }
    if buffer.starts_with("^") && is_x == true {
        buffer = buffer[1..].trim().to_string();
        is_exponent = true;
    } else if buffer.starts_with("**") && is_x == true {
        buffer = buffer[2..].trim().to_string();
        is_exponent = true;
    }
    let (power, end, error) = parse_power(buffer.clone(), is_exponent, is_x);
    
    if error < 0 {
        return (None, input_base_length - buffer.len(), error);
    }
    buffer = buffer[end..].trim_start().to_string();
    if signed_coef != 0.0 {
        return (Some(PolynomePart { coef: signed_coef, power: power.unwrap(), opright }), input_base_length - buffer.len(), 0)
    } else {
        return (None, input_base_length - buffer.len(), 0);
    }
}

pub fn display_error(base: &String, error_index: usize) -> Option<Vec<PolynomePart>> {
    if error_index >= base.len() - 1 {
        println!("Error in input: '\x1b[1m{}\x1b[0m'", base);
    } else {
        println!("Error in input: '\x1b[1m{}\x1b[91m{}\x1b[0m\x1b[1m{}\x1b[0m'", base[..error_index].to_string(), base[error_index..error_index + 1].to_string(), base[error_index + 1..].to_string());
    }
    println!("\x1b[1m{}", "~".repeat(error_index as usize + 17) + "\x1b[91m^\x1b[0m");
    println!("Invalid input format please format each part as 'ax^b' with a, b as float and integer");
    return None;
}

/// Parse the input arguments
/// We suppose that all block are polynome part so we run parse_polypart on each block
pub fn parse_input(input: String) -> Option<Vec<PolynomePart>> {

    let mut all_part: Vec<PolynomePart> = Vec::new();
    let mut buffer = input.trim().to_string();
    let mut opright = false;
    let mut first = true;

    while buffer.len() > 0 {
        let (new_part, end, error) = parse_polypart(buffer.clone(), opright, first);
        if first {
            first = false;
        }
        if end > buffer.len() {
            return display_error(&input, input.len() - 1);
        }
        buffer = buffer[end..].trim_start().to_string();
        if error < 0 {
            return display_error(&input, input.len() - buffer.len());
        }
        if end == 0 {
            return display_error(&input, input.len() - buffer.len());
        }
        
        if !new_part.is_none() {
            all_part.push(new_part.unwrap());
        }

        if buffer.starts_with("=") {
            if opright == true {
                return display_error(&input, input.len() - buffer.len());
            }
            first = true;
            buffer = buffer[1..].trim_start().to_string();
            opright = true;
        }
    }
    return Some(all_part);
}
