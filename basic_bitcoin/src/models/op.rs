use std::ops::{Deref, DerefMut};
use hex::encode;
use sha1::{Sha1};
use sha2::{Sha256, Sha512, Digest};
use ripemd::Ripemd160;


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
pub fn encode_num(num: i32) -> Vec<u8> {
    if num == 0 { return vec![] }

    let abs_num = num.abs();
    let negative = num < 0;
    let mut result = vec![];

    let mut num_clone = num.clone();
    // 뒤 8bits 부터 저장
    while num_clone != 0 {
        result.push((num_clone & 0xff) as u8);
        num_clone >>= 8;
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

pub fn decode_num(element:&Vec<u8>) -> i32 {
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

    if decode_num(&element) == 0 {
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
    if decode_num(&element) == 0 {
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
    if decode_num(&element) == 0 {
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
pub fn op_return(stack: &mut Stack) -> bool {
    false
}

/// op_code : 107
/// main stack 최상위 element 를 꺼내서 alt_stack 으로 이동
/// alt_stack - main stack 의 값을 일시적으로 제거하거나 나중에 사용할 값을 저장하기 위해 사용
pub fn op_totaltstack(stack: &mut Stack, alt_stack: &mut Stack) -> bool {
    if stack.is_empty() { return false; }

    if let Some(val) = stack.pop() {
        alt_stack.push(val);
    };
    true
}

/// op_code : 108
/// alt_stack 최상위 element 를 꺼내서 main stack 으로 이동
pub fn op_fromaltstack(stack: &mut Stack, alt_stack: &mut Stack) -> bool {
    if alt_stack.is_empty() { return false; }
    
    if let Some(val) = alt_stack.pop() {
        stack.push(val);
    }
    true
}

/// op_code : 109
/// main stack 최상위 2개의 element 를 drop
/// transaction 유효성 검사, output 잠금 해제 등을 위해 사용
pub fn op_2drop(stack: &mut Stack) -> bool {
    let length = stack.len();
    
    if length < 2 { return false; }
    
    stack.pop().unwrap();
    stack.pop().unwrap();
    true
}

/// op_code : 110
/// main stack 최상단 2 개 element 를 복제하여 동일한 순서대로 stack 에 추가
pub fn op_2dup(stack: &mut Stack) -> bool {
    let length = stack.len();

    if length < 2 { return false; }

    let end_elements = stack
        .iter()
        .rev()
        .take(2)
        .rev()
        .cloned()
        .collect::<Vec<_>>();

    stack.extend(end_elements);

    true
}

/// op_code : 111
/// main stack 최상단 3 개 element 를 복제하여 동일한 순서대로 stack 에 추가
pub fn op_3dup(stack: &mut Stack) -> bool {
    let length = stack.len();

    if length < 3 { return false; }

    let end_elements = stack
        .iter()
        .rev()
        .take(3)
        .rev()
        .cloned()
        .collect::<Vec<_>>();

    stack.extend(end_elements);

    true
}

/// op_code : 112
/// stack 두 번째 쌍의 두 항목을 복사하여 stack 의 맨 위로 가져옴
/// vec![... x, y, z, w] => vec![... x, y, z, w, x, y]
pub fn op_2over(stack: &mut Stack) -> bool {
    let length = stack.len();

    if length < 4 { return false; }

    stack.extend(stack[length - 4.. length - 2].to_vec());
    true
}

/// op_code : 113
/// stack 세 번째 쌍의 두 항목을 제거하고 stack 의 맨 위로 이동
/// vec![... x, y, z, w, a, b] => vec![... z, w, a, b, x, y]
pub fn op_2rot(stack: &mut Stack) -> bool {
    let length = stack.len();

    if length < 6 { return false; }

    let slice = stack[length - 6..length - 4].to_vec();
    stack.splice(length - 6..length - 4, []);
    stack.extend(slice);
    true
}

/// op_code : 114
/// stack 상위 두 쌍의 항목를 서로 바꿈
/// vec![... x, y, z, w] => vec![... z, w, x, y]
pub fn op_2swap(stack: &mut Stack) -> bool {
    let length = stack.len();

    if length < 4 { return false; }

    let slice = stack[length - 4..length - 2].to_vec();
    stack.splice(length - 4..length - 2, []);
    stack.extend(slice);
    true
}

/// op_code : 115
/// stack 최상단 element 가 0 (거짓) 이 아닌 경우, 그 값을 복사하여 최상단에 추가
pub fn op_ifdup(stack: &mut Stack) -> bool {
    if let Some(top) = stack.last() {
        if decode_num(top) !=  0{
            stack.push(*top);
        }
        return true;
    } else {
        return false;
    }
}

/// op_code : 116
/// stack length 값을 stack 최상단에 추가
pub fn op_depth(stack: &mut Stack) -> bool {
    let depth = encode_num(stack.len() as i32);
    stack.push(depth);

    true
}

/// op_code : 117
/// stack 최상단 element 를 버림
pub fn op_drop(stack: &mut Stack) -> bool {
    if stack.is_empty() { return false; }

    stack.pop().unwrap();
    true
}

/// op_dup  : stack 최상단 element 를 복사하여 stack 위에 저장
/// op_code_functions : 118 
pub fn op_dup(stack: &mut Stack) -> bool {
    if stack.len() < 1 { return false; }

    stack.push(*stack.last().unwrap());
    true
}

/// op_code : 119
/// stack 최상단에서 두번째 element 를 버림
pub fn op_nip(stack: &mut Stack) -> bool {
    if stack.len() < 2 { return false; }

    stack.remove(stack.len() - 2);
    true
}

/// op_code : 120
/// stack 최상단에서 두번째 element 를 복사하여 stack 최상단에 추가
pub fn op_over(stack: &mut Stack) -> bool {
    let length = stack.len();
    if length < 1 { return false; }

    stack.push(stack[length - 2]);
    true 
}

/// op_code : 121
/// stack 최상단 element 를 가져와서 해당 element 숫자 번째 있는 값을 복사하여 최상단에 추가
/// vec![... x, y, z, 2] -> vec![... x, y, z, y]
pub fn op_pick(stack: &mut Stack) -> bool {
    if stack.is_empty() { return false; }

    let n = decode_num(&stack.pop().unwrap()) as usize;

    if n >= 0 && stack.len() <= n { return false; }

    stack.push(stack[stack.len() - n]);
    true
}

/// op_code : 122
/// stack 최상단 element 를 가져와서 해당 element 숫자 번째 있는 값을 이동하여 최상단에 추가
/// vec![... x, y, z, 2] -> vec![... x, z, y]
pub fn op_roll(stack: &mut Stack) -> bool {
    if stack.is_empty() { return false; }

    let n = decode_num(&stack.pop().unwrap()) as usize;

    if n >=  0 && stack.len() <= n { return false; }
    
    stack.push(stack.remove(stack.len() - n));
    true
}

/// op_code : 123
/// stack 최상단에서 3번째 값을 최상단으로 이동
pub fn op_rot(stack: &mut Stack) -> bool {
    let length = stack.len();

    if length < 3 { return false; }

    stack.push(stack.remove(length - 3));
    true
}

/// op_code : 124
/// stack 최상단에서 2번째 값을 최상단으로 이동
pub fn op_swap(stack: &mut Stack) -> bool {
    let length = stack.len();
    if length < 2 { return false; }

    stack.push(stack.remove(length - 2));
    true
}

/// op_code : 125
/// stack 최상단에서 2번째 값을 복사하여 최상단에 추가
pub fn op_tuck(stack: &mut Stack) -> bool {
    let length = stack.len();
    if length < 2 { return false; }

    stack.push(stack[length - 2]);
    true
}

/// op_code : 130
/// stack 최상단 항목의 크기를 byte 단위로 측정하고 그 값을 stack 최상단에 추가
/// 예를 들어, stack 최상단 항목이 vec<u8> 일 경우, 
/// 해당 vec 길이를 byte 단위로 측정하여 그 값을 stack 에 추가 
pub fn op_size(stack: &mut Stack) -> bool {
    if stack.is_empty() { return false; }

    let n = encode_num(stack.last().unwrap().len() as i32);
    stack.push(n);
    true
}

/// op_code : 135
/// stack 최상단 element 와 두번째 element 가 동일하면 1, 다르면 0 을 추가
pub fn op_equal(stack: &mut Stack) -> bool {
    if stack.len() < 2 { return false; }

    let element1 = stack.pop().unwrap();
    let element2 = stack.pop().unwrap();

    if element1 == element2 {
        stack.push(encode_num(1));
    } else {
        stack.push(encode_num(0))
    }
    true
}

/// op_code : 136
/// op_equal 과 동일하게 최상단 element 와 그 아래 element 가 동일한지 비교하지만
/// op_equalverify 는 비교 결과 (참, 거짓) 을 stack 에 저장하지 않음.
pub fn op_equlverify(stack: &mut Stack) -> bool {
    if stack.len() < 2 { return false; }

    let element1 = stack.pop().unwrap();
    let element2 = stack.pop().unwrap();

    if element1 != element2 {
        return false;
    } 
    true
}

/// op_code : 139
/// 최상단 element + 1
pub fn op_1add(stack: &mut Stack) -> bool {
    if stack.is_empty() { return false; }

    let element = decode_num(&stack.pop().unwrap());
    stack.push(encode_num(element + 1));
    true
}

/// op_code : 140
/// 최상단 element - 1
pub fn op_1sub(stack: &mut Stack) -> bool {
    if stack.is_empty() { return false; }

    let element = decode_num(&stack.pop().unwrap());
    stack.push(encode_num(element - 1));
    true
}

/// op_code : 143
/// 최상단 -element
pub fn op_negate(stack: &mut Stack) -> bool {
    if stack.is_empty() { return false; }
    
    let element = decode_num(&stack.pop().unwrap());
    stack.push(encode_num(-element));
    true
}

/// op_code : 144
pub fn op_abs(stack: &mut Stack) -> bool {
    if stack.is_empty() { return false; }

    let n = decode_num(&stack.pop().unwrap());
    if element < 0 {
        stack.push(encode_num(-n));
    } else {
        stack.push(encode_num(n));
    }
    true
}

/// op_code : 145
/// 최상위 element 값이  0  -> 1
///                   그외  -> 0
pub fn op_not(stack: &mut Stack) -> bool {
    if stack.is_empty() { return false; }

    let n = decode_num(&stack.pop().unwrap());
    if n == 0 {
        stack.push(encode_num(1));
    } else {
        stack.push(encode_num(0));
    }
    true
}

/// op_code : 146
/// not 의 반대 개념
/// 최상위 element 값이  0  -> 0
///                   그외  -> 1
pub fn op_0notequal(stack: &mut Stack) -> bool {
    if stack.is_empty() { return false; }

    let n = decode_num(&stack.pop().unwrap());
    if n == 0 {
        stack.push(encode_num(0));
    } else {
        stack.push(encode_num(1));
    }
    true
}

/// op_code : 147
/// /// 최상위 element pop() + 두번째 element pop()
pub fn op_add(stack: &mut Stack) -> bool {
    if stack.len() < 2 { return false; }

    let n1 = decode_num(&stack.pop().unwrap());
    let n2 = decode_num(&stack.pop().unwrap());

    stack.push(encode_num(n1 + n2));
    true
}

/// op_code : 148
/// 최상위 element pop() - 두번째 element pop()
pub fn op_sub(stack: &mut Stack) -> bool {
    if stack.len() < 2 { return false; }

    let n1 = decode_num(&stack.pop().unwrap());
    let n2 = decode_num(&stack.pop().unwrap());

    stack.push(encode_num(n1 - n2));
    true
}

/// op_code : 149
/// 최상위 element pop() * 두번째 element pop()
pub fn op_mul(stack: &mut Stack) -> bool {
    if stack.len() < 2 { return false; }

    let n1 = decode_num(&stack.pop().unwrap());
    let n2 = decode_num(&stack.pop().unwrap());

    stack.push(encode_num(n1 * n2));
    true
}

/// op_code : 154
/// 최상위 element pop()  두번째 element pop()
/// 둘다 참(0이 아닌)  이면 참(1) , 아니면 거짓(0) stack 에 push
pub fn op_booland(stack: &mut Stack) -> bool {
    if stack.len() < 2 { return false; }

    let n1 = decode_num(&stack.pop().unwrap());
    let n2 = decode_num(&stack.pop().unwrap());

    if n1 != 0 && n2 != 0 {
        stack.push(encode_num(1));
    } else {
        stack.push(encode_num(0));
    }
    true
}

/// op_code : 155
/// 최상위 element pop()  두번째 element pop()
/// 둘 하나라도 참(0이 아닌) 이면 참(1), 아니면 거짓(0) stack 에 push
pub fn op_boolor(stack: &mut Stack) -> bool {
    if stack.len() < 2 { return false; }

    let n1 = decode_num(&stack.pop().unwrap());
    let n2 = decode_num(&stack.pop().unwrap());

    if n1 != 0 || n2 != 0 {
        stack.push(encode_num(1));
    } else {
        stack.push(encode_num(0));
    }
    true
}

/// op_code : 156
/// 최상위 element pop()  두번째 element pop()
/// 두 수가 같으면 참(1), 아니면 거짓(0) 을 stack 에 push 
pub fn op_numequal(stack: &mut Stack) -> bool {
    if stack.len() < 2 { return false; }

    let n1 = decode_num(&stack.pop().unwrap());
    let n2 = decode_num(&stack.pop().unwrap());

    if n1 == n2 {
        stack.push(encode_num(1));
    } else {
        stack.push(encode_num(0));
    }
    true
}

/// op_code : 157
/// 최상위 element pop()  두번째 element pop()
/// 두 수가 같으면 참(1), 아니면 거짓(0)  return  (stack push X)
/// false 가 ruturn 되는 경우 script 진행 중지
pub fn op_numequalverify(stack: &mut Stack) -> bool {
    if stack.len() < 2 { return false; }

    let n1 = decode_num(&stack.pop().unwrap());
    let n2 = decode_num(&stack.pop().unwrap());

    if n1 != n2 {
        return false;
    } 
    true
}

/// op_code : 158
/// op_numequal 의 반대
/// 두 수가 같으면 거짓(0), 아니면 참(1) 을 stack 에 push 
pub fn op_numnotequal(stack: &mut Stack) -> bool {
    if stack.len() < 2 { return false; }

    let n1 = decode_num(&stack.pop().unwrap());
    let n2 = decode_num(&stack.pop().unwrap());

    if n1 == n2 {
        stack.push(encode_num(0));
    } else {
        stack.push(encode_num(1));
    }
    true
}

/// op_code : 159
/// 최상위 < 차상위
pub fn op_lessthan(stack: &mut Stack) -> bool {
    if stack.len() < 2 { return false; }
    
    let n1 = decode_num(&stack.pop().unwrap());
    let n2 = decode_num(&stack.pop().unwrap());

    if n1 < n2 {
        stack.push(encode_num(1));
    } else {
        stack.push(encode_num(0));
    }
    true
}

/// op_code : 160
/// 최상위 > 차상위
pub fn op_greaterthan(stack: &mut Stack) -> bool {
    if stack.len() < 2 { return false; }
    
    let n1 = decode_num(&stack.pop().unwrap());
    let n2 = decode_num(&stack.pop().unwrap());

    if n1 > n2 {
        stack.push(encode_num(1));
    } else {
        stack.push(encode_num(0));
    }
    true
}

/// op_code : 161
/// 최상위 <= 차상위
pub fn op_lessthanorequal(stack: &mut Stack) -> bool {
    if stack.len() < 2 { return false; }
    
    let n1 = decode_num(&stack.pop().unwrap());
    let n2 = decode_num(&stack.pop().unwrap());

    if n1 <= n2 {
        stack.push(encode_num(1));
    } else {
        stack.push(encode_num(0));
    }
    true
}

/// op_code : 162
/// 최상위 >= 차상위
pub fn op_greaterthanorequal(stack: &mut Stack) -> bool {
    if stack.len() < 2 { return false; }
    
    let n1 = decode_num(&stack.pop().unwrap());
    let n2 = decode_num(&stack.pop().unwrap());

    if n1 >= n2 {
        stack.push(encode_num(1));
    } else {
        stack.push(encode_num(0));
    }
    true
}

/// op_code : 163
pub fn op_min(stack: &mut Stack) -> bool {
    if stack.len() < 2 { return false; }
        
    let n1 = decode_num(&stack.pop().unwrap());
    let n2 = decode_num(&stack.pop().unwrap());

    if n1 < n2 {
        stack.push(encode_num(n1));
    } else {
        stack.push(encode_num(n2));
    }
    true
}

/// op_code : 164
pub fn op_max(stack: &mut Stack) -> bool {
    if stack.len() < 2 { return false; }
        
    let n1 = decode_num(&stack.pop().unwrap());
    let n2 = decode_num(&stack.pop().unwrap());

    if n1 > n2 {
        stack.push(encode_num(n1));
    } else {
        stack.push(encode_num(n2));
    }
    true
}

/// op_code : 165
/// stack vec![... max, min, x] 의 역순으로 꺼내서 
/// min <= x < max  이 성립하면 1, 아니면 0
pub fn op_within(stack: &mut Stack) -> bool {
    if stack.len() < 3 { return false; }
            
    let x = decode_num(&stack.pop().unwrap());
    let min = decode_num(&stack.pop().unwrap());
    let max = decode_num(&stack.pop().unwrap());

    if min <= x && x < max  {
        stack.push(encode_num(1));
    } else {
        stack.push(encode_num(0));
    }
    true
}

/// op_code : 166
/// RIPEMD-160은 "RACE Integrity Primitives Evaluation Message Digest"
/// RIPEMD-160은 160비트 (20바이트) 길이의 해시를 생성
///  Bitcoin 주소는 공개키를 SHA-256으로 해싱한 다음, 
/// 그 결과를 다시 RIPEMD-160으로 해싱하여 생성
pub fn op_ripemd160(stack: &mut Stack) -> bool {
    if stack.is_empty() { return false; }

    if let Some(element) = stack.pop() {
        let mut hasher = Ripemd160::new();
        hasher.update(element);
        let hash = hasher.finalize();
        
        stack.push(hash.to_vec());    
    };
    true
}

/// op_code : 167
pub fn op_sha1(stack: &mut Stack) -> bool {
    if stack.is_empty() { return false; }

    if let Some(element) = stack.pop() {
        let mut hasher = Sha1::new();
        hasher.update(element);
        let hash = hasher.finalize();
    
        stack.push(hash.to_vec());
    };
    true
}

/// op_code : 168
pub fn op_sha256(stack: &mut Stack) -> bool {
    if stack.is_empty() { return false; }

    if let Some(element) = stack.pop() {
        let mut hasher = Sha256::new();
        hasher.update(element);
        let hash = hasher.finalize();
    
        stack.push(hash.to_vec());
    };
    true
}

/// op_code : 169
/// sha256 + ripemd160
pub fn op_hash160(stack: &mut Stack) -> bool {
    if stack.is_empty() { return false; }

    if let Some(element) = stack.pop() {
        let mut hasher = Sha256::new();
        hasher.update(element);
        
        let mut ripemd = Ripemd160::new();

        // hasher.finalize_reset() 
        // 현재까지 hashing 된 data 에 댛나 최종 hash 값을 계산하고, hash 상태를 reset
        // 이렇게 생성된 hash 값은 다시 hasher.update() 의 parameter 전달되어
        // 다음 round 에 hashing 진행
        ripemd.update(hasher.finalize_reset());
        let hash = ripemd.finalize();

        stack.push(hash.to_vec());
    }
    true
}

/// op_code_function: 170 
/// sha256 을 2번 진행 => "double hashing"
/// Bitcoin 의 transaction hash 와 block hash 에서 사용되는 방식으로
/// 더 높은 수준의 보안을 제공
pub fn po_hash256(stack: &mut Stack) -> bool {
    if stack.is_empty() { return false; }

    if let Some(element) = stack.pop() {
        let mut hasher = Sha256::new();
        hasher.update(element);
        hasher.update(hasher.finalize_reset());
            
        stack.push(hasher.finalize().to_vec());
    }
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