use std::io::{Read, Write};
use std::error::Error;
use std::fmt::Display;

use byteorder::{ByteOrder, LittleEndian, WriteBytesExt};
use crate::models::helper::*;
use hex;

#[derive(Debug)]
enum Cmd {
    OpCode(u8),        
    BytesData(Vec<u8>),
}

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
}

impl Display for Script {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut messages = Vec::<String>::new();
        
        for cmd in self.cmds.iter() {
            match cmd {
                Cmd::OpCode(code) => {
                    let mut msg = "";
                    match code {
                        0 => msg = "OP_0",
                        // 76 => msg = "OP_PUSHDATA1", // 미구현된 연산자
                        // 77 => msg = "OP_PUSHDATA2", // 미구현된 연산자
                        // 78 => msg = "OP_PUSHDATA4", // 미구현된 연산자
                        79 => msg = "OP_1NEGATE",
                        81 => msg = "OP_1",
                        82 => msg = "OP_2",
                        83 => msg = "OP_3",
                        84 => msg = "OP_4",
                        85 => msg = "OP_5",
                        86 => msg = "OP_6",
                        87 => msg = "OP_7",
                        88 => msg = "OP_8",
                        89 => msg = "OP_9",
                        90 => msg = "OP_10",
                        91 => msg = "OP_11",
                        92 => msg = "OP_12",
                        93 => msg = "OP_13",
                        94 => msg = "OP_14",
                        95 => msg = "OP_15",
                        96 => msg = "OP_16",
                        97 => msg = "OP_NOP",
                        99 => msg = "OP_IF",
                        100 => msg = "OP_NOTIF",
                        // 103 => msg = "OP_ELSE",   // 미구현된 연산자
                        // 104 => msg = "OP_ENDIF",  // 미구현된 연산자
                        105 => msg = "OP_VERIFY",
                        106 => msg = "OP_RETURN",
                        107 => msg = "OP_TOALTSTACK",
                        108 => msg = "OP_FROMALTSTACK",
                        109 => msg = "OP_2DROP",
                        110 => msg = "OP_2DUP",
                        111 => msg = "OP_3DUP",
                        112 => msg = "OP_2OVER",
                        113 => msg = "OP_2ROT",
                        114 => msg = "OP_2SWAP",
                        115 => msg = "OP_IFDUP",
                        116 => msg = "OP_DEPTH",
                        117 => msg = "OP_DROP",
                        118 => msg = "OP_DUP",
                        119 => msg = "OP_NIP",
                        120 => msg = "OP_OVER",
                        121 => msg = "OP_PICK",
                        122 => msg = "OP_ROLL",
                        123 => msg = "OP_ROT",
                        124 => msg = "OP_SWAP",
                        125 => msg = "OP_TUCK",
                        130 => msg = "OP_SIZE",
                        135 => msg = "OP_EQUAL",
                        136 => msg = "OP_EQUALVERIFY",
                        139 => msg = "OP_1ADD",
                        140 => msg = "OP_1SUB",
                        143 => msg = "OP_NEGATE",
                        144 => msg = "'OP_ABS",
                        145 => msg = "OP_NOT",
                        146 => msg = "OP_0NOTEQUAL",
                        147 => msg = "OP_ADD",
                        148 => msg = "OP_SUB",
                        149 => msg = "OP_MUL",
                        154 => msg = "OP_BOOLAND",
                        155 => msg = "OP_BOOLOR",
                        156 => msg = "OP_NUMEQUAL",
                        157 => msg = "OP_NUMEQUALVERIFY",
                        158 => msg = "OP_NUMNOTEQUAL",
                        159 => msg = "OP_LESSTHAN",
                        160 => msg = "OP_GREATERTHAN",
                        161 => msg = "OP_LESSTHANOREQUAL",
                        162 => msg = "OP_GREATERTHANOREQUAL",
                        163 => msg = "OP_MIN",
                        164 => msg = "OP_MAX",
                        165 => msg = "OP_WITHIN",
                        166 => msg = "OP_RIPEMD160",
                        167 => msg = "OP_SHA1",
                        168 => msg = "OP_SHA256",
                        169 => msg = "OP_HASH160",
                        170 => msg = "OP_HASH256",
                        171 => msg = "OP_CODESEPARATOR",
                        // 172 => msg = "OP_CHECKSIG",               // 미구현된 연산자
                        // 173 => msg = "OP_CHECKSIGVERIFY",         // 미구현된 연산자
                        // 174 => msg = "OP_CHECKMULTISIG",          // 미구현된 연산자
                        // 175 => msg = "OP_CHECKMULTISIGVERIFY",    // 미구현된 연산자
                        // 176 => msg = "OP_NOP1",   // 미구현된 연산자
                        177 => msg = "OP_CHECKLOCKTIMEVERIFY",
                        178 => msg = "OP_CHECKSEQUENCEVERIFY",
                        // 179 => msg = "OP_NOP4",   // 미구현된 연산자
                        // 180 => msg = "OP_NOP5",   // 미구현된 연산자
                        // 181 => msg = "OP_NOP6",   // 미구현된 연산자
                        // 182 => msg = "OP_NOP7",   // 미구현된 연산자
                        // 183 => msg = "OP_NOP8",   // 미구현된 연산자
                        // 184 => msg = "OP_NOP9",   // 미구현된 연산자
                        // 185 => msg = "OP_NOP10",  // 미구현된 연산자
                        others => msg = format!("OP_[{}]", others).as_str(),
                    }
                    messages.push(msg.to_string());
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