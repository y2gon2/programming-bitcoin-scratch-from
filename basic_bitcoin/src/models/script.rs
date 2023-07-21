use std::io::{Read, Write};
use std::error::Error;
use std::fmt::Display;
use std::ops::Add;

use byteorder::{ByteOrder, LittleEndian, WriteBytesExt};
use log::info;

use crate::models::helper::*;
use crate::models::op::*;



#[derive(Debug, Clone)]
pub enum Cmd {
    OpCode(u8),        
    BytesData(Vec<u8>),
}

#[derive(Debug, Clone)]
pub struct Script {
    cmds: Vec<Cmd>,
}

impl Script {
    pub fn new(cmds: Option<Vec<Cmd>>) -> Self {
        match cmds {
            Some(c) => Self { cmds: c },
            None => Self { cmds: Vec::<Cmd>::new() },
        }
    }

    /// helper fucntion 에서 address script 에 대해 checksum 을 확인한
    /// payload (20bytes) 를 받아서 잠금 script 로 번환하는  method
    pub fn p2pkh_script(h160: Vec<u8>) -> Self {
        Self {
            cmds: vec![
                Cmd::OpCode(0x76),      // OP_DUP
                Cmd::OpCode(0xa9),      // OP_HASH160
                Cmd::BytesData(h160),   // valid payload 
                Cmd::OpCode(0x88),      // OP_EQUALVERIFY
                Cmd::OpCode(0xac)       // OP_CHECKSIG
            ],
        }        
    }

    pub fn parse<R: Read>(reader: &mut R) -> Result<Self, Box<dyn Error>> {
        let length = read_varint(reader)?;
        let mut cmds = Vec::<Cmd>::new();
        let mut count = 0usize;

        while count < length as usize {
            let mut current = [0u8; 1];
            reader.read_exact(&mut current)?;
            count += 1;

            let cur_byte = current[0];

            // 1 ~ 75 : 해당 길이만큼 읽어와서 stack 에 추가
            if cur_byte >= 1 && cur_byte <= 75 {
                let buf_len = cur_byte as usize;
                let mut cmd_buf = vec![0u8; buf_len];
                
                reader.read_exact(&mut cmd_buf)?;
                cmds.push(Cmd::BytesData(cmd_buf));

                count += buf_len;

            // 76 : OP_Push data1
            // 한번에 읽어야 할 element 의 길이가 76 ~ 255bytes 를 넘어갈 경우 해당 OP no. 사용
            // 1 byte 를 읽어와서 parsing 하여 해당 길이만큼의 data 를 읽어와서 stack 에 추가
            } else if cur_byte == 76 {
                let mut buf_76 = [0u8; 1];
                reader.read_exact(&mut buf_76)?;
                
                let data_length = LittleEndian::read_u64(&buf_76);
                let mut buf_data = vec![0u8; data_length as usize];
                reader.read_exact(&mut buf_data)?;
                
                cmds.push(Cmd::BytesData(buf_data));
                count += 1;

            // 77 : OP_push data2 
            // 한번에 읽어야 할 element 의 길이가 255bytes 를 넘어갈 경우 해당 OP no. 사용
            // 2 bytes 를 읽어와서 parsing 후 해당 길이만큼의 data 를 읽어와서 stack 에 추가
            } else if current[0] == 77 {
                let mut buf_77 = [0u8; 2];
                reader.read_exact(&mut buf_77)?;

                let data_length = LittleEndian::read_u64(&mut buf_77);
                let mut buf_data = vec![0u8; data_length as usize];
                reader.read_exact(&mut buf_data)?;

                cmds.push(Cmd::BytesData(buf_data));
                count += 2;

            // 78 이상은 자체 OP-code 이므로 추가로 읽을 필요없이 해당 1byte 만큼만 stack 에 추가
            } else {
                cmds.push(Cmd::OpCode(cur_byte));
            }
        } 
        if count != length as usize {
            return Err("pasing script failed".into())
        }

        Ok(Self { cmds })
    }

    // Script 에 저장된 OP-code, Data 길이를 저장 
    fn raw_serialize(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut result = Vec::<u8>::new();

        for cmd in &self.cmds {
            match cmd {
                Cmd::OpCode(int_val) => { result.write_u8(*int_val).unwrap() },
                Cmd::BytesData(data_val) => {
                    let length = data_val.len();

                    // 1 ~ 75 bytes 범위의 data 길이 == 1byte 값 이므로
                    if length < 75 {
                        result.write_u8(length as u8).unwrap();

                    // 76 ~ 255 bytes 범위의 data 길이 
                    // 1st byte : 76  2dn byte : 해당 data 길이
                    } else if length > 75 && length < 0x100 { 
                        result.write_u8(76 as u8).unwrap();
                        result.write_u8(length as u8).unwrap();

                    // 256 ~520 bytes 범위의 data 길이
                    // 1st byte : 77, 2 ~3 bytes: 해다 data 길이
                    } else if length >= 0x00 && length <= 520 {
                        result.write_u8(77 as u8).unwrap();
                        result.write_u16::<LittleEndian>(length as u16).unwrap();
                    } else {
                        return Err("cmd too long".into())
                    }
                    // 실제 data 넣음
                    result.write_all(data_val).unwrap();
                },
            }
        }
        Ok(result)
    }

    pub fn serialize(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut result = self.raw_serialize()?;
        let total = result.len() as u32;
        let mut encoded = encode_varint(total)?;
        encoded.append(&mut result);

        Ok(encoded)
    }

    pub fn evaluate(&self, z: u8) -> bool {
        let mut cmds = self.cmds.clone();
        let mut stack = Stack::new();
        let mut altstack = Stack::new();

        // element 에 대한 정보 확인 불가!!! 
        // dummy element, version, sequence, locktime 생성 후 대입
        let mut element = Vec::<u8>::new(); 
        let version = 2u32;
        let sequence = 1_000_000u32;
        let locktime = 1_000_000u32;

        while let Some(cmd) = cmds.pop() {
            match cmd {
                Cmd::BytesData(data) => stack.push(data),
                Cmd::OpCode(code) => {
                    let function_name = OP_CODE_NAMES[code as usize];
                    match code {
                        0 => { 
                            if stack.op_0() {
                                info!("bad op : {}", function_name);
                                return false;    
                            }
                        },
                        79 => { 
                            if stack.op_1negate() {
                                info!("bad op : {}", function_name);
                                return false;    
                            }
                        },
                        81 => { 
                            if stack.op_1() {
                                info!("bad op : {}", function_name);
                                return false;    
                            }
                        },
                        82 => { 
                            if stack.op_2() {
                                info!("bad op : {}", function_name);
                                return false;    
                            }
                        },
                        83 => { 
                            if stack.op_3() {
                                info!("bad op : {}", function_name);
                                return false;    
                            }
                        },
                        84 => { 
                            if stack.op_4() {
                                info!("bad op : {}", function_name);
                                return false;    
                            }
                        },
                        85 => { 
                            if stack.op_5() {
                                info!("bad op : {}", function_name);
                                return false;    
                            }
                        },
                        86 => { 
                            if stack.op_6() {
                                info!("bad op : {}", function_name);
                                return false;    
                            }
                        },
                        87 => { 
                            if stack.op_7() {
                                info!("bad op : {}", function_name);
                                return false;    
                            }
                        },
                        88 => { 
                            if stack.op_8() {
                                info!("bad op : {}", function_name);
                                return false;    
                            }
                        },
                        89 => { 
                            if stack.op_9() {
                                info!("bad op : {}", function_name);
                                return false;    
                            }
                        },
                        90 => { 
                            if stack.op_10() {
                                info!("bad op : {}", function_name);
                                return false;    
                            }
                        },
                        91 => { 
                            if stack.op_11() {
                                info!("bad op : {}", function_name);
                                return false;    
                            }
                        },
                        92 => { 
                            if stack.op_12() {
                                info!("bad op : {}", function_name);
                                return false;    
                            }
                        },
                        93 => { 
                            if stack.op_13() {
                                info!("bad op : {}", function_name);
                                return false;    
                            }
                        },
                        94 => { 
                            if stack.op_14() {
                                info!("bad op : {}", function_name);
                                return false;    
                            }
                        },
                        95 => { 
                            if stack.op_15() {
                                info!("bad op : {}", function_name);
                                return false;    
                            }
                        },
                        96 => { 
                            if stack.op_16() {
                                info!("bad op : {}", function_name);
                                return false;    
                            }
                        },
                        97 => { 
                            if stack.op_nop() {
                                info!("bad op : {}", function_name);
                                return false;    
                            }
                        },
                        99 => { 
                            if stack.op_if(&mut element) {
                                info!("bad op : {}", function_name);
                                return false;    
                            }
                        },
                        100 => { 
                            if stack.op_notif(&mut element) {
                                info!("bad op : {}", function_name);
                                return false;    
                            }
                        },
                        105 => { 
                            if stack.op_verify() {
                                info!("bad op : {}", function_name);
                                return false;    
                            }
                        },
                        106=> { 
                            if stack.op_return() {
                                info!("bad op : {}", function_name);
                                return false;    
                            }
                        },
                        107 => { 
                            if stack.op_toaltstack(&mut altstack) {
                                info!("bad op : {}", function_name);
                                return false;    
                            }
                        },
                        108 => { 
                            if stack.op_fromaltstack(&mut altstack) {
                                info!("bad op : {}", function_name);
                                return false;    
                            }
                        },
                        109 => { 
                            if stack.op_2drop() {
                                info!("bad op : {}", function_name);
                                return false;    
                            }
                        },
                        110 => { 
                            if stack.op_2dup() {
                                info!("bad op : {}", function_name);
                                return false;    
                            }
                        },
                        111 => { 
                            if stack.op_3dup() {
                                info!("bad op : {}", function_name);
                                return false;    
                            }
                        },
                        112 => { 
                            if stack.op_2over() {
                                info!("bad op : {}", function_name);
                                return false;    
                            }
                        },
                        113 => { 
                            if stack.op_2rot() {
                                info!("bad op : {}", function_name);
                                return false;    
                            }
                        },
                        114 => { 
                            if stack.op_2swap() {
                                info!("bad op : {}", function_name);
                                return false;    
                            }
                        },
                        115 => { 
                            if stack.op_ifdup() {
                                info!("bad op : {}", function_name);
                                return false;    
                            }
                        },
                        116 => { 
                            if stack.op_depth() {
                                info!("bad op : {}", function_name);
                                return false;    
                            }
                        },
                        117 => { 
                            if stack.op_drop() {
                                info!("bad op : {}", function_name);
                                return false;    
                            }
                        },
                        118 => { 
                            if stack.op_dup() {
                                info!("bad op : {}", function_name);
                                return false;    
                            }
                        },
                        119 => { 
                            if stack.op_nip() {
                                info!("bad op : {}", function_name);
                                return false;    
                            }
                        },
                        120 => { 
                            if stack.op_over() {
                                info!("bad op : {}", function_name);
                                return false;    
                            }
                        },
                        121 => { 
                            if stack.op_pick() {
                                info!("bad op : {}", function_name);
                                return false;    
                            }
                        },
                        122 => { 
                            if stack.op_roll() {
                                info!("bad op : {}", function_name);
                                return false;    
                            }
                        },
                        123 => { 
                            if stack.op_rot() {
                                info!("bad op : {}", function_name);
                                return false;    
                            }
                        },
                        124 => { 
                            if stack.op_swap() {
                                info!("bad op : {}", function_name);
                                return false;    
                            }
                        },
                        125 => { 
                            if stack.op_tuck() {
                                info!("bad op : {}", function_name);
                                return false;    
                            }
                        },
                        130 => { 
                            if stack.op_size() {
                                info!("bad op : {}", function_name);
                                return false;    
                            }
                        },
                        135 => { 
                            if stack.op_equal() {
                                info!("bad op : {}", function_name);
                                return false;    
                            }
                        },
                        136 => { 
                            if stack.op_equalverify() {
                                info!("bad op : {}", function_name);
                                return false;    
                            }
                        },
                        139 => { 
                            if stack.op_1add() {
                                info!("bad op : {}", function_name);
                                return false;    
                            }
                        },
                        140 => { 
                            if stack.op_1sub() {
                                info!("bad op : {}", function_name);
                                return false;    
                            }
                        },
                        143 => { 
                            if stack.op_negate() {
                                info!("bad op : {}", function_name);
                                return false;    
                            }
                        },
                        144 => { 
                            if stack.op_abs() {
                                info!("bad op : {}", function_name);
                                return false;    
                            }
                        },
                        145 => { 
                            if stack.op_not() {
                                info!("bad op : {}", function_name);
                                return false;    
                            }
                        },
                        146 => { 
                            if stack.op_0notequal() {
                                info!("bad op : {}", function_name);
                                return false;    
                            }
                        },
                        147 => { 
                            if stack.op_add() {
                                info!("bad op : {}", function_name);
                                return false;    
                            }
                        },
                        148 => { 
                            if stack.op_sub() {
                                info!("bad op : {}", function_name);
                                return false;    
                            }
                        },
                        149 => { 
                            if stack.op_mul() {
                                info!("bad op : {}", function_name);
                                return false;    
                            }
                        },
                        154 => { 
                            if stack.op_booland() {
                                info!("bad op : {}", function_name);
                                return false;    
                            }
                        },
                        155 => { 
                            if stack.op_boolor() {
                                info!("bad op : {}", function_name);
                                return false;    
                            }
                        },
                        156 => { 
                            if stack.op_numequal() {
                                info!("bad op : {}", function_name);
                                return false;    
                            }
                        },
                        157 => { 
                            if stack.op_numequalverify() {
                                info!("bad op : {}", function_name);
                                return false;    
                            }
                        },
                        158 => { 
                            if stack.op_numnotequal() {
                                info!("bad op : {}", function_name);
                                return false;    
                            }
                        },
                        159 => { 
                            if stack.op_lessthan() {
                                info!("bad op : {}", function_name);
                                return false;    
                            }
                        },
                        160 => { 
                            if stack.op_greaterthan() {
                                info!("bad op : {}", function_name);
                                return false;    
                            }
                        },
                        161 => { 
                            if stack.op_lessthanorequal() {
                                info!("bad op : {}", function_name);
                                return false;    
                            }
                        },
                        162 => { 
                            if stack.op_greaterthanorequal() {
                                info!("bad op : {}", function_name);
                                return false;    
                            }
                        },
                        163 => { 
                            if stack.op_max() {
                                info!("bad op : {}", function_name);
                                return false;    
                            }
                        },
                        164 => { 
                            if stack.op_min() {
                                info!("bad op : {}", function_name);
                                return false;    
                            }
                        },
                        165 => { 
                            if stack.op_within() {
                                info!("bad op : {}", function_name);
                                return false;    
                            }
                        },
                        166 => { 
                            if stack.op_ripemd160() {
                                info!("bad op : {}", function_name);
                                return false;    
                            }
                        },
                        167 => { 
                            if stack.op_sha1() {
                                info!("bad op : {}", function_name);
                                return false;    
                            }
                        },
                        168 => { 
                            if stack.op_sha256() {
                                info!("bad op : {}", function_name);
                                return false;    
                            }
                        },
                        169 => { 
                            if stack.op_hash160() {
                                info!("bad op : {}", function_name);
                                return false;    
                            }
                        },
                        170 => { 
                            if stack.op_hash256() {
                                info!("bad op : {}", function_name);
                                return false;    
                            }
                        },
                        172 => { 
                            if stack.op_checksig(z) {
                                info!("bad op : {}", function_name);
                                return false;    
                            }
                        },
                        173 => { 
                            if stack.op_checksigverify(z) {
                                info!("bad op : {}", function_name);
                                return false;    
                            }
                        },
                        174 => { 
                            if stack.op_checkmultisig(z) {
                                info!("bad op : {}", function_name);
                                return false;    
                            }
                        },
                        175 => { 
                            if stack.op_checkmultisigverify(z) {
                                info!("bad op : {}", function_name);
                                return false;    
                            }
                        },
                        177 => { 
                            if stack.op_checklocktimeverify(locktime, sequence) {
                                info!("bad op : {}", function_name);
                                return false;    
                            }
                        },
                        178 => { 
                            if stack.op_checksequenceverify(version, sequence) {
                                info!("bad op : {}", function_name);
                                return false;    
                            }
                        },
                        _ => info!("invalid op-code : {}", code),
                    }
                },
            }

            };
        if stack.is_empty() { return false; }
        if let Some(popped) = stack.pop() {
            if popped == b"" { return false; }
        }

        true
    }
}

impl Display for Script {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut messages = Vec::<String>::new();
        
        for cmd in self.cmds.iter() {
            match cmd {
                Cmd::OpCode(code) => {
                    let function_name = OP_CODE_NAMES[*code as usize];
                    if  function_name == "" { 
                        messages.push(format!("OP_[{}]", code)); 
                    } else {
                        messages.push(function_name.to_string());
                    }
                },
                Cmd::BytesData(data) => {
                    let hex_string: String = data.iter().map(|b| format!("{:02x}", b)).collect();
                    messages.push(hex_string);
                },
            } 
        }
        write!(f, "{}", messages.join(" "))
    }
}

/// Add trait implementation 이 필요한 이유
/// 
/// Bitcoin Scripting System 은 크게 다음 두가지 scripting 이 사용된다.
/// 1. Locking Script (or Script Pubkey)
///    transaction output 의 일부로, transacion output 을 '잠금'하는 역할을 한다.
///    이는 특정 조건을 만족하는 사람만이 해당 transaction output 을 사용할 수 이도록 한다.
/// 
/// 2. Unlocking Script (or ScriptSig)
///    Transaction input 의 일부로, 잠긴 transaction output 을 '해제' 하는 역할을 한다.
/// 
/// Bitcoin 에서 Transaction output 을 사용하려면, (즉 새 transaction input 을 사용하려면) 
/// 해당 transaction output 의 locking script 와 새로운 transaction 의 unlocking script 를 
/// 합쳐서 실행해야 한다. 
/// 이 두 script 가 합쳐진 후에는 하나의 완전한 script 처럼 동작하며, 이 script 가 성공적으로
/// 실행되면 transaction output 의 사용이 허가된다. 
/// 따라서 위 작업을 'Add' trait impl 로 단순하고 명시적으로 결합 작업이 진해도도록 만들어준다. 
impl Add for Script {
    type Output = Script;

    fn add(self, rhs: Self) -> Self::Output {
        let mut added = self.cmds.clone();
        added.extend(rhs.cmds);
        Script { cmds: added }        
    }
}



#[cfg(test)]
mod test_script {
    use super::*;

    #[test]
    fn t_struct () {
        let a = ["a", "b", "c"];
        let s = a.join(" ? ");
        println!("{}", s);
    }
}