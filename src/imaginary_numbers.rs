use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(PartialEq, Eq,Debug, Clone, Copy)]
pub struct Complex<T>{
    pub real: T,
    pub imaginary: T
}

impl <T> Complex<T>{
    pub fn new(real:T, imaginary: T)-> Complex<T>{
        return Complex{real, imaginary};
    }
}

impl <T: Neg<Output = T>> Complex<T> 
{
    pub fn conjugate(self)-> Self{
        return Self::new(self.real, -self.imaginary);
    }

}

impl <T: Neg<Output = T>> Neg for Complex<T> 
{
    type Output = Self;
    
    fn neg(self) -> Self::Output {
        return Self::new(-self.real, -self.imaginary);
    }

} 

impl <T: Add<Output = T>> Add for Complex<T> 
{
    type Output = Self;

    fn add(self, rhs: Complex<T>) -> Self::Output {
        return Self::new(self.real + rhs.real, self.imaginary + rhs.imaginary);
    }
}

impl  <T: Sub<Output = T>> Sub for Complex<T>{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let real = self.real-rhs.real;
        let imaginary = self.imaginary - rhs.imaginary;
        return Complex{real, imaginary};
    }
}
    


impl <T: Mul<Output = T>+Sub<Output = T>+Add<Output = T>+Copy> Mul for Complex<T> { // (a+bi)(c+di)= ac +adi + cbi -bd= (ac-db) + (ad + cb)i
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let real = (self.real * rhs.real) - (self.imaginary * rhs.imaginary);
        let  imaginary = (self.real *rhs.imaginary) + (self.imaginary *rhs.real);
        return Complex{real,imaginary};
    }
}

impl  <T: Mul<Output = T>+Div<Output = T>+Sub<Output = T>+Add<Output = T>+Copy> Div for Complex<T>{ //(u+vi)/(x+yi) = (ux + vy)/(xx + yy) + (vx - uy)/(xx + yy)i
    type Output = Self;
    
    fn div(self, rhs: Self) -> Self::Output {
        let x = self.real;
        let y= self.imaginary;
        let u= rhs.real;
        let v = rhs.imaginary;
        let denominator = (x*x + y*y);
        let real_numerator = (u*x + v*y);
        let imaginary_numerator = (v*x - u*y);
        let real = real_numerator/denominator;
        let imaginary = imaginary_numerator/denominator;
        Complex{
            real,
            imaginary
        }
    }


}