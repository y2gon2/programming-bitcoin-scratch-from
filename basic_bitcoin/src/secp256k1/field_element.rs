//! Finite Field 정의 및 해다 구현체의 연산 정의
//! 
//! 해당 struct field 는 추후 SHA256 Hash value 를 포함할 수 있도록 BigUint type 으로 설
//! prime field 값은 책에 명시된 2^256 - 2^32 - 977 로 고정
//! 
//! BigUint type 을 기본으로 아래와 같이 Finite Field 연산 구현  
//! 현재 구현된 연산    : ==, != , +, -, *
//! 미구현된 연산       :/ , %
//! 거듭 제곱의 경우 exponent - BitUint type 으로 구현

use std::error::Error;
use std::fmt::{Display};
use std::ops::{Add, Sub, Mul, Rem, Div};

use num::{BigInt, BigUint, Zero, One, FromPrimitive};

#[derive(Debug, Clone)]
pub struct FieldElement {
    num: BigUint,
    prime: BigUint,
}


impl FieldElement {
    pub fn new(num: BigUint) -> Result<Self, Box<dyn Error>> {        
        let secp256k1_prime = 
            BigUint::from(2u64).pow(256) - BigUint::from(2u64).pow(32) - BigUint::from(977u64);
        
        if num >= secp256k1_prime {
            let msg = format!("Num {} not in field range 0 to {}", num, secp256k1_prime - BigUint::from(1u64));
            return Err(msg.into());
        }
        Ok(Self {
            num,
            prime: secp256k1_prime,
        })
    }

    pub fn get_prime(&self) -> &BigUint {
        &self.prime
    }

    pub fn get_number(&self) -> &BigUint {
        &self.num
    }

    pub fn set_number(&mut self, new_num: BigUint) {
        self.num = new_num;
    }

    pub fn zero() -> Self {
        FieldElement::new(BigUint::zero()).unwrap()
    }

    pub fn to_the_power_of(&self, exponent: BigUint) -> Self {
        let exp = exponent % (self.get_prime() - BigUint::from_u64(1u64).unwrap());
        let new_num = Self::mod_pow(self.num.clone(), exp.into(), &self.prime);
        FieldElement::new(new_num).unwrap()
    }

    // credit to https://rob.co.bb/posts/2019-02-10-modular-exponentiation-in-rust/
    fn mod_pow(mut base: BigUint, mut exp: BigUint, modulus: &BigUint) -> BigUint {
        if *modulus == BigUint::one() {
            return BigUint::zero();
        }
        let mut result = BigUint::one();
        base = base % modulus;
        while exp > BigUint::zero() {
            if &exp % BigUint::from_u64(2u64).unwrap() == BigUint::one() {
                result = result * &base % modulus;
            }
            exp = exp >> 1;
            base = base.clone() * base % modulus
        }
        result
    }
}


impl PartialEq for FieldElement {
    fn eq(&self, other: &Self) -> bool {
        return self.num == other.num  && self.prime == other.prime;
    }
}

impl Eq for FieldElement {}

impl Display for FieldElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FieldElement_{:64} ({:64})", self.prime.to_str_radix(16), self.num.to_str_radix(16))
    }
}

impl Add for FieldElement {
    type Output = FieldElement;

    fn add(self, other: Self) -> Self::Output {
        if self.prime != other.prime {
            panic!("Cannt add two numbers in different Field.(Prime value is not matched)");
        }
        let new_num = (self.num + other.num) % self.prime;
        FieldElement::new(new_num).unwrap()
    }
}

impl Sub for FieldElement {
    type Output = FieldElement;

    fn sub(self, other: Self) -> Self::Output {
        if &self.prime != &other.prime {
            panic!("Cannt add two numbers in different Field.(Prime value is not matched)");
        }

        let mut new_num = BigUint::from(self.get_number().clone());
        if &self.num >=  &other.num {
            new_num = (self.get_number() - other.get_number()) % self.get_prime();
        } else {
            let ne_diff = (other.get_number() - self.get_number()) % self.get_number();
            if ne_diff == BigUint::zero() {
                new_num = BigUint::zero();
            } else {
                new_num = self.get_prime() - ne_diff; 
            }
        }
        FieldElement::new(new_num).unwrap()
    }
}

impl Mul for FieldElement {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let new_num = self.get_number() * rhs.get_number();
        FieldElement::new(new_num).unwrap()
    }
}

// impl Div for FieldElement {
//     type Output = Self;

//     fn div(self, rhs: Self) -> Self::Output {
        
//     }
// }


#[cfg(test)]
mod field_element_tests {
    use super::*;
    use num::Num;

    const BIG1: &str = "79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798";
    const B1PLUS1: &str = "79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81799";
    const BIG2: &str = "483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8";

    #[test]
    fn confirm_secp256k1() {
        let gx = BigUint::from_str_radix(BIG1, 16).unwrap();
        let gy = BigUint::from_str_radix(BIG2, 16).unwrap();
        let p: BigUint = BigUint::from_u8(2).unwrap().pow(256_u32)
            - BigUint::from_u8(2).unwrap().pow(32_u32)
            - BigUint::from_u32(977).unwrap();

        assert_eq!(gy.pow(2_u32) % &p, (gx.pow(3_u32) + 7_u32) % p)
    }

    #[test]
    fn field_element_eq() {
        let a = FieldElement::new(
            BigUint::from_str_radix(BIG1, 16).unwrap()
        ).unwrap();

        let b = FieldElement::new(
            BigUint::from_str_radix(BIG1, 16).unwrap()
        ).unwrap();

        let c = FieldElement::new (
            BigUint::from_str_radix(BIG2, 16).unwrap()
        ).unwrap();

        assert!(a == b);
        assert!(a != c);
    }

    #[test]
    fn field_element_add_sub() {
        let a = FieldElement::new(
            BigUint::from_str_radix(BIG1, 16).unwrap()
        ).unwrap();

        let b = FieldElement::new(
            BigUint::from_str_radix("1", 16).unwrap(),
        ).unwrap();

        let c = FieldElement::new (
            BigUint::from_str_radix(B1PLUS1, 16).unwrap()
        ).unwrap();

        assert!(a.clone() + b.clone() == c.clone());
        assert!(a.clone() - c.clone() == FieldElement::new(b.clone().get_prime() - b.clone().get_number()).unwrap());
    }

    #[test]
    fn field_element_mul_div() {
        let a = FieldElement::new(
            BigUint::from_str_radix(BIG1, 16).unwrap()
        ).unwrap();

        let c = FieldElement::new (
            BigUint::from_str_radix("1", 16).unwrap()
        ).unwrap();

        assert!(a.clone() * c.clone() == a.clone());
    }

    #[test]
    fn pow_test1() {
        let mut a = FieldElement::new(
            BigUint::from_str_radix(BIG1, 16).unwrap()
        ).unwrap();

        let b = a.clone();
        a.to_the_power_of(BigUint::from_i8(1).unwrap());

        assert_eq!(a, b);
    }

    #[test]
    fn pow_test2() {
        let mut a2 = FieldElement::new(
            BigUint::from_i8(2).unwrap()
        ).unwrap();

        let a8 = FieldElement::new(
            BigUint::from_i8(8).unwrap()
        ).unwrap();

        a2.to_the_power_of(BigUint::from_i8(3).unwrap());
        assert_eq!(a2, a8);
    }

}