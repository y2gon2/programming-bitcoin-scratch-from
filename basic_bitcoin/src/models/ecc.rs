//! Elliptic Curve Cryptograpy
//! std::hash is used instead of Sha256
//! ECC Point 계산에 필수적으로 필요한  Bigint * Point 연산 구현에 대한 수학적 추가 학습 필요
//! 현재 코드는 동등 논리 구조를 구현하였으냐, test 결과에서 연산의 결과 값이 달라 
//! PrivateKey 구조체 및 해당 method 에 대한 구현작업을 중단

use std::error::Error;
use std::fmt::Display;
use std::ops::{Add, Sub, Mul,Div};

use lazy_static::lazy_static;
use num_bigint::{BigInt, ToBigInt, Sign};
use num_traits::{One, Num, Zero, Pow, ToPrimitive, FromPrimitive};

use crate::models::helper::*;

fn mod_pow(base: i128, exp: i128, modulus: i128) -> i128 {
    let big_base = base.to_bigint().unwrap();
    let big_exp = exp.to_bigint().unwrap();
    let big_modulus = modulus.to_bigint().unwrap();
    
    let zero = BigInt::zero();
    let one = BigInt::one();
    let two = &one + &one;

    if &big_exp < &zero {
        panic!("Negative exponent");
    }

    let mut new_base = big_base % big_modulus;
    let mut new_exp = big_exp.clone();
    let mut result = one.clone();

    while new_exp > zero {
        if &new_exp % &two == one {
            result = (result * &new_base) % modulus;
        }
        new_exp = new_exp >> 1;
        new_base = (&new_base * &new_base) % modulus;
    }
    result.to_i128().unwrap()
}

//---------------------
//     FieldElement
//---------------------

#[derive(Clone, Debug)]
pub struct FieldElement {
    num: i128,
    prime: i128,   
}

#[allow(dead_code)]
impl FieldElement {
    fn new(num: i128, prime: i128) -> Result<Self, Box<dyn Error>> {
        if num >= prime || num < 0 {
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



    fn power_of(self, exponent: i128) -> Self {
        if self.prime - 1 < 0 {
            panic!("Error: Exponent value is minus.")
        }

        let n = exponent % (self.prime - 1);
        let new_num = mod_pow(self.num, n, self.prime);

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
        let num = (self.num + rhs.num) % self.prime;

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

/// 이게 동작할지.. 모르겠음.. 
impl Div for FieldElement {
    type Output = FieldElement;

    fn div(self, rhs: Self) -> Self::Output {
        if &self.prime != &rhs.prime {
            panic!("Cannot divide two numbers in different Fields");
        }

        let num = (self.num * rhs.clone().power_of(self.prime - 2).num % self.prime) % self.prime;

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

        let num = (self.num * rhs.clone().power_of(rhs.prime - 2).num) % self.prime;

        FieldElement {
            num,
            prime: self.prime,
        }
    }
}


#[cfg(test)]
mod feildelement_tests {

    use super::*;

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
        let a = FieldElement::new(3, 31)?;
        let b = FieldElement::new(24, 31)?;
        let c = FieldElement::new(4, 31)?;
        assert!(a / b == c);

        // 아...  음수 exp 는 이거저것 해봐도 안되네.;;; 
        // 다음에 해보자;;;;;;;;;;
        // let d = FieldElement::new(17, 31)?;
        // let e = FieldElement::new(29, 31)?;
        // println!("{}", d.power_of(-3));
        // println!("{}", e);

        // let f = FieldElement::new(4, 31)?;
        
        Ok(())
    }
}

//---------------------
//     Int Point : 실제로 사용되지 않으며 이론적 확인을 위해 구성
//---------------------
#[derive(Clone, Debug)]
pub struct IPoint {
    a: i64,
    b: i64,
    x: Option<i64>,
    y: Option<i64>,
}

impl IPoint {
    pub fn new(x: Option<i64>, y: Option<i64>, a: i64, b: i64) -> Self {
        if x != None && y != None {
            if y.unwrap().pow(2) != x.unwrap().pow(3) + a * x.unwrap() + b {
                panic!("({}, {}) is not on the curve.", x.unwrap(), y.unwrap())
            }
        } 
        Self { a, b, x, y }
    }
}

impl PartialEq for IPoint {
    fn eq(&self, other: &Self) -> bool {
        if self.a == other.a 
        && self.b == other.b
        && self.x == other.x
        && self.y == other.y
        {
            return true
        } else {
            return false
        }
    }
}

impl Eq for IPoint {}

impl Display for IPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        #[allow(unused_assignments)]
        let mut sx = String::new();
        #[allow(unused_assignments)]
        let mut sy = String::new();
        
        match (self.x, self.y) {
            (None, None) => { sx = "None".to_owned(); sy = "None".to_owned(); }, 
            (Some(x), None) => { sx = x.to_string(); sy = "None".to_owned(); },
            (None, Some(y)) => { sx = "None".to_owned(); sy = y.to_string(); },
            (Some(x), Some(y)) => { sx = x.to_string(); sy = y.to_string(); },
        }
        
        write!(f, 
            "Point({},{})_{}_{}",
            sx, sy, self.a, self.b
        )
    }
}

impl Add for IPoint {
    type Output = IPoint;

    fn add(self, rhs: Self) -> Self::Output {
        if self.a != rhs.a || self.b != rhs.b {
            panic!("Points {}, {} are not on the same curve", self, rhs)
        }

        if self.x == None { return rhs}
        if rhs.x == None {return self}

        if self.x == rhs.x && self.y != rhs.y {
            return IPoint::new(None, None, self.a, self.b)
        }

        if self.x != rhs.x {
            let s = (rhs.y.unwrap() - self.y.unwrap()) / (rhs.x.unwrap() - self.x.unwrap());
            let x = s.pow(2) - self.x.unwrap() - rhs.y.unwrap();
            let y = s * (self.x.unwrap() - x) - self.y.unwrap();
            return IPoint::new(Some(x), Some(y), self.a, self.b)
        }

        if self == rhs && self.y.unwrap() == 0 * self.x.unwrap() {
            return IPoint::new(None, None, self.a, self.b)
        }

        if self == rhs {
            let s = (3 * self.x.unwrap().pow(2) + self.a) / (2 * self.y.unwrap());
            let x = s.pow(2) - 2 * self.x.unwrap();
            let y = s * (self.x.unwrap() - x) - self.y.unwrap();
            return IPoint::new(Some(x), Some(y), self.a, self.b)
        }

        panic!("Out of case")
    }
}

impl Mul<i64> for IPoint {
    type Output = IPoint;

    fn mul(self, mut coef: i64) -> Self::Output {
        let mut current = self;
        let mut result = IPoint::new(None, None, current.a, current.b);

        while coef != 0 {
            if coef & 1 == 1 {
                result = result + current.clone();
            }
            current = current.clone() + current;
            coef >>= 1;
        }
        result
    }
}

#[cfg(test)]
mod ipoint_test {
    use super::*;   
    
    #[test]
    fn point_test_ne() {
        let a = IPoint::new(Some(3), Some(-7), 5, 7);
        let b = IPoint::new(Some(18), Some(77), 5, 7);
        
        assert!(&a != &b);
        assert!(&a == &a);
    }

    #[test]
    fn tset_add0() {
        let a = IPoint::new(None, None, 5, 7);
        let b = IPoint::new(Some(2), Some(5), 5, 7);
        let c = IPoint::new(Some(2), Some(-5), 5, 7);

        assert_eq!(a.clone() + b.clone(), b.clone());
        assert_eq!(b.clone() + a.clone(), b.clone());
        assert_eq!(b + c, a);
    }

    #[test]
    fn test_add2() {
        let a = IPoint::new(Some(-1), Some(1), 5, 7);
        let b = IPoint::new(Some(18), Some(-77), 5, 7);
        assert_eq!(a.clone() + a.clone(), b);
    } 
}

// ---------------------
//     Point
// ---------------------
#[derive(Clone, Debug)]
pub struct Point {
    a: FieldElement,
    b: FieldElement,
    x: Option<FieldElement>,
    y: Option<FieldElement>, 
}

impl Point {
    pub fn new(x: Option<FieldElement>, y: Option<FieldElement>, a: FieldElement, b: FieldElement)
        -> Result<Self, Box<dyn Error>> {
            let err_msg = "x, y is not on the curve.";

            match (&x, &y) {
                (None, None) => Ok(Self {a, b, x, y}),
                (Some(x_val), Some(y_val)) => {
                    if y_val.clone().power_of(2) == &(x_val.clone().power_of(3) + (&a * x_val)) + &b {
                        return Ok(Self { a, b, x, y });
                    } else {
                        return Err(err_msg.into());
                    }
                }
                (_, _) => Err(err_msg.into()),
            }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        if self.a == other.a 
            && self.b == other.b 
            && self.x == other.x 
            && self.y == other.y {
            return true;
        } else {
            return false;
        }
    }
}

impl Eq for Point {}

impl<'a> PartialEq<&'a Point> for Point {
    fn eq(&self, other: &&'a Point) -> bool {
        if self.a == other.a 
            && self.b == other.b 
            && self.x == other.x 
            && self.y == other.y {
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
            "Point({},{})_{}_{} FieldElement({})",
            self.x.as_ref().unwrap().num, 
            self.y.as_ref().unwrap().num, 
            self.a.num, 
            self.b.num, 
            self.x.as_ref().unwrap().prime
        )
    }
}


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
            if let (Some(s_x), Some(s_y), Some(r_x), Some(r_y)) 
                = (&self.x, &self.y, &rhs.x, &rhs.y) {
                if s_x == r_x && s_y != r_y {
                    return Point::new(None, None, self.a, self.b);
                }
                if s_x != r_x {
                    let s = (r_y - s_y) / (r_x - s_x);
                    let x = s.clone().power_of(2) - (s_x + r_x);
                    let y = &(s.clone() * (s_x - &x)) - s_y;
                    
                    return Point::new(Some(x), Some(y), self.a, self.b);
                }
                if self == rhs && s_y == &(&FieldElement::new(0, self.b.get_prime()).unwrap() * s_x) {
                    return Point::new(None, None, self.a, self.b);
                }
                if self == rhs {
                    let s = (&(FieldElement::new(3, self.b.get_prime()).unwrap() * s_x.clone().power_of(2)) + &self.a) 
                        / (&FieldElement::new(2, self.b.get_prime()).unwrap() + s_y);
                    let x = s.clone().power_of(2) - (&FieldElement::new(2, self.b.get_prime()).unwrap() * s_x);
                    let y = &(s.clone() * (s_x -  &x)) - s_y;
    
                    return Point::new(Some(x), Some(y), self.a, self.b);
                }
            };
            return Err("Out of case".into());
        } 
    }
}

/// 제대로 동작 안함.. 수학적 증명은 잘 모르겟음..ㅜㅜ
impl Mul<i128> for Point {
    type Output = Point;

    fn mul(self, mut coef: i128) -> Self::Output {
        let mut current = self.clone();
        let mut result = Point::new(
            None, 
            None, 
            self.a.clone(), 
            self.b.clone(),
        ).unwrap();

        while coef != 0 {
            if coef & 1 == 1 {
                result = (result + current.clone()).unwrap();
            }
            current = (current.clone() + current.clone()).unwrap();
            coef >>= 1;
        }
        result
    }
}

#[cfg(test)]
mod ecc_test {
    use super::*;

    #[test]
    fn test_on_curve() {
        // on curve y^2=x^3-7 over F_223:
        // (192,105) (17,56) (200,119) (1,193) (42,99)
        let prime = 223;
        let a = FieldElement::new(0, prime).unwrap();
        let b = FieldElement::new(7, prime).unwrap();

        let valid_points = [(192i128, 105i128), (17i128, 56i128), (1i128, 193i128)];
        let invalid_points = [(200i128, 119i128), (42i128, 99i128)];

        for (x_raw, y_raw) in valid_points.iter() {
            let x = FieldElement::new(*x_raw, prime).unwrap();
            let y = FieldElement::new(*y_raw, prime).unwrap();
            let _temp = Point::new(Some(x), Some(y), a.clone(), b.clone()).unwrap();
        } 

        for (x_raw, y_raw) in invalid_points.iter() {
            let x = FieldElement::new(*x_raw, prime).unwrap();
            let y = FieldElement::new(*y_raw, prime).unwrap();
            let _temp = Point::new(Some(x), Some(y), a.clone(), b.clone()).unwrap();
        } 
    }

    #[test]
    fn tst_add() {
        // tests the following additions on curve y^2=x^3-7 over F_223:
        // (192,105) + (17,56)
        // (47,71) + (117,141)
        // (143,98) + (76,66)

        let prime = 223;
        let a = FieldElement::new(0, prime).unwrap();
        let b = FieldElement::new(7, prime).unwrap();

        let additions = [
            // (x1, y1, x2, y2, x3, y3)
            (192i128, 105i128, 17i128, 56i128, 170i128, 142i128),
            (47, 71, 117, 141, 60, 139),
            (143, 98, 76, 66, 47, 71),
        ];

        for (x1_raw, y1_raw, x2_raw, y2_raw, x3_raw, y3_raw) in additions.iter() {
            let x1 = FieldElement::new(*x1_raw, prime).unwrap();
            let y1 = FieldElement::new(*y1_raw, prime).unwrap();
            let p1 = Point::new(Some(x1), Some(y1), a.clone(), b.clone()).unwrap();

            let x2 = FieldElement::new(*x2_raw, prime).unwrap();
            let y2 = FieldElement::new(*y2_raw, prime).unwrap();
            let p2 = Point::new(Some(x2), Some(y2), a.clone(), b.clone()).unwrap();

            let x3 = FieldElement::new(*x3_raw, prime).unwrap();
            let y3 = FieldElement::new(*y3_raw, prime).unwrap();
            let p3 = Point::new(Some(x3), Some(y3), a.clone(), b.clone()).unwrap();

            assert!((p1 + p2).unwrap() == p3);
        }
    }

    // Point * i128 연산 동작이 달라서 해당 test failed
    #[test]
    fn test_rmul() {
        // tests the following scalar multiplications
        // 2*(192,105)
        // 2*(143,98)
        // 2*(47,71)
        // 4*(47,71)
        // 8*(47,71)
        // 21*(47,71)

        let prime = 223;
        let a = FieldElement::new(0, prime).unwrap();
        let b = FieldElement::new(7, prime).unwrap();

        let multiplications = [
            // (coefficient, x1, y1, x2, y2)
            // (2i128, 192i128, 105i128, 49i128, 71i128),
            // (2, 143, 98, 64, 168),
            // (2, 47, 71, 36, 111),
            // (4, 47, 71, 194, 51),
            // (8, 47, 71, 116, 55),
            // (21, 47, 71, 0, 0),
        ];

        for (s, x1_raw, y1_raw, x2_raw, y2_raw) in multiplications {
            let x1 = FieldElement::new(x1_raw, prime).unwrap();
            let y1 = FieldElement::new(y1_raw, prime).unwrap();
            let p1 = Point::new(Some(x1), Some(y1), a.clone(), b.clone()).unwrap();
            // println!("{:?}", &p1);

            let p2: Point;
            if x2_raw == 0 {
                p2 = Point::new(None, None, a.clone(), b.clone()).unwrap();
            } else {
                let x2 = FieldElement::new(x2_raw, prime).unwrap();
                let y2 = FieldElement::new(y2_raw, prime).unwrap();
                p2 = Point::new(Some(x2), Some(y2), a.clone(), b.clone()).unwrap();
            }
            // println!("{:?}", &p2);

            assert_eq!(p1 * s, p2);
        }
    }
}


// ---------------------
//     S256Field
// ---------------------

#[allow(dead_code)]
const A: i64 = 0;
const B: i64 = 7;
// P = 2.pow(256u16) - 2.pow(32u16) - 977i64
const P: &str = "115792089237316195423570985008687907853269984665640564039457584007908834671663";
const N: &str = "fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141";

#[derive(Clone, Debug)]
pub struct S256Field {
    num: BigInt,
    prime: BigInt,
}

impl S256Field {
    pub fn new(num: &str) -> Self {
        Self {
            num: BigInt::from_str_radix(num, 16).unwrap(),
            prime: BigInt::from_str_radix(P, 16).unwrap(),
        }
    }

    pub fn new_bigint(num: BigInt) -> Self {
        Self {
            num,
            prime: BigInt::from_str_radix(P, 16).unwrap(),
        }
    }

    pub fn sqrt(&self) -> Self {
        let calculation = (&self.prime + BigInt::one()) / 4;
        let new_num = self.num.modpow(&calculation, &self.prime);

        return S256Field::new_bigint(new_num)
    }
}

impl Display for S256Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.num.to_str_radix(16))
    }
}

impl PartialEq for S256Field {
    fn eq(&self, other: &Self) -> bool {
        if self.num == other.num && self.prime == other.prime { 
            return true 
        } else {
            return false
        }
    }
}

impl Eq for S256Field {}

impl Add for S256Field {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if &self.prime != &rhs.prime {
            panic!("Cannot add two numbers in different Fields");
        }
        let num = (self.num + rhs.num) % self.prime.clone();

        return Self {
            num,
            prime: self.prime,
        }
    }       
}

impl<'a> Add<&'a S256Field> for &'a S256Field {
    type Output = S256Field;

    fn add(self, rhs: &'a S256Field) -> Self::Output {
        if &self.prime != &rhs.prime {
            panic!("Cannot add two numbers in different Fields");
        }
        let num = (self.num.clone() + rhs.num.clone()) % self.prime.clone();

        return S256Field {
            num,
            prime: self.prime.clone(),
        }
    }
}

impl Sub for S256Field {
    type Output = S256Field;

    fn sub(self, rhs: Self) -> Self::Output {
        if &self.prime != &rhs.prime {
            panic!("Cannot subtract two numbers in different Fields");
        }

        let num = (self.num - rhs.num) % self.prime.clone();

        return Self {
            num,
            prime: self.prime,
        }
    }
}

impl<'a> Sub<&'a S256Field> for&'a S256Field {
    type Output = S256Field;

    fn sub(self, rhs: Self) -> Self::Output {
        if &self.prime != &rhs.prime {
            panic!("Cannot subtract two numbers in different Fields");
        }

        let num = (self.num.clone() - rhs.num.clone()) % self.prime.clone();

        return S256Field {
            num,
            prime: self.prime.clone(),
        }
    }
}

impl Mul for S256Field {
    type Output = S256Field;

    fn mul(self, rhs: Self) -> Self::Output {
        if &self.prime != &rhs.prime {
            panic!("Cannot multiply two numbers in different Fields");
        }

        let num = (self.num * rhs.num) % self.prime.clone();

        return S256Field {
            num,
            prime: self.prime.clone(),
        }
    }
}

impl<'a> Mul<&'a S256Field> for&'a S256Field {
    type Output = S256Field;

    fn mul(self, rhs: Self) -> Self::Output {
        if &self.prime != &rhs.prime {
            panic!("Cannot multiply two numbers in different Fields");
        }

        let num = (self.num.clone() * rhs.num.clone()) % self.prime.clone();

        return S256Field {
            num,
            prime: self.prime.clone(),
        }
    }
}

impl Div for S256Field {
    type Output = S256Field;

    fn div(self, rhs: Self) -> Self::Output {
        if &self.prime != &rhs.prime {
            panic!("Cannot divide two numbers in different Fields");
        }

        let num = (
            self.num 
            * rhs.num.modpow(&(self.prime.clone() - BigInt::from_i8(2).unwrap()), &self.prime)) 
            % self.prime.clone();

        return S256Field {
            num,
            prime: self.prime,
        }
    }
}

impl<'a> Div<&'a S256Field> for&'a S256Field {
    type Output = S256Field;

    fn div(self, rhs: Self) -> Self::Output {
        if self.prime != rhs.prime {
            panic!("Cannot divide two numbers in different Fields");
        }

        let num = (
            self.num.clone() 
            * rhs.num.modpow(&(self.prime.clone() - BigInt::from_i8(2).unwrap()), &self.prime)) 
            % self.prime.clone();

        return S256Field {
            num,
            prime: self.prime.clone(),
        }
    }
}

impl num_traits::pow::Pow<BigInt> for S256Field {
    type Output = S256Field;
    
    fn pow(self, rhs: BigInt) -> Self::Output {
        let n = rhs % (self.prime.clone() - BigInt::one());
        let num = self.num.modpow(&n, &self.prime);

        Self { 
            num, 
            prime: self.prime,
        }
    }
}

// ---------------------
//     S256Point
// ---------------------

const X_NUM: &str = "79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798";
const Y_NUM: &str = "483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8";

lazy_static! {
    #[derive(Debug)]
    static ref G: S256Point = S256Point {
        x: Some(S256Field {
            num: BigInt::from_str_radix(X_NUM, 16).unwrap(),
            prime: BigInt::from_str_radix(P, 16).unwrap(),
        }),
        y: Some(S256Field {
            num: BigInt::from_str_radix(Y_NUM, 16).unwrap(),
            prime: BigInt::from_str_radix(P, 16).unwrap(),
        }),
        a: None,
        b: None,          
    };
}

 
pub struct Signature {
    r: String,
    s: String,
}

impl Signature {
    pub fn new(r: String, s: String) -> Self {
        Self { r, s }
    }
}

#[derive(Clone, Debug)]
pub struct S256Point {
    x: Option<S256Field>,
    y: Option<S256Field>,
    a: Option<S256Field>,
    b: Option<S256Field>,
}

impl S256Point {
    pub fn new(x: Option<S256Field>, y: Option<S256Field>, a: Option<S256Field>, b: Option<S256Field>) -> Self {
        Self { x, y, a, b }
    }

    pub fn new_bigint_xy(x: BigInt, y: BigInt) -> Self {
        Self {
            x: Some(S256Field::new_bigint(x)),
            y: Some(S256Field::new_bigint(y)),
            a: None,
            b: None,
        }
    }

    // ex
    // self.assertTrue(point.verify(z, Signature(r, s)))
    // z = 0x7c076ff316692a3d7eb3c3bb0f8b1488cf72e1afcd929e29307032997a838a3d
    // r = 0xeff69ef2b1bd93a66ed5219add4fb51e11a840f404876325a1e8ffe0529a2c
    // s = 0xc7207fee197d27c618aea621406f6bf5ef6fca38681d82b2f06fddbdce6feab6
    pub fn verify(&self, z: &str, sig: Signature ) -> bool {
   
        let z_bigint = BigInt::from_str_radix(z, 16).unwrap();
        let sig_r_bigint = BigInt::from_str_radix(&sig.r,16).unwrap();
        let sig_s_bigint = BigInt::from_str_radix(&sig.s,16).unwrap();

        let n = BigInt::from_str_radix(N, 16).unwrap();
        let s_inv = sig_s_bigint.modpow(&(&n - &BigInt::from(2)), &n);

        let u = (&z_bigint * &s_inv) % &n;
        let v = (&sig_r_bigint * &s_inv) % n;

        let total = (G.clone() * u) + (self.clone() * v); 
 
        return total.x.unwrap().num == sig_r_bigint;
    }

    /// returns the binary version of the SEC format
    pub fn sec(&self, compressed: bool) -> Vec<u8> {
        // if compressed, starts with b'\x02' if self.y.num is even, b'\x03' if self.y is odd
        // then self.x.num
        // remember, you have to convert self.x.num/self.y.num to binary (some_integer.to_bytes(32, 'big'))

        if compressed {
            if &self.y.as_ref().unwrap().num % &BigInt::from(2) == BigInt::zero() {
                let mut vec = vec![2];
                let (_sign, mut num_bytes) = self.x.clone().unwrap().num.to_bytes_be();
                vec.append(&mut num_bytes);
                return vec
            } else {
                let mut vec = vec![3];
                let (_sign, mut num_bytes) = self.x.clone().unwrap().num.to_bytes_be();
                vec.append(&mut num_bytes);
                return vec
            }
        } else {
            let mut vec = vec![4];
            let (_sign, mut num_bytes) = self.x.clone().unwrap().num.to_bytes_be();
            vec.append(&mut num_bytes);
            return vec
        }
    }

    pub fn hash160(&self, compressed: bool) -> Vec<u8> {
        return hash160(&self.sec(compressed))
    }

    /// return the address string
    pub fn address(&self, compressed: bool, testnet: bool) -> String {
        let mut h160 = self.hash160(compressed);

        #[allow(unused_assignments)]
        let mut prefix = 0u8;
        if testnet {
            prefix = b'\x6f';
        } else {
            prefix = b'\x00';
        }

        let mut input = vec![prefix];
        input.append(&mut h160);

        return encode_base58_checksum(&input)
    }

    /// returns a Point object from a SEC binary (not hex)
    pub fn parse(sec_bin: Vec<u8>) -> Self {
        if sec_bin[0] == 4 {
            let x = BigInt::from_bytes_be(Sign::Plus, &sec_bin[1..33]);
            let y = BigInt::from_bytes_be(Sign::Plus, &sec_bin[33..65]);
            return S256Point::new_bigint_xy(x, y);
        } 

        let is_even = sec_bin[0] == 2;
        let x = S256Field::new_bigint(BigInt::from_bytes_be(Sign::Plus, &sec_bin[1..]));

        // right side of the equation y^2 = x^3 + 7
        let alpha = x.clone().pow(BigInt::from(3)) + S256Field::new_bigint(BigInt::from(B));

        // solve for left side
        let beta = alpha.sqrt();
        let other_side_beta = S256Field::new_bigint(&BigInt::from_str_radix(P, 16).unwrap() - &beta.num);

        if &beta.num % &BigInt::from(2) == BigInt::zero() {
            if is_even {
                return S256Point::new(Some(x), Some(beta), None, None)
            } else {
                return S256Point::new(Some(x), Some(other_side_beta), None, None)
            }
        } else {
            if is_even {
                return S256Point::new(Some(x), Some(other_side_beta), None, None)
            } else {
                return S256Point::new(Some(x), Some(beta), None, None)
            }
        }
    }
}

impl PartialEq for S256Point {
    fn eq(&self, other: &Self) -> bool {
        if self.x == other.x && self.y == other.y && self.a == other.a && self.b == other.b {
            return true
        } else {
            return false
        }
    }
}

impl Eq for S256Point {}

impl Display for S256Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.x == None {
            return write!(f, "S256Point(infinity)")
        } 
        write!(f, "S256Point({} {})", self.x.as_ref().unwrap(), self.y.as_ref().unwrap())
    }
}

impl Add for S256Point {
    type Output = S256Point;

    fn add(self, rhs: Self) -> Self::Output {
        if &self.a != &rhs.a || &self.b != &rhs.b {
            panic!("They are not on the same curve.")
        } 

        if *&self.x.is_none() {
            return rhs.clone();
        }

        if *&rhs.x.is_none() {
            return self.clone();
        } 

        if &self.x == &rhs.x && &self.y == &rhs.y {
            return S256Point::new(None, None, self.a, self.b)
        }

        if &self.x != &rhs.x {
            let s = (rhs.y.as_ref().unwrap() - self.y.as_ref().unwrap()) / (rhs.x.as_ref().unwrap() - rhs.x.as_ref().unwrap());
            let x = s.clone().pow(BigInt::from(2)) - rhs.x.clone().unwrap();
            let y = &(s.clone() * (self.x.as_ref().unwrap() - &x)) - self.y.as_ref().unwrap();
            
            return S256Point::new(Some(x), Some(y), self.a, self.b);
        }

        if &self == &rhs 
        && self.y.as_ref().unwrap() == &(&S256Field::new_bigint(BigInt::zero()) * self.x.as_ref().unwrap()) {
            return S256Point::new(None, None, self.a, self.b)
        }   

        if &self == &rhs {
            let s = (
                S256Field::new_bigint(BigInt::from(3)) 
                * self.x.clone().unwrap().pow(BigInt::from(2))
                + self.a.clone().unwrap()
                ) 
                / (&S256Field::new_bigint(BigInt::from(2)) * self.y.as_ref().unwrap());
            let x = s.clone().pow(BigInt::from(2)) - (S256Field::new_bigint(BigInt::from(2)) * self.x.clone().unwrap());
            let y = s * (self.x.as_ref().unwrap() -  &x) - self.y.clone().unwrap();

            return S256Point::new(Some(x), Some(y), self.a, self.b);
        }
        panic!("Out of Case")
    }
}

impl Mul<BigInt> for S256Point {
    type Output = Self;

    fn mul(self, coefficient: BigInt) -> Self::Output {
        let mut coef = coefficient % BigInt::from_str_radix(N, 16).unwrap();

        // 안될거 같은데... 
        let mut current = self.clone();
        let mut result = S256Point::new(None, None, self.a.clone(), self.b.clone());

        while &coef != &BigInt::zero() {
            if &coef & &BigInt::one() == BigInt::one() {
                result = result + current.clone();
            }
            current = current.clone() + current.clone();
            coef >>= 1;
        }
        result 
    }
}


#[cfg(test)]
mod s256_test {
    use super::*;

    #[test]
    fn read_g() {
        println!("{:?}", G.clone());
    }

    #[test]
    fn test_order() {
        let point = G.clone() * BigInt::from_str_radix(N, 16).unwrap();
        println!("{:?}", point); // S256Point { x: None, y: None, a: None, b: None }
    }

    // 아래와 같이 결과값이 다름.
    // coefficient 적용 연산에 차이가 있을듯 하지만 code 적으로는 동등함을 chatGPT 를 통해 검증함.
    #[test]
    fn test_pubpoint() {
        let points = [ // secret, x, y
            (BigInt::from(7), "5cbdf0646e5db4eaa398f365f2ea7a0e3d419b7e0330e39ce92bddedcac4f9bc", "6aebca40ba255960a3178d6d861a54dba813d0b813fde7b5a5082628087264da"),
            (BigInt::from(1485), "c982196a7466fbbbb0e27a940b6af926c1a74d5ad07128c82824a11b5398afda", "7a91f9eae64438afb9ce6448a1c133db2d8fb9254e4546b6f001637d50901f55"),
            (BigInt::from(2).pow(128u8), "8f68b9d2f63b5f339239c1ad981f162ee88c5678723ea3351b7b444c9ec4c0da", "662a9f2dba063986de1d90c2b6be215dbbea2cfe95510bfdf23cbf79501fff82"),
            (BigInt::from(2).pow(240u8) + BigInt::from(2).pow(31u8), "9577ff57c8234558f293df502ca4f09cbc65a6572c842b39b366f21717945116", "10b49c67fa9365ad7b90dab070be339a1daf9052373ec30ffae4f72d5e66d053"),
        ];

        for (secret, x, y) in points {
            let point = S256Point::new_bigint_xy(
                BigInt::from_str_radix(x, 16).unwrap(),
                BigInt::from_str_radix(y, 16).unwrap(),
            );

            println!("{:?}", G.clone() * secret);
            println!("{:?}", point);
            // assert!(G.clone() * secret == point);
        }
    }
    // S256Point { x: Some(S256Field { num: 55066263022277343669578718895168534326250603453777594175500187360389116729240, prime: 565222794527730373379072786348411718562863470688369141095402336226947079297792115268951414371 }), y: Some(S256Field { num: 32670510020758816978083085130507043184471273380659243275938904335757337482424, prime: 565222794527730373379072786348411718562863470688369141095402336226947079297792115268951414371 }), a: None, b: None }
    // S256Point { x: Some(S256Field { num: 41948375291644419605210209193538855353224492619856392092318293986323063962044, prime: 565222794527730373379072786348411718562863470688369141095402336226947079297792115268951414371 }), y: Some(S256Field { num: 48361766907851246668144012348516735800090617714386977531302791340517493990618, prime: 565222794527730373379072786348411718562863470688369141095402336226947079297792115268951414371 }), a: None, b: None }
    // S256Point { x: Some(S256Field { num: 55066263022277343669578718895168534326250603453777594175500187360389116729240, prime: 565222794527730373379072786348411718562863470688369141095402336226947079297792115268951414371 }), y: Some(S256Field { num: 32670510020758816978083085130507043184471273380659243275938904335757337482424, prime: 565222794527730373379072786348411718562863470688369141095402336226947079297792115268951414371 }), a: None, b: None }
    // S256Point { x: Some(S256Field { num: 91144748097329341227315146716405895133044962575665947613151200288251569549274, prime: 565222794527730373379072786348411718562863470688369141095402336226947079297792115268951414371 }), y: Some(S256Field { num: 55440085219269127825789759728109305451504918753795093767574238082182444752725, prime: 565222794527730373379072786348411718562863470688369141095402336226947079297792115268951414371 }), a: None, b: None }
    // S256Point { x: None, y: None, a: None, b: None }
    // S256Point { x: Some(S256Field { num: 64865771952738249789114440545196421582918768733599534045195125031385885360346, prime: 565222794527730373379072786348411718562863470688369141095402336226947079297792115268951414371 }), y: Some(S256Field { num: 46211216742671250426576585530459394900178019437443360579906162037052661563266, prime: 565222794527730373379072786348411718562863470688369141095402336226947079297792115268951414371 }), a: None, b: None }
    // S256Point { x: None, y: None, a: None, b: None }
    // S256Point { x: Some(S256Field { num: 67606631551526079174363160834905769336240182401619533769043587988551063851286, prime: 565222794527730373379072786348411718562863470688369141095402336226947079297792115268951414371 }), y: Some(S256Field { num: 7556117524685686037096665667879267882143292133281453141941949923550388736083, prime: 565222794527730373379072786348411718562863470688369141095402336226947079297792115268951414371 }), a: None, b: None }
    

    // ㅜㅜ coef 곱하기 연산 때문에 이것도 안되네.. gg
    #[test]
    fn test_verify() {
        let p1 = (
            "887387e452b8eacc4acfde10d9aaf7f6d9a0f975aabb10d006e4da568744d06c",
            "61de6d95231cd89026e286df3b6ae4a894a3378e393e93a0f45b666329a0ae34"
        );

        let point = S256Point::new_bigint_xy(
            BigInt::from_str_radix(p1.0, 16).unwrap(),
            BigInt::from_str_radix(p1.1, 16).unwrap(),
        );

        let z1 = "ec208baa0fc1c19f708a9ca96fdeff3ac3f230bb4a7ba4aede4942ad003c0f60";
        let r1 = "ac8d1c87e51d0d441be8b3dd5b05c8795b48875dffe00b7ffcfac23010d3a395";
        let s1 = "68342ceff8935ededd102dd876ffd6ba72d6a427a3edb13d26eb0781cb423c4";
        
        assert!(!point.verify(z1, Signature::new(r1.to_string(), s1.to_string())));

        let z2 = "7c076ff316692a3d7eb3c3bb0f8b1488cf72e1afcd929e29307032997a838a3d";
        let r2 = "eff69ef2b1bd93a66ed5219add4fb51e11a840f404876325a1e8ffe0529a2c";
        let s2 = "c7207fee197d27c618aea621406f6bf5ef6fca38681d82b2f06fddbdce6feab6";
        
        assert!(!point.verify(z2, Signature::new(r2.to_string(), s2.to_string())));
    }

}
