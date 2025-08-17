use crate::imaginary_numbers::Complex;
use crate::quaternions::Quaterion;
#[test]
fn add_complex_f32() {
    let one = Complex::new(3.4, 3.0);
    let two = Complex::new(5.2, 4.7);
    let result = one+two;
    let result_expected = Complex::new(8.6, 7.7);
    assert_eq!(result, result_expected);
}
#[test]
fn conjugate_f32() {
    let one = Complex::new(3.3, 3.5);
    let result = one.conjugate();
    let result_expected = Complex::new(3.3, -3.5);
    assert_eq!(result, result_expected);
}
#[test]
fn equal_f32() {
    let one = Complex::new(3.34, 3.77777);
    let two = Complex::new(5.4542, -4.3454);
    let tree = Complex::new(5.4542, -4.3454);
    if one== two{
        panic!()
    }
    if tree != two{
        panic!()
    }
}
#[test]
fn mul_complex_f32(){
    let one = Complex::new(1.1, 3.0);
    let two = Complex::new(5.0, 4.0);
    let result = one*two;
    let result_expected = Complex::new(5.5-12.0 , 4.4+15.0);
    assert_eq!(result, result_expected);
}
#[test]
fn neg_complex_f32(){
    let one = Complex::new(3.546, 27.98);
    let result = -one;
    let result_expected = Complex::new(-3.546 , -27.98);
    assert_eq!(result, result_expected);
}
#[test]
fn sub_complex_f32() {
    let one = Complex::new(3.5, 3.5);
    let two = Complex::new(1.45, -4.5);
    let result = one-two;
    let result_expected = Complex::new(2.05, 8.0);
    assert_eq!(result, result_expected);
}    



