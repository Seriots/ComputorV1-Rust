use std::fmt::Debug;


#[derive(Debug, Clone, Copy)]
pub struct ComplexeSolution2s {
    pub delta: f32,
    pub numerator: f32,
    pub denominator: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct RealSimpleSolution2s {
    pub numerator: f32,
    pub denominator: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct RealSolution2s {
    pub delta: f32,
    pub numerator: f32,
    pub denominator: f32,
}

pub enum Solution {
    RealSolution2s,
    RealSimpleSolution2s,
    ComplexeSolution2s,
    F32,
}

//impl Solution {
//    pub fn new() -> Self {
//        Solution { float: 0.0 }
//    }

//    pub fn from_float(float: f32) -> Self {
//        Solution { float }
//    }

//    pub fn from_real(delta: f32, numerator: f32, denominator: f32) -> Self {
//        Solution { real: RealSolution2s { delta, numerator, denominator } }
//    }

//    pub fn from_real_simple(numerator: f32, denominator: f32) -> Self {
//        Solution { real_simple: RealSimpleSolution2s { numerator, denominator } }
//    }

//    pub fn from_complexe(delta: f32, numerator: f32, denominator: f32) -> Self {
//        Solution { complexe: ComplexeSolution2s { delta, numerator, denominator } }
//    }
//}

//impl PartialEq for Solution {
//    fn eq(&self, other: &Self) -> bool {
//        unsafe {self.float == other.float}
//    }
//}

//impl Debug  for Solution {
//    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//        write!(f, "{}", self.float)
//    }
//}