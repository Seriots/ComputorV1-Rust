
#[cfg(test)]
use crate::parsing::parse_input;

#[cfg(test)]
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

#[cfg(test)]
#[test]
fn sign_test_polyparser() {
    assert_eq!(None, parse_input("-+-+-+--".to_string()));
}

#[cfg(test)]
#[test]
fn float_test_polyparser() {
    assert_eq!(None, parse_input(". = 9".to_string()));
    assert_ne!(None, parse_input(".9 = 9".to_string()));
    assert_ne!(None, parse_input("9. = 9".to_string()));
    assert_ne!(None, parse_input("99 = 9".to_string()));
    assert_ne!(None, parse_input("99.99 = 9".to_string()));
}

#[cfg(test)]
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