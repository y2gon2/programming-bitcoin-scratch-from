//! Elliptic Curve Cryptography
//! 타원 곡선 위 유한체를 적용한 타원곡선 암호 구현
//! 


use std::error::Error;
use std::ops::{Add, Sub, Mul, Rem, Div};
use std::fmt::Display;


#[derive(Clone, Debug)]
struct Point {
    a: i32,
    b: i32,
    x: Option<i32>,
    y: Option<i32>,
}

impl Point {
    fn new(x: Option<i32>, y: Option<i32>, a: i32, b: i32) 
        -> Result<Self, Box<dyn Error>> {
            let err_msg = "x, y is not on the curve.";

            match (&x, &y) {
                (None, None) => Ok(Self { a, b, x, y }),
                (Some(x_value), Some(y_value)) => {
                    if y_value.pow(2) == 
                        x_value.pow(3) + (a * x_value) + b {
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
            if let (Some(s_x), Some(s_y), Some(r_x), Some(r_y)) = (&self.x, &self.y, &rhs.x, &rhs.y) {
                if s_x == r_x && s_y != r_y {
                    return Point::new(None, None, self.a, self.b);
                }
                if s_x != r_x {
                    let s = (r_y - s_y) / (r_x - s_x);
                    let x = s.pow(2) - s_x - r_x;
                    let y = s * (s_x - x) - s_y;
                    
                    return Point::new(Some(x), Some(y), self.a, self.b);
                }
                if self == rhs && s_y == &(0 * s_x) {
                    return Point::new(None, None, self.a, self.b);
                }
                if self == rhs {
                    let s = (3 * s_x.pow(2) + self.a) / (2 + s_y);
                    let x = s.pow(2) - 2 * s_x;
                    let y = s * (s_x -  x) - s_y;
    
                    return Point::new(Some(x), Some(y), self.a, self.b);
                }
            };
            return Err("Out of case".into());
        } 
    }
}

#[cfg(test)]
mod point_test {
    use super::*;

    #[test]
    fn new_test() -> Result<(), Box<dyn Error>>{
        let p1 = Point::new(Some(-1), Some(-1), 5, 7)?;
        let p1_eq = Point::new(Some(-1), Some(-1), 5, 7)?;
        let p2 = Point::new(Some(-1), Some(1), 5, 7)?;
        let inf =  Point::new(None, None, 5, 7)?;
        
        assert!(p1 == p1_eq);
        assert!(p1 != p2);
        

        assert!(&(p1.clone() + inf.clone())? == &p1);
        assert!(&(inf.clone() + p2.clone())? == &p2);
        assert!(&(p1.clone() + p2.clone())? == &inf);

        Ok(())
    }

    #[test]
    fn ep_test() {

    }
}