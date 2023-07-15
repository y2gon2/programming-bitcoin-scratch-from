use std::ops::{Deref, DerefMut};
use sha2::{Sha256, Sha512, Digest};


#[derive(Clone, Debug)]
pub struct Stack(Vec<Vec<u8>>);

impl Deref for Stack {
    type Target = Vec<Vec<u8>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Stack {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0        
    }
}

/// 입력된 수 (예제는 16 bits 수로 가정) 에 대해서 little-endian 방식으로 변환 method
/// 
/// ex. 500 (10 진수) 을 little endian 으로 변환환다고 하면
///     500 (10 진수)  ->  0000 0001 1111 0100 
/// little-endain 으로 ->  1111 0100 0000 0001
/// 
/// 해당 함수에서는 i32 값을 받아서 그 절대값을 little-endian 으로 저장
/// 음수의 경우 가장 높은(왼쪽) bit 를 1로 재설정 필요
pub fn encode_num(mut num: i32) -> Vec<u8> {
    if num == 0 { return vec![] }

    let abs_num = num.abs();
    let negative = num < 0;
    let mut result = vec![];

    // 뒤 8bits 부터 저장
    while num != 0 {
        result.push((num & 0xff) as u8);
        num >>= 8;
    }

    // 음수(1000 0000) / 양수(0000 00)의 표현을 위해 1byte 를 더 추가하는건가???
    if result.last().unwrap() & 0x80 != 0 {
        if negative {
            result.push(0x80);
        } else {
            result.push(0);
        }

    // 마지막 바이트의 상위 비트가 설정되어 있지 않고, 입력된 숫자가 음수인 경우
    //  마지막 바이트의 상위 비트를 설정하여 음수임을 나타냄 (????)
    } else if negative {
        let last_index = result.len() - 1;
        result[last_index] != 0x80;
    }
    result
}

pub fn decode_num(element:&mut Vec<u8>) -> i32 {
    if element.is_empty() { return 0 }

    let mut big_endian = element.clone();
    big_endian.reverse();
    let mut negative = true;
    let mut result = 0i32;

    if big_endian[0] & 0x80 != 0 {
        negative = true;
        result = (big_endian[0] & 0x7f) as i32;
    } else {
        negative = false;
        result = big_endian[0] as i32;
    }

    // 앞에서 result 32bits 중 bit-endian 가장 앞의 8bits 마 받은 상황이므로
    // 나머지 3bytes 를 추가로 입힘
    for c in &big_endian[1..] {
        result <<= 8;
        result += *c as i32;
    }

    if negative {
        -result
    } else {
        result
    }
}


/// op_dup  : 해당 element 를 복사하여 stack 위에 저장
/// op_code_functions : 118 
pub fn op_dup(stack: &mut Stack) -> bool {
    if stack.len()< 1 { return false; }

    let element = stack[stack.len() - 1].clone();
    stack.push(element);
    true
}

/// op_hash256 : stack element 를 가져와서 hash256 hashing 된 값을 stack 에 저장
/// op_code_function: 170 
pub fn po_hash256(stack: &mut Stack) -> bool {
    if stack.len() < 1 { return false; }

    if let Some(element) = stack.pop() {
        let mut hasher = Sha256::new();
        hasher.update(element);
        stack.push(hasher.finalize().to_vec());
    }
    true
}

/// OP-code : 0  
pub fn op_0(stack: &mut Stack) -> bool {
    stack.push(encode_num(0));
    true
}

/// op_code : 79
pub fn op_1negate(stack: &mut Stack) -> bool {
    stack.push(encode_num(-1));
    true
}

/// op_code : 81
pub fn op_1(stack: &mut Stack) -> bool {
    stack.push(encode_num(1));
    true
}

/// op_code : 82
pub fn op_2(stack: &mut Stack) -> bool {
    stack.push(encode_num(2));
    true
}

/// op_code : 83
pub fn op_3(stack: &mut Stack) -> bool {
    stack.push(encode_num(3));
    true
}

/// op_code : 84
pub fn op_4(stack: &mut Stack) -> bool {
    stack.push(encode_num(4));
    true
}

/// op_code : 85
pub fn op_5(stack: &mut Stack) -> bool {
    stack.push(encode_num(5));
    true
}

/// op_code : 86
pub fn op_6(stack: &mut Stack) -> bool {
    stack.push(encode_num(6));
    true
}

/// op_code : 87
pub fn op_7(stack: &mut Stack) -> bool {
    stack.push(encode_num(7));
    true
}

/// op_code : 88
pub fn op_8(stack: &mut Stack) -> bool {
    stack.push(encode_num(8));
    true
}

/// op_code : 89
pub fn op_9(stack: &mut Stack) -> bool {
    stack.push(encode_num(9));
    true
}

/// op_code : 90
pub fn op_10(stack: &mut Stack) -> bool {
    stack.push(encode_num(10));
    true
}


/// op_code : 91
pub fn op_11(stack: &mut Stack) -> bool {
    stack.push(encode_num(11));
    true
}

/// op_code : 92
pub fn op_12(stack: &mut Stack) -> bool {
    stack.push(encode_num(12));
    true
}

/// op_code : 93
pub fn op_13(stack: &mut Stack) -> bool {
    stack.push(encode_num(13));
    true
}

/// op_code : 94
pub fn op_14(stack: &mut Stack) -> bool {
    stack.push(encode_num(14));
    true
}

/// op_code : 95
pub fn op_15(stack: &mut Stack) -> bool {
    stack.push(encode_num(15));
    true
}

/// op_code : 96
pub fn op_16(stack: &mut Stack) -> bool {
    stack.push(encode_num(16));
    true
}

/// op_code : 97
pub fn op_nop(stack: &mut Stack) -> bool {
    true
}

/// op_code : 99
/// 일반 programming language 에서 if 와 유사한 역할
/// 사용의 예
/// -----------------------------------------------------
/// <condition> OP_IF
///     <code to be executed if condition is true>
/// OP_ENDIF
/// -----------------------------------------------------
/// stack 맨 위 요소(<condition>)를 참(일반적으로 0이 아닌) 또는 거짓(일반적으로 0) 으로 해석
/// 그 결과에 따라 script 실행 흐름을 제어
pub fn op_if(stack: &mut Stack, items: &mut Vec<u8>) -> bool {
    if stack.is_empty() { return false; }

    let mut true_items = Vec::<u8>::new();
    let mut false_items = Vec::<u8>::new();
    let mut current_array = &mut true_items;

    let mut found = false;
    let mut num_endifs_needed = 1usize;

    while !items.is_empty() {
        let item = items.remove(0);
        match item {
            99 | 100 => {
                num_endifs_needed += 1;
                current_array.push(item);
            },
            103 if num_endifs_needed == 1 => {
                current_array = &mut false_items;
            },
            104 => {
                if num_endifs_needed == 1 {
                    found = true;
                    break;
                } else {
                    num_endifs_needed -= 1;
                    current_array.push(item);    
                }
            },
            _ => { current_array.push(item) }
        }
    }
    
    if !found { return false }
    
    let mut element = stack.pop().unwrap();

    if decode_num(&mut element) == 0 {
        items.splice(0..0, false_items);
    } else {
        items.splice(0..0, true_items);
    }

    true
}

/// op_code : 100
/// op_if 의 반대로 <condition> 이 거짓(0) 일때 조건을 수행
pub fn op_notif(stack: &mut Stack, items: &mut Vec<u8>) -> bool {
    if stack.is_empty() { return false; }

    let mut true_items = Vec::<u8>::new();
    let mut false_items = Vec::<u8>::new();
    let mut current_array = &mut true_items;
    let mut found = false;
    let mut num_endifs_needed = 1i32;

    while !items.is_empty() {
        let item = items.remove(0);

        match item {
            99 | 100 => {
                num_endifs_needed += 1;
                current_array.push(item);
            },
            103 if num_endifs_needed == 1 => {
                current_array = &mut false_items;
            },
            104 => {
                if num_endifs_needed == 1 {
                    found = true;
                    break;
                } else {
                    num_endifs_needed -= 1;
                    current_array.push(item);
                }
            },
            _ => {
                current_array.push(item);
            },
        }
    }
    if !found { return false; }

    let mut element = stack.pop().unwrap();
    if decode_num(&mut element) == 0 {
        items.splice(0..0, true_items);
    } else {
        items.splice(0..0, false_items);
    }
    true
}

/// op_code : 105
/// stack 최상단 element 가 true (0이 아닌) 지를 확인
/// 해당 element  0이 아니라면 script 를 계속 진행
///               0이면 script 실행을 즉시 중지
/// 위 조건에 따라 op_verify  는 script 가 어떤 조건을 만족하는지 확인하는데 사용
/// ex. 특정 signiture 가 올바른지 또는 어떤 계산의 결과가 예상하는 값인지를 확인
pub fn op_verify(stack: &mut Stack) -> bool {
    if stack.is_empty() { return false; }

    let mut element = stack.pop().unwrap();
    if decode_num(&mut element) == 0 {
        return false;
    }
    true
}

/// op_code : 106
/// 거래의 유효성을 '무효' 로 설정하여 거래가 blockchain 에 포함되지 않도록함
/// 주로 'data carrier' 로 사용됨. 
///   -> Bitcoin context 에서 transaction 의 일부로 data 를 blockchain 에 저장하는 기능을 의미
///      op_return 연산자를 통해 bitcoin transaction 에 임의의 데이터를 포함시킬 수 있는 방법을 제공
///      이 데이터는 거래의 유효성에 영향을 주지 않지만, 외부 응용 프로그램에서 조회하거나 사용할 수 있음
///      예를 들어, 시간 증명, 메타데이터 저장, 디지털 자산 인증, 메시지 등의 목적으로 사용할 수 있음
/// 
/// OP_RETURN 이 script 에 나타나면 해당 script 는 즉시 중단됨.
/// 따라서 OP_RETURN 이후에 나타나는 data 는 script 의 실행에 영항을 주지 않고 무시됨.
/// 거래 자체는 블록체인에 포함되지 않으나, OP_RETURN 다음에 오는 데이터는 
/// blockchain 의 transaction pool에 일시적으로 저장
/// 외부의 application 들이 이 data 를 읽을 수 있음.
pub fn op_(stack: &mut Stack) -> bool {
    false
}

/// op_code :
pub fn op_(stack: &mut Stack) -> bool {
    stack.push(encode_num());
    true
}

/// op_code :
pub fn op_(stack: &mut Stack) -> bool {
    stack.push(encode_num());
    true
}

/// op_code :
pub fn op_(stack: &mut Stack) -> bool {
    stack.push(encode_num());
    true
}

/// op_code :
pub fn op_(stack: &mut Stack) -> bool {
    stack.push(encode_num());
    true
}

/// op_code :
pub fn op_(stack: &mut Stack) -> bool {
    stack.push(encode_num());
    true
}

/// op_code :
pub fn op_(stack: &mut Stack) -> bool {
    stack.push(encode_num());
    true
}

/// op_code :
pub fn op_(stack: &mut Stack) -> bool {
    stack.push(encode_num());
    true
}

/// op_code :
pub fn op_(stack: &mut Stack) -> bool {
    stack.push(encode_num());
    true
}

/// op_code :
pub fn op_(stack: &mut Stack) -> bool {
    stack.push(encode_num());
    true
}

/// op_code :
pub fn op_(stack: &mut Stack) -> bool {
    stack.push(encode_num());
    true
}

/// op_code :
pub fn op_(stack: &mut Stack) -> bool {
    stack.push(encode_num());
    true
}

/// op_code :
pub fn op_(stack: &mut Stack) -> bool {
    stack.push(encode_num());
    true
}

/// op_code :
pub fn op_(stack: &mut Stack) -> bool {
    stack.push(encode_num());
    true
}

/// op_code :
pub fn op_(stack: &mut Stack) -> bool {
    stack.push(encode_num());
    true
}

/// op_code :
pub fn op_(stack: &mut Stack) -> bool {
    stack.push(encode_num());
    true
}

/// op_code :
pub fn op_(stack: &mut Stack) -> bool {
    stack.push(encode_num());
    true
}

/// op_code :
pub fn op_(stack: &mut Stack) -> bool {
    stack.push(encode_num());
    true
}

/// op_code :
pub fn op_(stack: &mut Stack) -> bool {
    stack.push(encode_num());
    true
}

/// op_code :
pub fn op_(stack: &mut Stack) -> bool {
    stack.push(encode_num());
    true
}

/// op_code :
pub fn op_(stack: &mut Stack) -> bool {
    stack.push(encode_num());
    true
}

/// op_code :
pub fn op_(stack: &mut Stack) -> bool {
    stack.push(encode_num());
    true
}

/// op_code :
pub fn op_(stack: &mut Stack) -> bool {
    stack.push(encode_num());
    true
}

/// op_code :
pub fn op_(stack: &mut Stack) -> bool {
    stack.push(encode_num());
    true
}

/// op_code :
pub fn op_(stack: &mut Stack) -> bool {
    stack.push(encode_num());
    true
}

/// op_code :
pub fn op_(stack: &mut Stack) -> bool {
    stack.push(encode_num());
    true
}

/// op_code :
pub fn op_(stack: &mut Stack) -> bool {
    stack.push(encode_num());
    true
}

/// op_code :
pub fn op_(stack: &mut Stack) -> bool {
    stack.push(encode_num());
    true
}

/// op_code :
pub fn op_(stack: &mut Stack) -> bool {
    stack.push(encode_num());
    true
}

/// op_code :
pub fn op_(stack: &mut Stack) -> bool {
    stack.push(encode_num());
    true
}

/// op_code :
pub fn op_(stack: &mut Stack) -> bool {
    stack.push(encode_num());
    true
}

/// op_code :
pub fn op_(stack: &mut Stack) -> bool {
    stack.push(encode_num());
    true
}

/// op_code :
pub fn op_(stack: &mut Stack) -> bool {
    stack.push(encode_num());
    true
}

/// op_code :
pub fn op_(stack: &mut Stack) -> bool {
    stack.push(encode_num());
    true
}

/// op_code :
pub fn op_(stack: &mut Stack) -> bool {
    stack.push(encode_num());
    true
}

/// op_code :
pub fn op_(stack: &mut Stack) -> bool {
    stack.push(encode_num());
    true
}

/// op_code :
pub fn op_(stack: &mut Stack) -> bool {
    stack.push(encode_num());
    true
}

/// op_code :
pub fn op_(stack: &mut Stack) -> bool {
    stack.push(encode_num());
    true
}

/// op_code :
pub fn op_(stack: &mut Stack) -> bool {
    stack.push(encode_num());
    true
}

/// op_code :
pub fn op_(stack: &mut Stack) -> bool {
    stack.push(encode_num());
    true
}

/// op_code :
pub fn op_(stack: &mut Stack) -> bool {
    stack.push(encode_num());
    true
}

/// op_code :
pub fn op_(stack: &mut Stack) -> bool {
    stack.push(encode_num());
    true
}

/// op_code :
pub fn op_(stack: &mut Stack) -> bool {
    stack.push(encode_num());
    true
}

/// op_code :
pub fn op_(stack: &mut Stack) -> bool {
    stack.push(encode_num());
    true
}

/// op_code :
pub fn op_(stack: &mut Stack) -> bool {
    stack.push(encode_num());
    true
}

/// op_code :
pub fn op_(stack: &mut Stack) -> bool {
    stack.push(encode_num());
    true
}

/// op_code :
pub fn op_(stack: &mut Stack) -> bool {
    stack.push(encode_num());
    true
}

/// op_code :
pub fn op_(stack: &mut Stack) -> bool {
    stack.push(encode_num());
    true
}

/// op_code :
pub fn op_(stack: &mut Stack) -> bool {
    stack.push(encode_num());
    true
}

/// op_code :
pub fn op_(stack: &mut Stack) -> bool {
    stack.push(encode_num());
    true
}

/// op_
/// op_code_function: 


/// op_
/// op_code_function: 
fn ex() {}


#[cfg(test)]
mod op_test {
    use super::*;

    #[test]
    fn sha256_test() {
        // create a Sha256 object
        let mut hasher = Sha256::new();

        // write input message
        hasher.update(b"hello world");

        // read hash digest and consume hasher
        let result = hasher.finalize();

        println!("{:?}", result);
    }
}