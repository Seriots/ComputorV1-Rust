#![allow(dead_code)]

use std::fmt::Display;
use std::ops::{Add, Neg, Sub};
use std::vec;


pub union Solution {
    real: f32,
    complex: (f32, f32),
}

///Result struct from polynome
#[derive(Debug)]
pub struct PolyRoots {
    pub roots: Option<Vec<f32>>,
    pub all_reals: bool,
    pub degree: u8,
}

impl PolyRoots {
    pub fn new(roots: Option<Vec<f32>>, all_reals: bool, degree: u8) -> Self {
        PolyRoots { roots, all_reals, degree }
    }
}

impl Display for PolyRoots {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        
        if self.roots == None {
            if self.all_reals {
                return write!(f, "Each number is a solution")
            }
            else {
                return write!(f, "There is no solution")
            }
        }
        else {
            let roots = self.roots.as_ref().unwrap();
            match (self.degree, roots.len()) {
                (1, 1) => {
                    return write!(f, "The solution is:\n{}", roots[0])
                },
                (2, 1) => {
                    return write!(f, "Discriminant is zero, the solution is:\n{}", roots[0])
                },
                (2, 2) => {
                    return write!(f, "Discriminant is strictly positive, the two solutions are:\n{}\n{}", roots[0], roots[1])
                },
                _ => {
                    return write!(f, "The polynomial degree is strictly greater than 2, I can't solve.")
                }

            }
        }
    }

}

/// Polynome2S is a struct that represents a 2nd degree polynome
#[derive(Debug)]
pub struct Polynome2S {
    pub a: f32,
    pub b: f32,
    pub c: f32,
}


impl Polynome2S {
    /// Create a new Polynome2S
    pub fn new(a: f32, b: f32, c: f32) -> Self {
        Polynome2S { a, b, c }
    }


    pub fn from_polypart(polypart: &Vec<PolynomePart>) -> Option<Self> {
        let mut a = 0.0;
        let mut b = 0.0;
        let mut c = 0.0;

        for part in polypart.iter() {
            match part.power {
                2 => a = part.coef,
                1 => b = part.coef,
                0 => c = part.coef,
                _ => return None
            }
        }
        Some(Polynome2S { a, b, c })
    }

    pub fn compute_2nd_degree(&self) -> PolyRoots {
        let delta = self.b * self.b - 4.0 * self.a * self.c;

        if delta > 0.0 {
            let x1 = (-self.b - delta.sqrt()) / 2.0 * self.a;
            let x2 = (-self.b + delta.sqrt()) / 2.0 * self.a;
            PolyRoots {roots: Some(vec![x1, x2]), all_reals: false, degree: 2}
        }
        else if delta < 0.0 {
            PolyRoots {roots: None, all_reals: false, degree: 2}
        }
        else {
            let x1 = -self.b / 2.0 * self.a;
            PolyRoots {roots: Some(vec![x1]), all_reals: false, degree: 2}
        }
    }

    pub fn compute_1st_degree(&self) -> PolyRoots {
        let root = -self.c / self.b;

        PolyRoots {roots: Some(vec![root]), all_reals: false, degree: 1}

    }

    pub fn compute_0nd_degree(&self) -> PolyRoots {

       let mut all_reals = false;
        if self.c == 0.0 {
            all_reals = true;
        }
        PolyRoots { roots: None, all_reals, degree: 0}
    }
    /// Get the degree of the polynome
    pub fn get_poly_degree(&self) -> u8 {
        if self.a != 0.0 {
            return 2
        }
        else if self.b != 0.0 {
            return 1
        }
        else {
            return 0
        }
    }

    /// Compute the value of the polynome for a given x
    pub fn get_roots(&self) -> PolyRoots {
        match self.get_poly_degree() {
            2 => self.compute_2nd_degree(),
            1 => self.compute_1st_degree(),
            0 => self.compute_0nd_degree(),
            _ => panic!("Polynome degree unavailable")
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct PolynomePart {
    pub coef: f32,
    pub power: u8,
    pub opright: bool, 
}

impl PolynomePart {
    pub fn zero() -> Self {
        PolynomePart { coef: 0.0, power: 0, opright: false }
    }

    pub fn from_power(power: u8) -> Self {
        PolynomePart { coef: 0.0, power, opright: false }
    }

    pub fn sign_to_value(sign: char) -> f32 {
        match sign {
            '+' => 1.0,
            '-' => -1.0,
            _ => panic!("Sign not recognized")
        }
    }
}

impl Add for PolynomePart {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        if self.power != other.power {
            panic!("Power of the two polynome parts must be the same")
        }

        let coef = self.coef + other.coef ;

        PolynomePart { coef, power: self.power, opright: self.opright}
    }
}

impl Sub for PolynomePart {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.power != rhs.power {
            panic!("Power of the two polynome parts must be the same")
        }
        let coef = self.coef - rhs.coef;

        PolynomePart { coef, power: self.power, opright: self.opright }
    }
}

impl Neg for PolynomePart {
    type Output = Self;

    fn neg(self) -> Self::Output {
        PolynomePart { coef:-self.coef, power: self.power, opright: self.opright }
    }
}

impl Display for PolynomePart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let sign = if self.coef >= 0.0 {'+'} else {'-'};
        let coef = self.coef.abs();
        write!(f, " {} {}x^{}", sign, coef, self.power)
    }
}


#[cfg(test)]
#[test]
fn test_poly_2nd_2soluce()  {
    let poly = Polynome2S::new(1.0, -3.0, 2.0);
    let polyroots = poly.get_roots();
    assert_eq!(polyroots.roots, Some(vec![1.0, 2.0]));
    assert_eq!(polyroots.all_reals, false);
    assert_eq!(polyroots.degree, 2);
}

#[cfg(test)]
#[test]
fn test_poly_2nd_1soluce()  {
    let poly = Polynome2S::new(3.0, 6.0, 3.0);
    let polyroots = poly.get_roots();
    assert_eq!(polyroots.roots, Some(vec![-9.0]));
    assert_eq!(polyroots.all_reals, false);
    assert_eq!(polyroots.degree, 2);
}

#[cfg(test)]
#[test]
fn test_poly_2nd_0soluce()  {
    let poly = Polynome2S::new(1.0, 2.0, 3.0);
    let polyroots = poly.get_roots();
    assert_eq!(polyroots.roots, None);
    assert_eq!(polyroots.all_reals, false);
    assert_eq!(polyroots.degree, 2);
}

#[cfg(test)]
#[test]
fn test_poly_1st()  {
    let poly = Polynome2S::new(0.0, 3.0, 6.0);
    let polyroots = poly.get_roots();
    assert_eq!(polyroots.roots, Some(vec![-2.0]));
    assert_eq!(polyroots.all_reals, false);
    assert_eq!(polyroots.degree, 1);
}

#[cfg(test)]
#[test]
fn test_poly_0st_all_reals()  {
    let poly = Polynome2S::new(0.0, 0.0, 0.0);
    let polyroots = poly.get_roots();
    assert_eq!(polyroots.roots, None);
    assert_eq!(polyroots.all_reals, true);
    assert_eq!(polyroots.degree, 0);
}

#[cfg(test)]
#[test]
fn test_poly_0st_nosoluce()  {
    let poly = Polynome2S::new(0.0, 0.0, 6.0);
    let polyroots = poly.get_roots();
    assert_eq!(polyroots.roots, None);
    assert_eq!(polyroots.all_reals, false);
    assert_eq!(polyroots.degree, 0);
}

