use std::{cmp, fmt::{Debug, Display}};

use crate::poly::simplify_expression;

pub trait Solution2s: Display + Debug {
    fn compute(&self) -> f32;
}


#[derive(Debug, Clone, Copy)]
pub struct ComplexeSolution2s {
    pub delta: f32,
    pub delta_sign: i8,
    pub numerator: f32,
    pub denominator: f32,
}

impl ComplexeSolution2s {
    #[allow(dead_code)]
    pub fn new() -> Self {
        ComplexeSolution2s { delta: 0.0, delta_sign: 1, numerator: 0.0, denominator: 0.0 }
    }

    pub fn from(delta: f32, delta_sign: i8, numerator: f32, denominator: f32) -> Self {
        ComplexeSolution2s { delta, delta_sign, numerator, denominator }
    }
}

// facto(num + delta)
// ――――――――――――――――――
//       denom
impl Display for ComplexeSolution2s {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        let x_root;
        if f.precision() == None {
            x_root = 0;
        } else {
            x_root = f.precision().unwrap();
        }
        
        let (factorised_part, numerator_part, delta_part, denominator_part) = simplify_expression(self.delta as u32, self.numerator.trunc() as i32, self.denominator.trunc() as i32, self.delta_sign);
        let upper: String;
        let middle: String;
        let denom: String;
        let lower: String;
        denom = format!("{}", denominator_part);

        if factorised_part != 1 {
            upper = format!("{}({}{}i)", factorised_part, numerator_part, delta_part);
        } else {
            upper = format!("{}{}i", numerator_part, delta_part);
        }
        middle = "―".repeat(cmp::max(upper.len() - 2, denom.len()));
        lower = format!("{}{}{}", " ".repeat((((middle.len() / 3) - denom.len())).div_ceil(2)) ,denominator_part, " ".repeat(((middle.len() / 3) - denom.len()) / 2));
        if denominator_part != 1 {
            return write!(f, "     {}\nx{} = {}\n     {}", upper, x_root, middle, lower);
        }
        else {
            return write!(f, "x{} = {}", x_root, upper);
        }
    }
}

impl Solution2s for ComplexeSolution2s {
    fn compute(&self) -> f32 {
        (self.numerator + (self.delta.sqrt() * self.delta_sign as f32)) / self.denominator
    }
}


#[derive(Debug, Clone, Copy)]
pub struct RealSolution2s {
    pub delta: f32,
    pub delta_sign: i8,
    pub numerator: f32,
    pub denominator: f32,
}

impl RealSolution2s {
    #[allow(dead_code)]
    pub fn new() -> Self {
        RealSolution2s { delta: 0.0, delta_sign: 1, numerator: 0.0, denominator: 0.0 }
    }

    pub fn from(delta: f32, delta_sign: i8, numerator: f32, denominator: f32) -> Self {
        RealSolution2s { delta, delta_sign, numerator, denominator }
    }
}

impl Display for RealSolution2s {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let x_root;
        if f.precision() == None {
            x_root = 0;
        } else {
            x_root = f.precision().unwrap();
        }

        // If any value is not an integer, we display a float
        let whole_number = (self.numerator + self.delta_sign as f32 * self.delta.sqrt()) / self.denominator;
        if whole_number == whole_number.trunc() {
            return write!(f, "x{} = {}", x_root, whole_number);
        }
        if self.delta != self.delta.trunc() || self.denominator != self.denominator.trunc() || self.numerator != self.numerator.trunc(){
            return write!(f, "x{} = {}", x_root, (self.numerator + self.delta_sign as f32 * self.delta.sqrt()) / self.denominator);
        } else {
            let (factorised_part, numerator_part, delta_part, denominator_part) = simplify_expression(self.delta as u32, self.numerator.trunc() as i32, self.denominator.trunc() as i32, self.delta_sign);
            let mut upper: String;
            let middle: String;
            let denom: String;
            let lower: String;
            denom = format!("{}", denominator_part);
            if delta_part.sqrt_part == None {
                upper = format!("{}", factorised_part * (numerator_part + delta_part.whole_part.iter().product::<i32>()));
                middle = "―".repeat(cmp::max(upper.len(), denom.len()));
            } else {
                if factorised_part != 1 {
                    upper = format!("{}({}{})", factorised_part, numerator_part, delta_part);
                } else {
                    upper = format!("{}{}", numerator_part, delta_part);
                }
                middle = "―".repeat(cmp::max(upper.len() - 2, denom.len()));
            }
            lower = format!("{}{}{}", " ".repeat((((middle.len() / 3) - denom.len())).div_ceil(2)) ,denominator_part, " ".repeat(((middle.len() / 3) - denom.len()) / 2));
            if upper.len() < middle.len() / 3 {
                upper = format!("{}{}", " ".repeat((middle.len() / 3) - upper.len()), upper);
            }
            if denominator_part != 1 {
                return write!(f, "     {}\nx{} = {}\n     {}", upper, x_root, middle, lower);
            }
            else {
                return write!(f, "x{} = {}", x_root, upper);
            }
        }
    }
}

impl Solution2s for RealSolution2s {
    fn compute(&self) -> f32 {
        (self.numerator + (self.delta.sqrt() * self.delta_sign as f32)) / self.denominator
    }
}

