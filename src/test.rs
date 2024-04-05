#![cfg(test)]

use crate::poly::Polynome2S;
use crate::parsing::parse_input;

#[test]
fn general_test_polyparser() {
    assert_ne!(None, parse_input("1 * x + 54 * x ^ 1 - 40 * x ^ 2 = 3 * X ^ 2".to_string()));
    assert_eq!(None, parse_input("5+1*x^2 +45X^1 - 20X^2 + 30X + 20= X - 20X -40X^2 + 80 -80X9.5 + 20".to_string()));
    assert_ne!(None, parse_input("1x = 0".to_string()));
    assert_ne!(None, parse_input("1 = 0".to_string()));
    assert_ne!(None, parse_input("1x^3 = 0".to_string()));
    assert_eq!(None, parse_input("1* = 0".to_string()));
    assert_eq!(None, parse_input("^5 = 0".to_string()));
    assert_eq!(None, parse_input("1^5 = 0".to_string()));
    assert_eq!(None, parse_input("+1^5 = 0".to_string()));
    assert_ne!(None, parse_input("5+-+--1x = 0".to_string()));
    assert_ne!(None, parse_input("5+1x2 = 0".to_string()));
}

#[test]
fn sign_test_polyparser() {
    assert_eq!(None, parse_input("-+-+-+--".to_string()));
}

#[test]
fn float_test_polyparser() {
    assert_eq!(None, parse_input(". = 9".to_string()));
    assert_ne!(None, parse_input(".9 = 9".to_string()));
    assert_ne!(None, parse_input("9. = 9".to_string()));
    assert_ne!(None, parse_input("99 = 9".to_string()));
    assert_ne!(None, parse_input("99.99 = 9".to_string()));
}

#[test]
fn x_test_polyparser() {
    assert_ne!(None, parse_input("x = 9".to_string()));
    assert_eq!(None, parse_input("x*5 = 9".to_string()));
    assert_eq!(None, parse_input("xa5 = 9".to_string()));
    assert_ne!(None, parse_input("x**5 = 9".to_string()));
    assert_eq!(None, parse_input("x   * *5 = 9".to_string()));
    assert_ne!(None, parse_input("X^5 = 9".to_string()));
    assert_eq!(None, parse_input("X^^5 = 9".to_string()));
    assert_eq!(None, parse_input("X^ ^5 = 9".to_string()));
    assert_ne!(None, parse_input("1*X = 9".to_string()));
    assert_eq!(None, parse_input("1**X = 9".to_string()));
    assert_ne!(None, parse_input("98X = 9".to_string()));
    assert_ne!(None, parse_input("98 X = 9".to_string()));
}

#[test]
fn test_poly_2nd_2soluce()  {
    let poly = Polynome2S::new(1.0, -3.0, 2.0);
    let polyroots = poly.get_roots();
    assert_eq!(polyroots.compute(), Some(vec![1.0, 2.0]));
    assert_eq!(polyroots.all_reals, false);
    assert_eq!(polyroots.degree, 2);
}

#[cfg(test)]
#[test]
fn test_poly_2nd_1soluce()  {
    let poly = Polynome2S::new(3.0, 6.0, 3.0);
    let polyroots = poly.get_roots();
    assert_eq!(polyroots.compute(), Some(vec![-1.0]));
    assert_eq!(polyroots.all_reals, false);
    assert_eq!(polyroots.degree, 2);
}

#[cfg(test)]
#[test]
fn test_poly_2nd_0soluce()  {
    let poly = Polynome2S::new(2.0, 2.0, 2.0);
    let polyroots = poly.get_roots();
    assert_eq!(polyroots.compute(), Some(vec![-1.3660254, 0.3660254]));
    assert_eq!(polyroots.all_reals, false);
    assert_eq!(polyroots.degree, 2);
}

#[cfg(test)]
#[test]
fn test_poly_1st()  {
    let poly = Polynome2S::new(0.0, 3.0, 6.0);
    let polyroots = poly.get_roots();
    assert_eq!(polyroots.compute(), Some(vec![-2.0]));
    assert_eq!(polyroots.all_reals, false);
    assert_eq!(polyroots.degree, 1);
}

#[cfg(test)]
#[test]
fn test_poly_0st_all_reals()  {
    let poly = Polynome2S::new(0.0, 0.0, 0.0);
    let polyroots = poly.get_roots();
    assert_eq!(polyroots.compute(), None);
    assert_eq!(polyroots.all_reals, true);
    assert_eq!(polyroots.degree, 0);
}

#[cfg(test)]
#[test]
fn test_poly_0st_nosoluce()  {
    let poly = Polynome2S::new(0.0, 0.0, 6.0);
    let polyroots = poly.get_roots();
    assert_eq!(polyroots.compute(), None);
    assert_eq!(polyroots.all_reals, false);
    assert_eq!(polyroots.degree, 0);
}

