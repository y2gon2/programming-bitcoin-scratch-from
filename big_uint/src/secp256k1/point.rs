//! Elliptic Curve Cryptography
//! 타원 곡선 위 유한체를 적용한 타원곡선 암호 구현
//! 
//! Holding Point
//! 해당 타원 곡선 구조체 연산 (+) 을 구현하기 위해서 해당 filed type 인 FieldElement 의
//! 나눗셈이 구현되어야 하는데 나눗셈 구현에 expoenent 로 prime (BigUint) 가 사용되어야 하는데
//! 해당 구현 문제로 작업 중단


use std::error::Error;
use std::ops::{Add};
use std::fmt::Display;
use num::{BigUint, FromPrimitive};
use crate::secp256k1::field_element::FieldElement;

#[derive(Clone, Debug)]
struct Point {
    a: FieldElement,
    b: FieldElement,
    x: Option<FieldElement>,
    y: Option<FieldElement>,
}

impl Point {
    #[allow(dead_code)]
    fn new(a: FieldElement, b: FieldElement, x: Option<FieldElement>, y: Option<FieldElement>) 
        -> Result<Self, Box<dyn Error>> {
            let err_msg = "x, y is not on the curve.";

            match (&x, &y) {
                (None, None) => Ok(Self { a, b, x, y }),
                (Some(x_value), Some(y_value)) => {
                    if y_value.to_the_power_of(BigUint::from_i8(2).unwrap()) == 
                    (&(x_value.to_the_power_of(BigUint::from_i8(3).unwrap()) + (&a * x_value)?)? + &b)? {
                        return Ok(Self { a, b, x, y });
                    } else {
                        return Err(err_msg.into());
                    }
                },
                (_, _ ) => Err(err_msg.into()),
            }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        if self.a == other.a && self.b == other.b && self.x == other.x && self.y == other.y {
            return true;
        } else {
            return false;
        }
    }
}

impl<'a> PartialEq<&'a Point> for Point {
    fn eq(&self, other: &&'a Point) -> bool {
        if self.a == other.a && self.b == other.b && self.x == other.x && self.y == other.y {
            return true;
        } else {
            return false;
        }
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Point: {{\n\t x:{:?}\n\t y:{:?}\n\t a:{:?}\n\t b:{:?}\n }}",
            self.x, self.y, self.a, self.b
        )
    }
}

// FieldElement 나누기 연산에 대한 trait 선언에 에러는 없으나 expoent 에 너무 큰수가 들어가서
// 실제로 연산 시 너무 오래걸려서 제대로 실행되는지 확인 불가. 
// 따라서 우선 해당 코드가 작동한다는 가정하에 이후 code 내용 작성 
impl Add for Point {
    type Output = Result<Point, Box<dyn Error>>;

    fn add(self, rhs: Self) -> Self::Output {
        if &self.a != &rhs.a || &self.b != &rhs.b {
            panic!("They are not on the same curve.")
        } else if self.x.is_none() || self.y.is_none() {
            return Ok(rhs.clone());
        } else if rhs.x.is_none() || rhs.y.is_none() {
            return Ok(self.clone());
        } else {
            if let (Some(s_x), Some(s_y), Some(r_x), Some(r_y)) = (&self.x, &self.y, &rhs.x, &rhs.y) {
                if s_x == r_x && s_y != r_y {
                    return Point::new(self.a, self.b, None, None);
                }
                if s_x != r_x {
                    let s = ((r_y - s_y)? / (r_x - s_x)?)?;  //  나누셈이 제대로 작동하는지 알수 없음..;;
                    let x = (&(&s.to_the_power_of(BigUint::from_u8(2).unwrap()) - s_x)? - r_x)?;
                    let y = (&(s * (s_x - &x)?)? - s_y)?;
                    
                    return Point::new(self.a, self.b, Some(x), Some(y));
                }
                if self == rhs && s_y == &(&FieldElement::new(BigUint::from_u8(0).unwrap())? * s_x)? {
                    return Point::new(self.a, self.b, None, None);
                }
                if self == rhs {
                    let s = 
                        ((&(&FieldElement::new(BigUint::from_u8(3).unwrap())? * &(s_x.to_the_power_of(BigUint::from_u8(2).unwrap()))).unwrap() + &self.a)? 
                        / (&FieldElement::new(BigUint::from_u8(2).unwrap())? + s_y).unwrap())?;
                    let x = (s.to_the_power_of(BigUint::from_u8(2).unwrap()) - (&FieldElement::new(BigUint::from_u8(2).unwrap())? * s_x)?)?;
                    let y = (&(s * (s_x -  &x)?)? - s_y)?;
    
                    return Point::new(self.a, self.b, Some(x), Some(y));
                }
            };
            return Err("Out of case".into());
        } 
    }
}

#[cfg(test)]
mod secp256k1point_test {
    use super::*;
    use num::BigUint;

    type FE = FieldElement;
    type PNT = Point;

    // const BIG1: &str = "79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798";
    // const B1PLUS1: &str = "79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81799";
    // const BIG2: &str = "483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8";

    #[test]
    fn new_test() -> Result<(), Box<dyn Error>>{
        let _a = PNT::new(
            FE::new(BigUint::from_u8(1).unwrap())?,
            FE::new(BigUint::from_u8(1).unwrap())?,
            None,
            None,
        )?;
        
        // let b = PNT::new(
        //     FE::new(BigUint::from_u8(1).unwrap())?,
        //     FE::new(BigUint::from_u8(1).unwrap())?,
        //     Some(FE::new(BigUint::from_u8(1).unwrap())?),
        //     None,
        // )?; //Error 발생

        let _on_elliptic = PNT::new(
            FE::new(BigUint::from_u8(1).unwrap())?,
            FE::new(BigUint::from_u8(2).unwrap())?,
            Some(FE::new(BigUint::from_u8(1).unwrap())?),
            Some(FE::new(BigUint::from_u8(2).unwrap())?),
        )?;
        Ok(())
    }

    #[test]
    fn ep_test() -> Result<(), Box<dyn Error>> {
        let a = PNT::new(
            FE::new(BigUint::from_u8(1).unwrap())?,
            FE::new(BigUint::from_u8(2).unwrap())?,
            Some(FE::new(BigUint::from_u8(1).unwrap())?),
            Some(FE::new(BigUint::from_u8(2).unwrap())?),
        )?;
        let b = PNT::new(
            FE::new(BigUint::from_u8(1).unwrap())?,
            FE::new(BigUint::from_u8(2).unwrap())?,
            Some(FE::new(BigUint::from_u8(1).unwrap())?),
            Some(FE::new(BigUint::from_u8(2).unwrap())?),
        )?;
        let c = PNT::new(
            FE::new(BigUint::from_u8(1).unwrap())?,
            FE::new(BigUint::from_u8(7).unwrap())?,
            Some(FE::new(BigUint::from_u8(1).unwrap())?),
            Some(FE::new(BigUint::from_u8(3).unwrap())?),
        )?;

        assert!(a == b);
        assert!(a != c);

        let a_ref = &a;
        let b_ref = &b;
        let c_ref = &c;

        assert!(a_ref == b_ref);
        assert!(a_ref != c_ref);
        Ok(())
    }

    #[test]
    fn anything() {
        println!("{}", -2 % 10);
    }
}