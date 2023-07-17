use std::ops::{Deref, DerefMut};
use sha1::Sha1;
use sha2::{Sha256, Digest};
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
    if n < 0 {
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

/// op_code : 171
/// 생략 (현재 단계에서 구현 어려움)
/// script 내에서 현재 위치 이후의 연산자들만 고려하는 특별한 체크포인트를 설정
/// 이 연산자는 transaction의 서명을 검증하는 과정에서 사용
/// script 가 이 연산자를 만나면, 이 연산자 이후의 부분만이 서명 hash의 생성에 사용됩니다.
// pub fn op_codeseparator(stack: &mut Stack) -> bool {
    
//     true
// }


/// op_code : 172
/// stack 최상단 2 개의 element 를 각각 공개키와 서명으로 사용하여 taansaction 서명을 검증
/// 서명이 공개키와 일치하는지를 확인 후 , 검증 결과(true: 1, flase: 00)를 stack 에 다시 push 
pub fn op_checksig(stack: &mut Stack) -> bool {
    // to do
    true
}

/// op_code : 173
/// checksig 와 유사하지만, 검증에 실패할 경우 return false 로 실행 중단 
/// 성공할 경우 true (정상 실행 유지) 반환 (stack push X)
pub fn op_checksigverify(stack: &mut Stack) -> bool {
    // to do
    true
}

/// op_code : 174
/// 다수의 서명(multisig)를 검증
/// stack 가장 위의 element은 공개키 개수를, 그 아래 element 는 서명의 개수
/// 모든 서명이 검증에 성공하면 1(true), 그렇지 않으면 0(false) 을 stack 에 push
pub fn op_checkmultisig(stack: &mut Stack) -> bool {
    // to do
    true
}

/// op_code : 175
/// op_checkmultisig 에서 stack push 없이 검증이 실패하면 false 를 return 하여 실행 중단
pub fn op_checkmultisigverify(stack: &mut Stack) -> bool {
    // to do
    true
}

/// op_code : 177
/// OP_CHECKLOCKTIMEVERIFY (CLTV) 특정 시간 (locktime) 이후 bitcoin 사용할 수 있게 함. 
/// 추후 코드 검증 필요
pub fn op_checklocktimeverify(stack: &mut Stack, locktime: u32, sequence: u32) -> bool {
    if sequence == 0xffffffff { return false; }
    if stack.is_empty() { return false; }

    if let Some(element) = stack.last() {
        let n = decode_num(element);

        if n < 0 { return false; }

        // 500,000,000 의 의미
        // locktime <= 500,000,000 이면 block 높이
        // locktime < 500,000,000 이면 Unix Epoch Time 으로 해석됨,
        // stack 에서 가져온 값이 앞의 locktime 의 해석 조건과 동일해야 하므로 
        // 아래 조건이 충족되면 false return 
        //
        // 500,000,000 초는 1970년 00:00:00 으로 부터 약 15.85년 으로 
        // 이값을 초과하는 locktime 은 Unix 시간으로 해석되며
        // 그 이하는 block 의 높이로 해석되도록 Bitcoin protocol 상 설계되어 있음.
        if n < 500_000_000 && locktime > 500_000_000 { return false; }

        // 해당 조건은 아직 transaction 이 실행 될 시점 또는 블록 높이에 도달하지 않았기 때문에
        // 사용될 수 없음을 만드는 코드 조각.
        if locktime < n as u32 { return false; }
    }
    true
}

/// op_code : 178
/// OP_CHECKSEQUENCEVERIFY (CSV) 는 sequence 번호가 지나야 bitcoin 을 사용할 수 있게 함.
/// (해당 transaction output 을 해당 시간 기간동안 잠그는데 사용) 
/// 추후 코드 검증 필요
pub fn op_checksequenceverify(stack: &mut Stack, version: u32, sequence: u32) -> bool {
    
    // sequence field 최상위 bit 가 1 인 경우 기존의 상대적인 시간 잠금
    // (block 이 채굴된 이후 경과된 시간)이 무시되고,
    // 전통적인 nLockTime (특정 block 높이 또는 시간) 에 의해 잠겨짐 
    // 이는 Bitcoin protocol 상 설정되어 있음
    if sequence & (1 << 31) == (1 << 31) { return false; }
    if stack.is_empty() { return false; }

    if let Some(element) = stack.last() {
        let n = decode_num(element);

        if n < 0 { return false; }
        let n_u32 = n as u32;

        if n_u32 & (1 << 31) == (1 << 31) { 

            // 상대적인 locktime 설정은 version 2 이후부터 사용 가능하므로
            // 이것은 Bitcoin Improvement Proposal (BIP) 68과 112에 의해 도입된 변경 사항임. 
            // BIP 68은 nSequence 필드에 상대적인 블록 높이 또는 시간을 지정할 수 있게 하였고, 
            // BIP 112는 OP_CHECKSEQUENCEVERIFY (CSV)를 도입하여 이를 활용할 수 있게 하였음. 
            // 이들 BIP는 Bitcoin의 트랜잭션 버전 2에서 도입됨.
            if version < 2 { 
                return false; 
                
            // 앞에서 최상의 bit 에 대한 검사 처리를 했는데, 여기서 또 해야 하는지..
            // 확인 가능 자료가 없어서 그대로 둠
            } else if sequence & (1 << 31) == (1 << 31) {
                return false;

            // BIP 68에서 도입된 상대적인 잠금 시간 기능은 nSequence 필드의 비트를 
            // 특정한 방식으로 해석하도록 변경하였으며, 이는 nSequence 필드의 22번째 비트는 
            // 이 필드의 나머지 부분이 블록 높이를 기준으로 한 상대적인 잠금 시간을 나타내는지 
            // (22번째 비트가 0인 경우), 아니면 초 단위로 표현된 시간을 기준으로 한 
            // 상대적인 잠금 시간을 나타내는지 (22번째 비트가 1인 경우)를 결정.    
            } else if n_u32 & (1 << 22) != sequence & (1 << 22) {
                return false;
            } else if n_u32 & 0xffff > sequence &  0xffff {
                return false;
            }
        }
    }

    true
}

// OP_FUNCTION  구현 현황

    // 0: 'OP_0',               작성 완료
    // 76: 'OP_PUSHDATA1',      미작성
    // 77: 'OP_PUSHDATA2',      미작성
    // 78: 'OP_PUSHDATA4',      미작성
    // 79: 'OP_1NEGATE',        작성 완료
    // 81: 'OP_1',              작성 완료
    // 82: 'OP_2',              작성 완료
    // 83: 'OP_3',              작성 완료
    // 84: 'OP_4',              작성 완료
    // 85: 'OP_5',              작성 완료
    // 86: 'OP_6',              작성 완료
    // 87: 'OP_7',              작성 완료
    // 88: 'OP_8',              작성 완료
    // 89: 'OP_9',              작성 완료
    // 90: 'OP_10',             작성 완료
    // 91: 'OP_11',             작성 완료
    // 92: 'OP_12',             작성 완료
    // 93: 'OP_13',             작성 완료
    // 94: 'OP_14',             작성 완료
    // 95: 'OP_15',             작성 완료
    // 96: 'OP_16',             작성 완료
    // 97: 'OP_NOP',            작성 완료
    // 99: 'OP_IF',             작성 완료
    // 100: 'OP_NOTIF',         작성 완료
    // 103: 'OP_ELSE',          미작성
    // 104: 'OP_ENDIF',         미작성
    // 105: 'OP_VERIFY',        작성 완료
    // 106: 'OP_RETURN',        작성 완료
    // 107: 'OP_TOALTSTACK',    작성 완료
    // 108: 'OP_FROMALTSTACK',  작성 완료
    // 109: 'OP_2DROP',         작성 완료
    // 110: 'OP_2DUP',          작성 완료
    // 111: 'OP_3DUP',          작성 완료
    // 112: 'OP_2OVER',         작성 완료
    // 113: 'OP_2ROT',          작성 완료
    // 114: 'OP_2SWAP',         작성 완료
    // 115: 'OP_IFDUP',         작성 완료
    // 116: 'OP_DEPTH',         작성 완료
    // 117: 'OP_DROP',          작성 완료
    // 118: 'OP_DUP',	        작성 완료
    // 119: 'OP_NIP',           작성 완료
    // 120: 'OP_OVER',          작성 완료
    // 121: 'OP_PICK',          작성 완료
    // 122: 'OP_ROLL',          작성 완료
    // 123: 'OP_ROT',           작성 완료
    // 124: 'OP_SWAP',          작성 완료
    // 125: 'OP_TUCK',          작성 완료
    // 130: 'OP_SIZE',          작성 완료
    // 135: 'OP_EQUAL',         작성 완료
    // 136: 'OP_EQUALVERIFY',   작성 완료
    // 139: 'OP_1ADD',          작성 완료
    // 140: 'OP_1SUB',          작성 완료
    // 143: 'OP_NEGATE',        작성 완료
    // 144: 'OP_ABS',           작성 완료
    // 145: 'OP_NOT',           작성 완료
    // 146: 'OP_0NOTEQUAL',     작성 완료
    // 147: 'OP_ADD',           작성 완료
    // 148: 'OP_SUB',           작성 완료
    // 149: 'OP_MUL',           작성 완료
    // 154: 'OP_BOOLAND',       작성 완료
    // 155: 'OP_BOOLOR',        작성 완료
    // 156: 'OP_NUMEQUAL',      작성 완료
    // 157: 'OP_NUMEQUALVERIFY',작성 완료
    // 158: 'OP_NUMNOTEQUAL',   작성 완료
    // 159: 'OP_LESSTHAN',      작성 완료
    // 160: 'OP_GREATERTHAN',   작성 완료
    // 161: 'OP_LESSTHANOREQUAL',   작성 완료
    // 162: 'OP_GREATERTHANOREQUAL',작성 완료
    // 163: 'OP_MIN',           작성 완료
    // 164: 'OP_MAX',           작성 완료
    // 165: 'OP_WITHIN',        작성 완료
    // 166: 'OP_RIPEMD160',     작성 완료
    // 167: 'OP_SHA1',          작성 완료
    // 168: 'OP_SHA256',        작성 완료
    // 169: 'OP_HASH160',       작성 완료
    // 170: 'OP_HASH256',	    작성 완료
    // 171: 'OP_CODESEPARATOR', 미작성
    // 172: 'OP_CHECKSIG',      미작성
    // 173: 'OP_CHECKSIGVERIFY',미작성
    // 174: 'OP_CHECKMULTISIG', 미작성
    // 175: 'OP_CHECKMULTISIGVERIFY', 미작성
    // 176: 'OP_NOP1',          미작성
    // 177: 'OP_CHECKLOCKTIMEVERIFY',  작성완료(검증필요)
    // 178: 'OP_CHECKSEQUENCEVERIFY',  작성완료(검증필요)
    // 179: 'OP_NOP4',          미작성
    // 180: 'OP_NOP5',          미작성
    // 181: 'OP_NOP6',          미작성
    // 182: 'OP_NOP7',          미작성
    // 183: 'OP_NOP8',          미작성
    // 184: 'OP_NOP9',          미작성
    // 185: 'OP_NOP10',         미작성


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