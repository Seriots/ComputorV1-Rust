use crate::poly::Sqrt;

pub fn get_prime_factor(n: u32, sign: i8) -> Vec<i32> {
    let mut i = 2;
    let mut n = n;
    let mut result: Vec<i32> = Vec::new();
    result.push(sign as i32);
    while n >= i {
        if n % i == 0 {
            result.push(i as i32);
            n /= i;
        }
        else {
            i += 1;
        }
    }
    return result;
}

pub fn sqrt_from_prime_factor(v: Vec<i32>) -> Sqrt {
    let mut result: Sqrt = Sqrt::new(Vec::new(), None);
    let mut previous: Option<&i32> = None;
    let mut first = true;

    for elem in v.iter() {
        if first {
            result.whole_part.push(*elem);
            first = false;
        }
        else if previous == None {
            previous = Some(elem);
        }
        else {
            if previous.unwrap() == elem {
                result.whole_part.push(*elem);
                previous = None;
            }
            else {
                result.more_sqrt(*previous.unwrap() as u32);
                previous = Some(elem);
            }
        }
    }
    if previous != None {
        result.more_sqrt(*previous.unwrap() as u32);
    }
    return result;
}

fn factorise(first: &mut Vec<i32>, second: &mut Vec<i32>) -> (Vec<i32>, Vec<i32>, Vec<i32>) {
    let mut new: Vec<i32> = Vec::new();
    let mut start = true;
    for elem in first.iter() {
        if start {
            start = false;
            if elem == &-1 {
                continue ;
            }
        }
        if second.contains(&elem) {
            new.push(*elem);
            second.remove(second.iter().position(|&x| x == *elem).unwrap());
        }
    }
    for elem in new.iter() {
        first.remove(first.iter().position(|&x| x == *elem).unwrap());
    }
    
    return (new, first.to_vec(), second.to_vec());
}

pub fn simplify_expression(delta: u32, numerator: i32, denominator: i32, delta_sign: i8) -> (i32, i32, Sqrt, i32) {
    let mut facto_part: Vec<i32>;
    let mut denominator_part: Vec<i32> = get_prime_factor(denominator.abs() as u32, denominator.signum() as i8);
    let mut delta_part: Sqrt = sqrt_from_prime_factor(get_prime_factor(delta, delta_sign));
    let mut numerator_part: Vec<i32>;
    if delta_part.sqrt_part == None {
        let new_numerator = numerator + delta_part.whole_part.iter().product::<i32>();
        facto_part = get_prime_factor(new_numerator.abs() as u32, new_numerator.signum() as i8);
        numerator_part = vec![0];
        delta_part.whole_part = vec![1];
    } else if numerator as u32 == 0 {
        numerator_part = vec![0];
        facto_part = delta_part.whole_part;
        delta_part.whole_part = vec![1];
    } else {
        numerator_part = get_prime_factor(numerator.abs() as u32, numerator.signum() as i8);
        (facto_part, numerator_part, delta_part.whole_part) = factorise(&mut numerator_part, &mut delta_part.whole_part);
    }

    (_, facto_part, denominator_part) = factorise(&mut facto_part, &mut denominator_part);
    if denominator_part.iter().product::<i32>() <= -1 {
        denominator_part.push(-1);
        numerator_part.push(-1);
        delta_part.whole_part.push(-1);
    }

    return (facto_part.iter().product::<i32>(), numerator_part.iter().product::<i32>(), delta_part, denominator_part.iter().product::<i32>());
}