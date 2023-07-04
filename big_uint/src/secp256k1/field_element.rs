//! Finite Field 정의 및 해다 구현체의 연산 정의
//! 
//! 해당 struct field 는 추후 SHA256 Hash value 를 포함할 수 있도록 BigUint type 으로 설
//! prime field 값은 책에 명시된 2^256 - 2^32 - 977 로 고정
//! 
//! BigUint type 을 기본으로 아래와 같이 Finite Field 연산 구현  
//! 현재 구현된 연산    : ==, != , +, -, *
//! 미구현된 연산       :/ , %
//! 거듭 제곱의 경우 exponent - BitUint type 으로 구현
//! 
//! Holding Point 
//! 유한체 FieldElement 나눗셈을 구현하기 위해서 prime 값을 거듭제곱 지수로 사용해야하는데
//! 현재 expoenent 로 BigUint 적용이 어려움  -> 임시 해당 값을 u32 로 변환하여  Div 구현


use std::error::Error;
use std::fmt::{Display};
use std::ops::{Add, Sub, Mul, Div, Rem};

use num::{BigUint, Zero, One, FromPrimitive};

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

impl<'a> PartialEq<&'a FieldElement> for FieldElement {
    fn eq(&self, other: &&'a Self) -> bool {
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
    type Output = Result<FieldElement, Box<dyn Error>>;

    fn add(self, other: Self) -> Self::Output {
        if self.prime != other.prime {
            return Err("Cannt add two numbers in different Field.".into());
        }
        let new_num = (self.num + other.num) % self.prime;
        Ok(FieldElement::new(new_num)?)
    }
}

impl<'a> Add<&'a FieldElement> for &'a FieldElement {
    type Output = Result<FieldElement, Box<dyn Error>>;

    fn add(self, rhs: &'a FieldElement) -> Self::Output {
        if self.prime != rhs.prime {
            return Err("Cannt add two numbers in different Field.".into());
        }
        let new_num = (&self.num + &rhs.num) % &self.prime;
        Ok(FieldElement::new(new_num)?)
    }
} 

impl Sub for FieldElement {
    type Output = Result<FieldElement, Box<dyn Error>>;

    fn sub(self, rhs: Self) -> Self::Output {
        if &self.prime != &rhs.prime {
            return Err("Cannt add two numbers in different Field.".into());
        }

        let mut new_num = BigUint::zero();
        if &self.num >=  &rhs.num {
            new_num = (self.get_number() - rhs.get_number()) % self.get_prime();
        } else {
            let ne_diff = (rhs.get_number() - self.get_number()) % self.get_number();
            if ne_diff != BigUint::zero() {
                new_num = self.get_prime() - ne_diff;
            }
        }
        Ok(FieldElement::new(new_num)?)
    }
}

impl<'a> Sub<&'a FieldElement> for &'a FieldElement {
    type Output = Result<FieldElement, Box<dyn Error>>;

    fn sub(self, rhs: &'a FieldElement) -> Self::Output {
        if &self.prime != &rhs.prime {
            return Err("Cannt add two numbers in different Field.".into());
        }
        
        let mut new_num = BigUint::zero();
        if &self.num >= &rhs.num {
            new_num = (self.get_number() - rhs.get_number()) % self.get_prime();
        } else {
            let ne_diff = (rhs.get_number() - self.get_number()) % self.get_prime();
            if ne_diff != BigUint::zero() {
                new_num = self.get_prime() - ne_diff;
            }
        }
        Ok(FieldElement::new(new_num)?)
    }
}

impl Mul for FieldElement {
    type Output = Result<FieldElement, Box<dyn Error>>;

    fn mul(self, rhs: Self) -> Self::Output {
        let new_num = self.get_number() * rhs.get_number();
        Ok(FieldElement::new(new_num)?)
    }
}

impl<'a> Mul<&'a FieldElement> for &'a FieldElement {
    type Output = Result<FieldElement, Box<dyn Error>>;

    fn mul(self, rhs: &'a FieldElement) -> Self::Output {
        let new_num = self.get_number() * rhs.get_number();
        Ok(FieldElement::new(new_num)?)
    }
}

impl Div for FieldElement {
    type Output = Result<FieldElement, Box<dyn Error>>;

    fn div(self, rhs: Self) -> Self::Output {
        if &self.get_prime() != &rhs.get_prime() {
            return Err("Cannot divide two numbers in different Fields.".into());
        }

        let prime_u32_vec = self.get_prime().iter_u32_digits().collect::<Vec<u32>>();
        let new_num = 
            &self.num * rhs.get_number().pow(prime_u32_vec[0] - 2) % self.get_prime();
        
        return FieldElement::new(new_num);
    }
}

// source : https://github.com/garyray-k/programming_bitcoin_in_rust/blob/master/src/secp256k1/field_element.rs
// 에러는 없는데 계산시간이 너무 오래걸리는 거 같음... ;; 
// 실제로 사용 가능할지 모르겠음..
impl<'a> Div<&'a FieldElement> for &'a FieldElement {
    type Output = Result<FieldElement, Box<dyn Error>>;

    fn div(self, rhs: &'a FieldElement) -> Self::Output {
        if &self.get_prime() != &rhs.get_prime() {
            return Err("Cannot divide two numbers in different Fields.".into());
        }

        let prime_u32_vec = self.get_prime().iter_u32_digits().collect::<Vec<u32>>();
        let new_num = 
            &self.num * rhs.num.modpow(
                &(self.get_prime().clone() - BigUint::from_u32(2u32).unwrap()), 
                &self.prime,
            );
        let answer = new_num.rem(self.get_prime());

        return FieldElement::new(answer);
    }
}




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
        assert!(&a == &b);
        assert!(&a != &c);
    }

    #[test]
    fn field_element_add_sub() -> Result<(), Box<dyn Error>>{
        let a = FieldElement::new(
            BigUint::from_str_radix(BIG1, 16).unwrap()
        ).unwrap();

        let b = FieldElement::new(
            BigUint::from_str_radix("1", 16).unwrap(),
        ).unwrap();

        let c = FieldElement::new (
            BigUint::from_str_radix(B1PLUS1, 16).unwrap()
        ).unwrap();

        assert!((a.clone() + b.clone())? == c.clone());
        assert!((a.clone() - c.clone())? == (FieldElement::new(b.clone().get_prime() - b.clone().get_number())?));
        
        let ref_a = &a;
        let ref_b = &b;
        let ref_c = &c;
        let added = (ref_a + ref_b)?;
        let subtract = (ref_a - ref_c)?;
        assert!(added == c);
        assert!(subtract == (FieldElement::new(b.clone().get_prime() - b.clone().get_number()))?);
        Ok(())
    }

    #[test]
    fn field_element_mul_div() -> Result<(), Box<dyn Error>>{
        let a = FieldElement::new(
            BigUint::from_str_radix(BIG1, 16).unwrap()
        ).unwrap();

        let c = FieldElement::new (
            BigUint::from_str_radix("1", 16).unwrap()
        ).unwrap();

        assert!((a.clone() * c.clone())? == a.clone());
        assert!(&(&a * &c)? == &a );
        Ok(())
    }

    #[test]
    fn pow_test1() {
        let a = FieldElement::new(
            BigUint::from_str_radix(BIG1, 16).unwrap()
        ).unwrap();

        let b = a.clone();
        a.to_the_power_of(BigUint::from_i8(1).unwrap());

        assert_eq!(a, b);
    }

    #[test]
    fn pow_test2() {
        let a2 = FieldElement::new(
            BigUint::from_i8(2).unwrap()
        ).unwrap();

        let a8 = FieldElement::new(
            BigUint::from_i8(8).unwrap()
        ).unwrap();

        let a2_pow = a2.to_the_power_of(BigUint::from_i8(3).unwrap());

        assert_eq!(a2_pow, a8);
    }


    // 실제로 나눗셈이 되는지 모르겠음..
    // 오류가 발생하지는 않는데 계산시간이 너무 오래 걸리는듯...
    #[test]
    fn div_rem_test() -> Result<(), Box<dyn Error>>{
        let a6 = FieldElement::new(BigUint::from_i8(6).unwrap())?;
        let a5 = FieldElement::new(BigUint::from_i8(5).unwrap())?;
        let a3 = FieldElement::new(BigUint::from_i8(3).unwrap())?;
        let a2 = FieldElement::new(BigUint::from_i8(2).unwrap())?;

        assert!((a6.clone() / a3.clone())? == a2.clone());

        Ok(())
    }

    #[test]
    fn xxx() {
        #[derive(Debug)]
        struct MyStruct {
            value: i32,
        }

        let a = MyStruct { value: 5 };
        let b = MyStruct { value: 10 };
        let ref_a = &a;
        let ref_b = &b;
        let result = ref_a.value + ref_b.value;
        println!("Result: {}", result);
        println!("a: {:?}", a); // 이후에도 여전히 a와 b를 사용할 수 있음
        println!("b: {:?}", b);
    }

}