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

