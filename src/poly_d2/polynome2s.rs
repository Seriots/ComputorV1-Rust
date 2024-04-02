use std::vec;


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
        
    }

    pub fn from_1st_degree(poly: &Polynome2S) -> Self {
        
    }

    pub fn from_0nd_degree(poly: &Polynome2S) -> Self {

       let  mut all_reals = false;
        if poly.c == 0.0 {
            all_reals = true;
        }
        Self { roots: None, all_reals, degree: 0}
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