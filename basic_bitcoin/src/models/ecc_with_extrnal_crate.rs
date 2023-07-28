// // 1. Scalar
// //  secp256k1 곡선 위의 값을 나타냄. 보통 비밀키(private key) 또는 다른 형태의 키를 나타냄
// //  256 bits 의 큰 수로, 곡선 연산의 경우 modular P 연산을 사용하여 정규화 함.
// //
// // 2. ProjectivePoint
// //  secp256k1 곡선 위의 점을 projective 좌표계로 나타냄
// //  Projective 좌표계는 각 점을 (x, y, z) 세 개의 정보로 표시하는데, 여기서 z 는 3차원 축의 좌표값을
// //  의미하는 것이 아니라, (x/z, y/z)  와 2차원의 '비율'만을 표현하기 위한 정보이다. 
// //  예를 들면 (x, y, z) 와 (2x, 2y, 2z) 는 일반 유클리드 좌표계에서는 다른 점을 의미하지만, 
// //  projective 좌표계에서는 (x/z, y/z) = (2x/2z, 2y/2z) 이므로 동일한 점을 나타낸다.
// //  
// //  이 좌표계는 그래픽스, 컴퓨터 비전, 그리고 여기에서 논의하는 것처럼 타원 곡선 암호법 등 여러 분야에서 사용된다. 
// //  특히 타원 곡선 암호법에서는 큰수를 projective 좌표계를 사용하여 연산을 훨씬 더 효율적으로 수행할 수 있다.
// //
// //  3. AffinePoint
// //  secp256k1 곡선 위의 점을 Affine 좌표계로 나타냄.
// //  Affine 좌표계는 연산을 수행하기에는 비효율적일 수 있지만, 
// //  점을 인간이 이해할 수 있는 형태로 표현하는 데는 직관적이기 때문에 적합함.
// //  따라서, ProjectivePoint를 AffinePoint로 변환하여 결과를 인코딩하거나 디코딩하는 경우가 많음
// use k256::{Scalar, ProjectivePoint, AffinePoint};
// use k256::elliptic_curve::sec1::{EncodedPoint, FromEncodedPoint};
// use std::ops::{Add, Mul};

// struct S256Point {
//     point: ProjectivePoint,
// }

// impl S256Point {
//     pub fn new(x: Scalar, y: Scalar) -> Self {
//         let point = ProjectivePoint::from(AffinePoint::new(
//             Scalar::from(x), 
//             Scalar::from(y),
//             ));

//         S256Point { point: ProjectivePoint::from(affine_point) }
//     }
// }

// impl Add for S256Point {
//     type Output = Self;

//     fn add(self, rhs: Self) -> Self::Output {
//         Self {
//             point: self.point + rhs.point,
//         }
//     }
// }

// impl Mul<Scalar> for S256Point {
//     type Output = Self;

//     fn mul(self, rhs: Scalar) -> Self::Output {
//         Self {
//             point: self.point * rhs,
//         }
//     }
// }

// #[cfg(test)]
// mod test {
//     use super::*;

//     #[test]
//     fn new_test() {
//         let x1 = Scalar::from(10u64);
//         let y1 = Scalar::from(20u64);
//         let point1 = S256Point::new(x1, y1);

//         let x2 = Scalar::from(30u64);
//         let y2 = Scalar::from(30u64);
//         let point2 = S256Point::new(x2, y2);

//         let result_add = point1.add(&point2);
//         let scalar = Scalar::from(5u64);
//         let result_multiply = point1.multiply(&scalar);

//         println!("Result of addition: {:?}", result_add);
//         println!("Result of multiplication: {:?}", result_multiply);
//     }
// }