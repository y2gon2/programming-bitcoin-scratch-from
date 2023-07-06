use std::fmt::Display;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use std::io::{stdin, Read};

use crate::models::helper::*;

#[derive(Hash)]
pub struct Tx {
    version: u32,   // transaction version
    tx_ins: Option<Vec<u32>>,    // 사용할 bitcoin 정의
    tx_outs: Option<Vec<u32>>,   // bitcoin 이 전달될 목적지
    locktime: Option<u32>,  // tranaction 유효 시점
    testnet: bool,   // transaction 을 검증하기 위해서 어떤 Network (ex. testnet, mainnet 등) 에서 발생했는지 명
}

impl Tx {

    pub fn new(
        version: u32, 
        tx_ins: Option<Vec<u32>>, 
        tx_outs: Option<Vec<u32>>, 
        locktime: Option<u32>, 
    ) -> Self {
        Self {
            version,
            tx_ins,
            tx_outs,
            locktime,
            testnet: Tx::set_testnet_default(),
        }
    }


    fn set_testnet_default() -> bool {
        false
    }
    // transaction 자체를 hashing 함. 
    pub fn id(&self) -> u64 {
        let mut s = DefaultHasher::new();
        self.hash(&mut s);
        s.finish() 
    }

    // version - serialized bytes [u8; 4] array 를 입력받으면 
    // 이를 little endian 으로 parsing -> u32 로 변환하여 이를 version value 로 사용하는
    // Tx instance 생성
    pub fn parse() {
        let mut buf = String::new();
        let _ = stdin().lock().read_to_string(&mut buf);

        
    }
 }

impl Display for Tx {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tx_ins_string = self.tx_ins
            .clone()
            .unwrap()
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>()
            .join("->");

        let tx_outs_string = self.tx_outs
            .clone()
            .unwrap()
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>()
            .join("->");

        write!(
            f,
            "tx: {}\nversion: {}\ntx_ins:\n{}tx_outs:\n{}locktime: {}",
            self.id(), 
            self.version,
            tx_ins_string,
            tx_outs_string,
            self.locktime.unwrap(),
        )
    }
}