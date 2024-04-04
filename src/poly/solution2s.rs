use std::fmt::{Debug, Display};


pub trait Solution2s: Display + Debug {}


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

impl Display for ComplexeSolution2s {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} + {}i) / {}", self.numerator, self.delta, self.denominator)
    }
}

impl Solution2s for ComplexeSolution2s {}


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
        write!(f, "{} {} sqrt({}) / {}", self.numerator, if self.delta_sign < 0 {'-'} else {'+'} ,self.delta, self.denominator)
    }
}

impl Solution2s for RealSolution2s {}


impl Solution2s for f32 {}
