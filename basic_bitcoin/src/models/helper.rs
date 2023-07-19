//! collection of functions

use std::io::{Read};
use std::error::Error;

// 입력된 u8 4개의 입력값을 litte endian serialize 로 변환 및 계산된 u32 값으로 변환
pub fn little_endian_to_u32(four_u8: &Vec<u8>) -> u32 {
    let mut result: u32 = 0;

    for (idx, &byte) in four_u8.iter().enumerate() {
        result += (byte as u32) << (8 * idx);
    }
    result
} 

// 입력된 u8 8개의 입력값을 litte endian serialize 로 변환 및 계산된 u64 값으로 변환
pub fn little_endian_to_u64(eight_u8: &[u8; 8]) -> u64 {
    let mut result: u64 = 0;

    for (idx, &byte) in eight_u8.iter().enumerate() {
        result += (byte as u64) << (8 * idx);
    }

    result
}

pub fn u32_to_little_endian(input: u32, len: u8) -> Result<Vec<u8>, Box<dyn Error>> {
    match len {
        2 => {
            let bytes = (input as u16).to_le_bytes();
            Ok(bytes.to_vec())
        },
        4 => {
            let bytes = input.to_le_bytes();
            Ok(bytes.to_vec())
        },
        _ => Err("Invalid length".into())
    }
}

// 가변 정수 표현
// 다음의 규칙으로 0 ~ (2^64 - 1) 사이의 숫자를 표현
//
//  정수 범위                 Variats 표현 방법                             예
//    0~252            1 byte                                  100 -> 0x64 (일반적 표현 방시과 동일)
// 253 ~ 2^16-1        접두부 0xfd 이후 2bytes little endian    255 -> 0xfdff00
//                                                             555 -> 0xfd2b02              
// 2^16 ~ 2^32-1       접두부 0xfe 이후 4bytes little endian    70015 -> 0xfe7f110100 
// 2^32 ~ 2^64-1       접두부 0xff 이후 8bytes little endian    81005558675309 -> 0xff6dc7ed3e60100000

/// stream 으로 부터 필요한 bytes 만큼 읽고 정수로 반환
pub fn read_varint<R: Read>(reader: &mut R) -> Result<u64, std::io::Error> {
    let mut buf = Vec::<u8>::new();
 
    reader.read_to_end(&mut buf)?;
    println!("{:?}", buf);
    let i = buf[0];

    let mut buf = [0u8; 8];
    match i {
        0xfd => {
            let mut read_buf = [0u8; 2];
            reader.read_exact(&mut read_buf)?;

            buf[0] = read_buf[0];
            buf[1] = read_buf[1];

            Ok(u64::from_le_bytes(buf))
        },
        0xfe => {
            let mut read_buf = [0u8; 4];
            reader.read_exact(&mut read_buf)?;

            buf[0] = read_buf[0];
            buf[1] = read_buf[1];
            buf[2] = read_buf[2];
            buf[3] = read_buf[3];

            Ok(u64::from_le_bytes(buf))
        },
        0xff => {
            reader.read_exact(&mut buf)?;

            Ok(u64::from_le_bytes(buf))
        },
        _ => {
            Ok(u64::from(i))
        },
    }
}

// read_varint 와 반대로 정수값을 받아 varint 형식으로 변환된 bytes 를 반환
pub fn encode_varint(input: u32) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut v = Vec::<u8>::new();

    if  input < 0xfd {
        v.push(input as u8);
        Ok(v)
    } else if input < 0x10000 {
        v.push(0xfd);
        v.append(&mut u32_to_little_endian(input, 2)?);
        Ok(v)
    } else if input < 0xffffffff {
        v.push(0xfe);
        v.append(&mut u32_to_little_endian(input, 4)?);
        Ok(v)
    // } else if input < 0x10000000000000000 {
    //     v.push(input as u8); // 현재 입력 가능 값이 u32 로 해당 값은 범위를 초과함.
    //     Ok(v)
    } else {
        Err("input value is too large".into())
    }

}
 

#[cfg(test)]
mod hleper_test {
    use super::*;

    #[test]
    fn litte_endian() {
        let mut data: &[u8] = &[0xfd, 0x01, 0x02, 0xfe, 0x03, 0x04, 0x05, 0x06, 0xff, 0x07, 0x08, 0x09, 0x0a];
        let mut data2: &[u8] = &[0xff, 0x6d, 0xc7, 0xed, 0x3e, 0x60, 0x10, 0x00, 0x00, 0x64];
        let num1 = read_varint(&mut data).unwrap();
        let num2 = read_varint(&mut data).unwrap();
        let num3 = read_varint(&mut data2).unwrap();
        let num4 = read_varint(&mut data2).unwrap();
        println!("num1: {}", num1);
        println!("num2: {}", num2);
        println!("num3: {}", num3);
        println!("num4: {}", num4);
    }
}