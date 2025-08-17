use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(PartialEq, Eq,Debug, Clone, Copy)]
pub struct Quaterion<T>{
    pub real: T,
    pub i_part: T,
    pub j_part: T,
    pub k_part: T
}


impl <T> Quaterion<T>{
    pub fn new(real:T, i_part:T, j_part:T, k_part:T)-> Quaterion<T>{
        return Quaterion{real,i_part,j_part,k_part};
    }
}

impl <T: Neg<Output = T>> Quaterion<T> 
{
    pub fn conjugate(self)-> Self{
        return Self::new(self.real, -self.i_part, -self.j_part, -self.k_part);
    }

}

impl <T: Mul<Output = T>+Sub<Output = T>+Add<Output = T>+Copy> Mul for Quaterion<T> { // https://www.aero.iitb.ac.in/satelliteWiki/index.php/File:Equation16.JPG
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let a1 = self.real;
        let a2 = rhs.real;
        let b1 = self.i_part;
        let b2 = rhs.i_part;
        let c1 = self.j_part;
        let c2 = rhs.j_part;
        let d1 = self.k_part;
        let d2= rhs.k_part;
        let an = (a1*a2) - (b1*b2) - (c1*c2) - (d1*d2);
        let bn = (a1*b2) + (b1*a2) + (c1*d2) - (d1*c2);
        let cn = (a1*c2) - (b1*d2) + (c1*a2) + (d1*b2);
        let dn = (a1*d2) + (b1*c2) - (c1*b2) + (d1*a2);
        return Quaterion{real:an, i_part:bn, j_part:cn, k_part:dn};
    }
}
