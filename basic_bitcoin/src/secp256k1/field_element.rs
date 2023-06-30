//! Finite Field 정의 및 해다 구현체의 연산 정의
//! 
//! 해당 struct field type 은 i64 적용
//! 

use std::fmt::{Display};
use std::ops::{Add, Sub, Mul, Div};

#[derive(Debug, Clone)]
pub struct FieldElement {
    num: i64,
    prime: i64,
}


impl FieldElement {
    pub fn new(num: i64, prime: i64) -> Self { 
        Self {
            num,
            prime,
        }
    }

    pub fn get_prime(&self) -> i64 {
        self.prime
    }

    pub fn get_number(&self) -> i64 {
        self.num
    }

    pub fn set_number(&mut self, new_num: i64) {
        self.num = new_num;
    }

    pub fn set_prime(&mut self, new_prime: i64) {
        self.prime = new_prime
    }

    pub fn pow(&self, exponent: u32) -> Self {
        let n = exponent % (self.get_prime() as u32 - 1);
        let new_num = self.get_number().pow(n) % self.get_prime();
        FieldElement::new(new_num, self.get_prime())
    }
}


impl PartialEq for FieldElement {
    fn eq(&self, other: &Self) -> bool {
        return self.num == other.num  && self.prime == other.prime;
    }
}

impl<'a> PartialEq<&'a FieldElement> for FieldElement {
    fn eq(&self, other: &&'a Self) -> bool {
        return self.num == other.num  && self.prime == other.prime;
    }
}

impl Eq for FieldElement {}


impl Display for FieldElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FieldElement_{} ({})", self.get_prime(), self.get_number())
    }
}

impl Add for FieldElement {
    type Output = FieldElement;

    fn add(self, rhs: Self) -> Self::Output {
        if &self.prime != &rhs.prime {
            panic!("Cannot add two numbers in different Field.");
        }
        let new_num = (self.get_number() + rhs.get_number()) % self.get_prime();
        FieldElement::new(new_num, self.get_prime())
    }
}

impl<'a> Add<&'a FieldElement> for &'a FieldElement {
    type Output = FieldElement;

    fn add(self, rhs: &'a FieldElement) -> Self::Output {
        if &self.prime != &rhs.prime {
            panic!("Cannot add two numbers in different Field.");
        }
        let new_num = (self.get_number() + rhs.get_number()) % self.get_prime();
        FieldElement::new(new_num, self.get_prime())
    }
}

impl Sub for FieldElement {
    type Output = FieldElement;

    fn sub(self, rhs: Self) -> Self::Output {
        if &self.prime != &rhs.prime {
            panic!("Cannot subtract two numbers in different Field.");
        }

        let diff_num = (self.get_number() - rhs.get_number()) % self.get_prime();
        if diff_num >= 0 {
            return FieldElement::new(diff_num, self.get_prime());
        } else {
            return FieldElement::new(self.get_prime() - diff_num, self.get_prime());
        }
     }
}

impl<'a> Sub<&'a FieldElement> for &'a FieldElement {
    type Output = FieldElement;

    fn sub(self, rhs: &'a FieldElement) -> Self::Output {
        if &self.prime != &rhs.prime {
            panic!("Cannot subtract two numbers in different Field.");
        }

        let diff_num = (self.get_number() - rhs.get_number()) % self.get_prime();
        if diff_num >= 0 {
            return FieldElement::new(diff_num, self.get_prime());
        } else {
            return FieldElement::new(self.get_prime() - diff_num, self.get_prime());
        }
    }
}

impl Mul for FieldElement {
    type Output = FieldElement;

    fn mul(self, rhs: Self) -> Self::Output {
        if &self.prime != &rhs.prime {
            panic!("Cannot multiply two numbers in different Field.");
        }
        let new_num = self.get_number() * rhs.get_number() % self.get_prime();
        FieldElement::new(new_num, self.get_prime())
    }
}

impl<'a> Mul<&'a FieldElement> for &'a FieldElement {
    type Output = FieldElement;

    fn mul(self, rhs: &'a FieldElement) -> Self::Output {
        if &self.prime != &rhs.prime {
            panic!("Cannot multiply two numbers in different Field.");
        }
        let new_num = self.get_number() * rhs.get_number() % self.get_prime();
        FieldElement::new(new_num, self.get_prime())
    }
}

impl Div for FieldElement {
    type Output = FieldElement;

    fn div(self, rhs: Self) -> Self::Output {
        if &self.get_prime() != &rhs.get_prime() {
            panic!("Cannot divide two numbers in different Field.");
        }

        let new_num = self.get_number() * rhs.get_number().pow(self.get_prime() as u32 - 2) % self.get_prime();
        return FieldElement::new(new_num, self.get_prime())
    }
}

impl<'a> Div<&'a FieldElement> for &'a FieldElement {
    type Output = FieldElement;
    
    fn div(self, rhs: &'a FieldElement) -> Self::Output {
        if &self.get_prime() != &rhs.get_prime() {
            panic!("Cannot divide two numbers in different Field.");
        }

        let new_num = self.get_number() * rhs.get_number().pow(self.get_prime() as u32 - 2) % self.get_prime();
        return FieldElement::new(new_num, self.get_prime())
    }
}

// impl Rem for FieldElement {
//     type Output = FieldElement;

//     fn rem(self, rhs: Self) -> Self::Output {
//         if &self.get_prime() != &rhs.get_prime() {
//             panic!("Cannot oprate remainder between two numbers in different Field.");
//         }

//     }
// }

#[cfg(test)]
mod field_element_tests {
    use super::*;
 

    #[test]
    fn field_element_eq() {
        let a = FieldElement::new(100, 100);
        let b = FieldElement::new(100, 100);
        let c = FieldElement::new(100, -100);

        assert!(a == b);
        assert!(a != c);
        assert!(&a == &b);
        assert!(&a != &c);
    }

    #[test]
    fn field_element_add_sub() {
   
    }

    #[test]
    fn field_element_mul_div() {

    }


    #[test]
    fn xxx() {

    }

 
}