use crate::imaginary_numbers::Complex;

#[test]
fn add_complex_u32() {
    let one: Complex<u32> = Complex::new(3, 3);
    let two = Complex::new(5, 4);
    let result = one+two;
    let result_expected = Complex::new(8, 7);
    assert_eq!(result, result_expected);
}
#[test]
fn equal_u32() {
    let one:Complex<u32> = Complex::new(3, 3);
    let two = Complex::new(5, 4);
    let tree = Complex::new(5, 4);
    if one== two{
        panic!()
    }
    if tree != two{
        panic!()
    }
}
