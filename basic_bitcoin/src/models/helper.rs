//! collection of functions

use std::io::Read;
use std::error::Error;
use std::iter::repeat;
use sha2::{Sha256, Digest};
use num::bigint::{BigUint, ToBigUint};
use num::Integer;
use num_traits::{ToPrimitive, Zero};
use ripemd::Ripemd160;

#[allow(dead_code)]
pub const SIGHASH_ALL: u64 = 1;
#[allow(dead_code)]
pub const SIGHASH_NONE: u64 = 2;
#[allow(dead_code)]
pub const SIGHASH_SINGLE: u64 = 3;
const BASE58_ALPHABET: &str = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";


pub fn hash160(s: &Vec<u8>) -> Vec<u8> {
    let mut hasher_sha256 = Sha256::new();
    hasher_sha256.update(s);

    let mut hasher_ripemd160 = Ripemd160::new();
    hasher_ripemd160.update(hasher_sha256.finalize_reset());
    
    let hasher160 = hasher_ripemd160.finalize().to_vec();

    hasher160
}

pub fn hash256(s: &Vec<u8>) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(s);
    let re_hasher = hasher.finalize_reset(); 
    hasher.update(re_hasher);

    hasher.finalize().to_vec()
}

/// Base58 표현된 주소 Vec 에서  20bytes Hash 로 encode 
pub fn encode_base58(s: &Vec<u8>) -> String {
    let mut count = 0usize;

    // determine how many 0 bytes (b'\x00') s starts with
    // 입력받는 주소의 앞부분에 version 정보가 있으며, 이때 그 정보가 '0' 으로 표시될 수 있다.
    // 그런데 bytes array 를 ascii code 로 치환하는 과정에서 b'\x00' 은  ascii Null 을 의미하므로 
    // 무시되거나 종료의 의미르 가지므로 오류를 발생시킬 수 있다. 
    // 따라서 Base58 변환, Bitcoin 처리에서 b'\x00' 은 '1'로 표현한다.
    for c in s.iter() {
        if c == &0 {
            count += 1;
        } else {
            break;
        }
    }

    // convert to big endian integer
    let mut num = BigUint::from_bytes_be(s);
    let mut result = String::new();

    // 58 로 나누어 그 나머지는 해당 자리의 표현 문자로 치환하고, 몫은 반복하여 58 로 나눔
    while num > BigUint::zero() {
        let div_rem = num.div_rem(&58i32.to_biguint().unwrap());
        num = div_rem.0;
        result.insert(0, BASE58_ALPHABET.chars().nth(div_rem.1.to_usize().unwrap()).unwrap());
    }
    result.insert_str(0, &repeat('1').take(count).collect::<String>());
    result
}

pub fn encode_base58_checksum(s: &Vec<u8>) -> String {
    let mut s_clone = s.clone();
    let mut hash256 = hash256(&s);
    s_clone.append(&mut hash256);

    return encode_base58(&s_clone);
}

/// base58 이란?
/// 
/// Bitcoin 에서 주로 주소와 같은 고유 식별잘르 표현하는데 사용디는 인코딩 방식
///  숫자와 영문자를 사용하여 데이터를 표현하되, 숫자 '0'(영)과 소문자 'o'(오), 
/// 대문자 'I'(아이)와 소문자 'l'(엘), 대문자 'O'(오)와 숫자 '0'(영)과 같이 
/// 혼동하기 쉬운 문자를 제외한 58개의 문자를 사용
///
///  - BASE58_ALPHABET: 123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz
/// 
pub fn decode_base58(s: &str) ->  Result<Vec<u8>, Box<dyn Error>> {
    let mut num  = BigUint::from(0u64);

    for c in s.chars() {
        match BASE58_ALPHABET.find(c) {
            Some(x) => num = num * 58u64 + x as u64,
            None => return Err("Invalid character".into()),
        }
    }

    // bitcoin 주소 구조: 25bytes = (version: 1byte) + (Payload: 20bytes) + (Checksum: 4bytes)
    // Version : P2PKH(pay-to-public-key-hash) 0x00
    //           P2SH(pay-to-script-hash) 0x05
    // Payload : 주로 SHA-256 hash  + RIPEMD-160 hash 가 사용
    // Checksum: 데이터의 무결성을 확인하는데 사용하는 값
    //           Payload 와 Version Byte 에 double SHA-256 을 적용한 후, 첫 4bytes 를 사용
    let combined: Vec<u8> = num.to_bytes_be()[..25].to_vec();
    let checksum = &combined[combined.len() - 4..];
    let real_checksum = &hash256(&combined[..combined.len() - 4].to_vec())[..4];
    if checksum != real_checksum {
        return Err("Bad address".into())
    }

    Ok(combined[1..combined.len() - 4].to_vec())
}





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

/// stream 으로 부터 필요한 bytes 만큼 읽고 정수로 반환
/// * 가변 정수 표현
/// 다음의 규칙으로 0 ~ (2^64 - 1) 사이의 숫자를 표현
///
/// ```
///  정수 범위      Variats 표현 방법       예 
///   0~252         1 byte              100->0x64
/// 253~2^16-1      0xfd + 2bytes LE    255->0xfdff00
///                                     555->0xfd2b02            
/// 2^16~2^32-1     0xfe + 4bytes LE    70015->0xfe7f110100 
/// 2^32~2^64-1     0xff + 8bytes LE    81005558675309
///                                     ->0xff6dc7ed3e60100000
/// ```
pub fn read_varint<R: Read>(reader: &mut R) -> Result<u64, std::io::Error> {
    let buf = [0u8; 1];
 
    reader.read_to_end(&mut buf.to_vec())?;

    let i = buf[0];
    match i {
        0xfd => {
            let mut read_buf = [0u8; 2];
            reader.read_exact(&mut read_buf)?;

            Ok(u16::from_le_bytes(read_buf) as u64)
        },
        0xfe => {
            let mut read_buf = [0u8; 4];
            reader.read_exact(&mut read_buf)?;

            Ok(u32::from_le_bytes(read_buf) as u64)
        },
        0xff => {
            let mut read_buf = [0u8; 8];
            reader.read_exact(&mut read_buf)?;

            Ok(u64::from_le_bytes(read_buf))
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