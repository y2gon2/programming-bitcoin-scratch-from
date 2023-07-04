//! Elliptic Curve Crytography within finite field
//! 타원 곡선 구조체의 field type 을 유한체로 설정
//! 


use std::error::Error;
use std::fmt::Display;
use std::ops::Add;
use crate::secp256k1::field_element::FieldElement;

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
                    if y_val.pow(2) == &(x_val.pow(3) + (&a * x_val)) + &b {
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
        if self.a == other.a && self.b == other.b && self.x == other.x && self.y == other.y {
            return true;
        } else {
            return false;
        }
    }
}

impl Eq for Point {}

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
                    let x = s.pow(2) - (s_x + r_x);
                    let y = &(s * (s_x - &x)) - s_y;
                    
                    return Point::new(Some(x), Some(y), self.a, self.b);
                }
                if self == rhs && s_y == &(&FieldElement::new(0, self.b.get_prime()) * s_x) {
                    return Point::new(None, None, self.a, self.b);
                }
                if self == rhs {
                    let s = (&(FieldElement::new(3, self.b.get_prime()) * s_x.pow(2)) + &self.a) 
                        / (&FieldElement::new(2, self.b.get_prime()) + s_y);
                    let x = &s.pow(2) - &(&FieldElement::new(2, self.b.get_prime()) * s_x);
                    let y = &(s * (s_x -  &x)) - s_y;
    
                    return Point::new(Some(x), Some(y), self.a, self.b);
                }
            };
            return Err("Out of case".into());
        } 
    }
}
