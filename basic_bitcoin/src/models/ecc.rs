//! Elliptic Curve Cryptograpy
//! std::hash is used instead of Sha256


use std::error::Error;
use std::fmt::Display;
use std::ops::{Add, Sub, Mul,Div};

#[allow(unused_imports)]
use num_traits::pow::Pow;
#[allow(unused_imports)]
use num::{BigInt, ToPrimitive};

#[derive(Clone, Debug)]
pub struct FieldElement {
    num: i128,
    prime: i128,   
}

#[allow(dead_code)]
impl FieldElement {
    fn new(num: i128, prime: i128) -> Result<Self, Box<dyn Error>> {
        if num >= prime {
            let msg = format!("Num {} not in field range 0 to {}", num, prime - 1);
            return Err(msg.into());
        } 

        Ok(
            FieldElement { 
                num, 
                prime,
        })
    }

    fn get_num(&self) -> i128 {
        self.num
    }

    fn get_prime(&self) -> i128 {
        self.prime
    }

    fn power_of(self, exponent: i32) -> Self {
        let n = exponent % (self.prime as i32 - 1);
        // let base = BigInt::from(self.num);
        // let ex = BigInt::from(n);
        // let modulus = BigInt::from(self.prime);
        // let new_num = base.modpow(&ex, &modulus).to_i128().unwrap();
        let new_num = ((self.num as f64).powi(n)) as i128 % self.prime;

        Self {
            num: new_num,
            prime: self.prime,
        }
    }

    fn rmul(self, coefficient: i128) -> Self {
        let num = (self.num * coefficient) % self.prime;
        Self {
            num,
            prime: self.prime,
        }
    }
}

impl Display for FieldElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FieldElement_{} ({})", self.get_prime(), self.get_num())
    }
}

impl PartialEq for FieldElement {
    fn eq(&self, other: &Self) -> bool {
        return self.num == other.num && self.prime == other.prime;
    }
}

impl Eq for FieldElement {}

impl Add for FieldElement {
    type Output = FieldElement;

    fn add(self, rhs: Self) -> Self::Output {
        if &self.prime != &rhs.prime {
            panic!("Cannot add two numbers in different Fields");
        }
        let num = (self.num - rhs.num) % self.prime;

        return Self {
            num,
            prime: self.prime,
        }
    }
}

impl<'a> Add<&'a FieldElement> for &'a FieldElement {
    type Output = FieldElement;
    
    fn add(self, rhs: &'a FieldElement) -> Self::Output {
        if self.prime != rhs.prime {
            panic!("Cannot add two numbers in different Fields");
        }
        let num = (self.num + rhs.num) % self.prime;

        return FieldElement {
            num,
            prime: self.prime,
        }
    }
}

#[allow(unused_assignments)]
impl Sub for FieldElement {
    type Output = FieldElement;

    fn sub(self, rhs: Self) -> Self::Output {
        if &self.prime != &rhs.prime {
            panic!("Cannot subtract two numbers in different Fields");
        }
        let mut num = 0_i128;

        if &self.num >= &rhs.num {
            num = (self.num - rhs.num) % self.prime;
        } else {
            num = (self.prime - (rhs.num - self.num)) % self.prime;
        }

        return FieldElement {
            num,
            prime: self.prime,
        }
    }
}

#[allow(unused_assignments)]
impl<'a> Sub<&'a FieldElement> for &'a FieldElement {
    type Output = FieldElement;

    fn sub(self, rhs: &'a FieldElement) -> Self::Output {
        if &self.prime != &rhs.prime {
            panic!("Cannot subtract two numbers in different Fields");
        }
        let mut num = 0_i128;

        if &self.num >= &rhs.num {
            num = (self.num - rhs.num) % self.prime;
        } else {
            num = (self.prime - (rhs.num - self.num)) % self.prime;
        }

        return FieldElement {
            num,
            prime: self.prime,
        }
    }
}

impl Mul for FieldElement {
    type Output = FieldElement;

    fn mul(self, rhs: Self) -> Self::Output {
        if &self.prime != &rhs.prime {
            panic!("Cannot multifly two numbers in different Fields");
        }
        
        let num = (self.num * rhs.num) % self.prime;

        return FieldElement {
            num,
            prime: self.prime,
        }
    }
}

impl<'a> Mul<&'a FieldElement> for &'a FieldElement {
    type Output = FieldElement;

    fn mul(self, rhs: &'a FieldElement) -> Self::Output {
        if &self.prime != &rhs.prime {
            panic!("Cannot multifly two numbers in different Fields");
        }
        
        let num = (self.num * rhs.num) % self.prime;

        return FieldElement {
            num,
            prime: self.prime,
        }
    }
}

impl Div for FieldElement {
    type Output = FieldElement;

    fn div(self, rhs: Self) -> Self::Output {
        if &self.prime != &rhs.prime {
            panic!("Cannot divide two numbers in different Fields");
        }

        let num = (self.num * rhs.clone().power_of(rhs.prime as i32 - 2).num) % self.prime;

        Self {
            num,
            prime: self.prime,
        }
    }
}

impl<'a> Div<&'a FieldElement> for &'a FieldElement {
    type Output = FieldElement;

    fn div(self, rhs: &'a FieldElement) -> Self::Output {
        if self.prime != rhs.prime {
            panic!("Cannot divide two numbers in different Fields");
        }

        let num = (self.num * rhs.clone().power_of(rhs.prime as i32 - 2).num) % self.prime;

        FieldElement {
            num,
            prime: self.prime,
        }
    }
}

#[cfg(test)]
mod ecc_tests {

    use super::*;
    use num::FromPrimitive;

    #[test]
    fn power() {
        let a: f64 = 6.0;
        println!("{}", (6_u128^100_u128) % 6_u128);
        println!("{}", (a.powf(100.0) % 6.0) as u128 );
        println!("{}", (a.powf(10.0) % 6.0) as u128 );
        println!("{}", (6_i8 ^ 100) % 6 );
        println!("{}", (622 ^ 289) % 941 );
        println!("{}", (622.0_f64.powf(289.0) % 941.0) as usize );
    }

    #[test]
    fn pow_test() -> Result<(), Box<dyn Error>>{
        let fe2_7 = FieldElement::new(2, 7)?;
        let fe4_7 = FieldElement::new(4, 7)?;

        let fe2_4 = FieldElement::new(2, 4)?;
        // let fe0_4 = FieldElement::new(0, 4)?;
        
        
        assert!(fe2_7.power_of(4) == fe4_7.power_of(2));
        // assert!(fe2_4.clone().power_of(4) == fe0_4);

        println!("{:?}", fe2_4.power_of(3));
        Ok(())
    }

    #[test]
    fn modpow_test() {
        let a = BigInt::from_u8(2).unwrap();
        println!("{}", (2 ^ 4) % 4 );
        println!("{}", a.modpow(&BigInt::from_u8(4).unwrap(), &BigInt::from_u8(4).unwrap()));
    }
    #[test]
    fn test_ne() -> Result<(), Box<dyn Error>> {
        let a = FieldElement::new(2, 31)?;
        let b = FieldElement::new(2, 31)?;
        let c = FieldElement::new(15, 31)?;

        assert!(&a == &b);
        assert!(a != c);

        Ok(())
    }

    #[test]
    fn test_add() -> Result<(), Box<dyn Error>> {
        let a = FieldElement::new(2, 31)?;
        let b = FieldElement::new(15, 31)?;
        let c = FieldElement::new(17, 31)?;
        let d = FieldElement::new(21, 31)?;
        let e = FieldElement::new(7, 31)?;

        assert!(&a + &b == c.clone());
        assert!(&(&c + &d) == &e);
        Ok(())
    }

    #[test]
    fn test_sub() -> Result<(), Box<dyn Error>> {
        let a = FieldElement::new(29, 31)?;
        let b = FieldElement::new(4, 31)?;
        let c = FieldElement::new(25, 31)?;
        let d = FieldElement::new(15, 31)?;
        let e = FieldElement::new(30, 31)?;
        let f = FieldElement::new(16, 31)?;

        assert!(&a - &b == c.clone());
        assert!(d - e == f);
        // println!("{:?}", d - e);
        Ok(())
    }

    #[test]
    fn test_mul() -> Result<(), Box<dyn Error>> {
        let a = FieldElement::new(24, 31)?;
        let b = FieldElement::new(19, 31)?;
        let c = FieldElement::new(22, 31)?;

        assert!(&a * &b == c);
        Ok(())
    }

    #[test]
    fn test_rmul() -> Result<(), Box<dyn Error>> {
        let a = FieldElement::new(24, 31)?;

        assert!(a.clone().rmul(2) == &a + &a);
        Ok(())
    }

    #[test]
    fn test_pow() -> Result<(), Box<dyn Error>> {
        let a = FieldElement::new(17, 31)?;
        let b = FieldElement::new(15, 31)?;
        assert!(a.clone().power_of(3) == b);

        let c = FieldElement::new(5, 31)?;
        let d = FieldElement::new(18, 31)?;
        let e = FieldElement::new(16, 31)?;
        assert!(c.power_of(5) * d == e);
        
        Ok(())
    }

    #[test]
    fn test_div() -> Result<(), Box<dyn Error>> {
        // let a = FieldElement::new(3, 31)?;
        // let b = FieldElement::new(24, 31)?;
        // let c = FieldElement::new(4, 31)?;
        // assert!(a / b == c);

        // let d = FieldElement::new(17, 31)?;
        // let e = FieldElement::new(29, 31)?;
        // assert!(d.power_of(-3) == e);

        // let f = FieldElement::new(4, 31)?;
        
        Ok(())
    }
}