

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

    pub fn 

    /// Compute the value of the polynome for a given x
    pub fn get_root(&self, x: f32) -> f32 {
        self.a * x * x + self.b * x + self.c
    }
}