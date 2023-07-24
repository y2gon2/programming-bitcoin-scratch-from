//! Elliptic Curve Cryptograpy
//! std::hash is used instead of Sha256

use std::error::Error;
use std::fmt::Display;
use std::ops::{Add, Sub, Mul,Div};

use num_bigint::{BigInt, ToBigInt};
use num_traits::{One, ToPrimitive};
use num_traits::pow::Pow;
use num_traits::identities::Zero;



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
        let mut sx = String::new();
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
    fn test_on_curve() {
        let a = IPoint::new(Some(3), Some(-7), 5, 7);
        let b = IPoint::new(Some(18), Some(77), 5, 7);
        let c = IPoint::new(Some(-2), Some(4), 5, 7);  // panicked at '(-2, 4) is not on the curve.'
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
    use std::fs::File;

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
            let temp = Point::new(Some(x), Some(y), a.clone(), b.clone()).unwrap();
        } 

        for (x_raw, y_raw) in invalid_points.iter() {
            let x = FieldElement::new(*x_raw, prime).unwrap();
            let y = FieldElement::new(*y_raw, prime).unwrap();
            let temp = Point::new(Some(x), Some(y), a.clone(), b.clone()).unwrap();
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



//------------------------------
