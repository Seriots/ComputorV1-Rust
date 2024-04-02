#![allow(dead_code)]

use std::fmt::Display;
use std::vec;


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

    pub fn from_2nd_degree(poly: &Polynome2S) -> Self {
        let delta = poly.b * poly.b - 4.0 * poly.a * poly.c;

        if delta > 0.0 {
            let x1 = (-poly.b - delta.sqrt()) / 2.0 * poly.a;
            let x2 = (-poly.b + delta.sqrt()) / 2.0 * poly.a;
            Self {roots: Some(vec![x1, x2]), all_reals: false, degree: 2}
        }
        else if delta < 0.0 {
            Self {roots: None, all_reals: false, degree: 2}
        }
        else {
            let x1 = -poly.b / 2.0 * poly.a;
            Self {roots: Some(vec![x1]), all_reals: false, degree: 2}
        }
    }

    pub fn from_1st_degree(poly: &Polynome2S) -> Self {
        let root = -poly.c / poly.b;

        Self {roots: Some(vec![root]), all_reals: false, degree: 1}

    }

    pub fn from_0nd_degree(poly: &Polynome2S) -> Self {

       let  mut all_reals = false;
        if poly.c == 0.0 {
            all_reals = true;
        }
        Self { roots: None, all_reals, degree: 0}
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
            2 => PolyRoots::from_2nd_degree(self),
            1 => PolyRoots::from_1st_degree(self),
            0 => PolyRoots::from_0nd_degree(self),
            _ => panic!("Polynome degree unavailable")
        }
    }
}