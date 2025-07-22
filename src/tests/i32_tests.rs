use crate::imaginary_numbers::Complex;

#[test]
fn add_complex_i32() {
    let one = Complex::new(3, 3);
    let two = Complex::new(5, -4);
    let result = one+two;
    let result_expected = Complex::new(8, -1);
    assert_eq!(result, result_expected);
}
#[test]
fn conjugate_i32() {
    let one = Complex::new(33, 35);
    let result = one.conjugate();
    let result_expected = Complex::new(33, -35);
    assert_eq!(result, result_expected);
}
#[test]
fn equal_i32() {
    let one = Complex::new(3, 3);
    let two = Complex::new(5, -4);
    let tree = Complex::new(5, -4);
    if one== two{
        panic!()
    }
    if tree != two{
        panic!()
    }
}
#[test]
fn mul_complex_i32(){
    let one = Complex::new(3, 3);
    let two = Complex::new(5, 4);
    let result = one*two;
    let result_expected = Complex::new(3 , 27);
    assert_eq!(result, result_expected);
}
#[test]
fn neg_complex_i32(){
    let one = Complex::new(3, 27);
    let result = -one;
    let result_expected = Complex::new(-3 , -27);
    assert_eq!(result, result_expected);
}
#[test]
fn sub_complex_i32() {
    let one = Complex::new(3, 3);
    let two = Complex::new(5, -4);
    let result = one-two;
    let result_expected = Complex::new(-2, 7);
    assert_eq!(result, result_expected);
}
