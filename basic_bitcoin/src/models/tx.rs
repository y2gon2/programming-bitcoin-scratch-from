use std::error::Error;
use std::fmt::Display;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use std::io::{stdin, stdout, Read, Write};
use serde::ser::{Serialize, Serializer, SerializeStruct};
use sha2::{Sha256, Digest};
use hex;
use bincode;

use crate::models::helper::*;

//---------------------
//         Tx
//---------------------

type version = u32;
type locktime = u32;

#[derive(Hash, Debug, Clone)]
pub struct Tx {
    version: version,   // transaction version
    tx_ins: Option<Vec<TxIn>>,    // 사용할 bitcoin 정의
    tx_outs: Option<Vec<TxOut>>,   // bitcoin 이 전달될 목적지
    locktime: Option<locktime>,  // tranaction 유효 시점
    testnet: bool,   // transaction 을 검증하기 위해서 어떤 Network (ex. testnet, mainnet 등) 에서 발생했는지 명
}

impl Tx {

    pub fn new(
        version: version, 
        tx_ins: Option<Vec<TxIn>>, 
        tx_outs: Option<Vec<TxOut>>, 
        locktime: Option<locktime>, 
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
    pub fn id(&self) -> Result<String, Box<dyn Error>> {
        let id = self.hash()?;
        Ok(hex::encode(id))
    }

    // 
    fn hash(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut tx_serde = self.serialize()?;
        tx_serde.reverse();
        
        let mut hasher = Sha256::new();
        hasher.update(&tx_serde);
        
        Ok(hasher.finalize().to_vec())
    }

    // version - serialized bytes [u8; 4] array 를 입력받으면 
    // 이를 little endian 으로 parsing -> u32 로 변환하여 이를 version value 로 사용하는
    // Tx instance 생성
    pub fn parse<R: Read>(reader: &mut R, testnet: bool) -> Result<Self, Box<dyn Error>>{
        // Stream 에서 version(4 bytes, little-endian) 읽기
        let mut version_bytes = [0u8; 4];
        reader.read_exact(&mut version_bytes)?;
        let version = version::from_le_bytes(version_bytes);

        // Stream 에서  num_inputs (helper::varint) 읽기
        let num_inputs = read_varint(reader)?;

        // Parse num_inputs number of TxIns (검토 및 재작성) 
        // let mut inputs = Vec::with_capacity(num_inputs as usize);
        // for _ in 0..num_inputs {
        //     let input = TxIn::parse(reader)?;
        //     input.push(input);
        // }

        // Stream 에서 num_outputs (hepler::varint) 읽기
        let num_outputs = read_varint(reader)?;

        // Parse num_outputs number of TxIns (검토 및 재작성)
        // let output = Vec::with_capacity(num_outputs as usize);
        // for _ in 0..num_outputs {
        //      let output = TxOut::parse(reader)?;
        //      output.push(output);
        // }

        // Stream 에서 locktime (4 bytes, little-endian) 읽기
        let mut locktime_bytes = [0u8; 4];
        reader.read_exact(&mut locktime_bytes)?;
        let locktime = locktime::from_le_bytes(locktime_bytes);

        Ok(Self {
            version,
            tx_ins: Some(input),   
            tx_outs: Some(output),  
            locktime: Some(locktime), 
            testnet: false,
        })
    }

    fn serialize(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut result = Vec::<u8>::new();

        let mut version_serde = u32_to_little_endian(self.version, 4)?;
        result.append(&mut version_serde);

        let mut tx_ins_serde = Vec::<u8>::new();

        if let Some(internal_tx_ins) = self.tx_ins {
            let mut num_of_input = encode_varint(internal_tx_ins.len() as u32)?;
            tx_ins_serde.append(&mut num_of_input);

            for tx_in in internal_tx_ins.iter() {
                tx_ins_serde.append(&mut tx_in.serialize()?);
            }      
        };
        result.append(&mut tx_ins_serde);

        let mut tx_outs_serde = Vec::<u8>::new();

        if let Some(internal_tx_outs) = self.tx_outs {
            let mut num_of_output = encode_varint(internal_tx_outs.len() as u32)?;
            tx_outs_serde.append(&mut num_of_output);

            for tx_out in internal_tx_outs.iter() {
                tx_outs_serde.append(&mut tx_out.serialize()?);  // to do
            }
        };
        result.append(&mut tx_outs_serde);

        if let Some(internal_locktime) = self.locktime {
            result.append(&mut u32_to_little_endian(internal_locktime, 4)?);            
        };        

        Ok(result)
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
            self.id().unwrap(), 
            self.version,
            tx_ins_string,
            tx_outs_string,
            self.locktime.unwrap(),
        )
    }
}


//---------------------
//       TxIn
//---------------------

type PrevTx = [u8; 32];
type PrevIndex = u32;
type ScriptSig = Vec<u8>; 
type Sequence = u32;


#[derive(Hash, Debug, Clone)]
pub struct TxIn {
    prev_tx: PrevTx,
    // 이전 transaction identifier (TXID) 
    // transaction data hashing 으로 생성되어 TX 각각 고유한 값을 가짐.
    // 특정 TX 의 추적, 검색에 사용 가능

    prev_index: PrevIndex,
    // 이전 transaction 에서 여러 출력이 발생한 경우 이에 대한 output index
    // 예를 들면 Bob 이 Alice 와 Tom 에게 한번의 TX 에서 둘다 bitcoin  전송이 가능하며,
    // 이 때 각각의 구분하기 위한 index 가 필요하다. 
    // index 값은 0 부터 시작 한다. 
    
    script_sig: Option<ScriptSig>,
    // 이전 출력의 잠금 script 를 해제하기 위한 입력 script 
    // 일반적으로 송신자의 디지털 서명고 공개키를 포함하며, 이것으로 이전 출력이 
    // 해당 사용자에게 속하는 것을 증명하고, 그것을 사용할 권리가 있음을 보여줌
    //
    // "이전 출력의 잠금 script 를 해제" 작업이 필요한 이유
    // -> bitcoin 소유권을 보장하고 이중지불(double-spending)을 방지하기 위함
    //    
    // UTXO : Unspent Transaction Output (미사용 거래 출력)
    //  - 새로운 bitcoin TX 를 만들때, 이전 UTXO 를 입력으로 사용한다. 
    //    이때, 그 UTXO를 사용할 수 있는 권한을 증명해야 하는데, 
    //    이때 필요한 것이 바료 이전 출력의 잠금 script를 해제하는 것
    //    (나에게 입력된 TX 중 소비되지 않은 TX)

    sequence: Sequence,
    // bitcoin time-lock  기능을 지원하는데 사용
    // transaction 의 특정 block 높이 또는 시간이 될 때까지 승이되지 않도록 하는데 사용
    //
    // 최초 transaction 을 업데이트하거나 대체하기 위한 메카니즘을 제공하기 위해 도입
    // 그러나 bitcoin 초기 버젼에는 활성화 되어 있지 않음. 
}

impl TxIn {
    pub fn new(
        prev_tx: PrevTx, 
        prev_index: PrevIndex, 
        script_sig: Option<ScriptSig>, 
    ) -> Self {
        Self {
            prev_tx,
            prev_index,
            script_sig,
            sequence: 0xffffffff,
        }
    } 

    pub fn parse<R: Read>(reader: R) -> Result<Self, Box<dyn Error>> {
        
        // prev_tx (32bytes, little-endian)
        let mut prev_tx = [0u8; 32];
        reader.read_exact(&mut prev_tx)?;
        prev_tx.reverse();

        // prev_index : u32 from 4 bytes, little-endian
        let mut prev_index_bytes = [0u8; 4];
        reader.read_exact(&mut prev_index_bytes)?;
        
        let  prev_index = PrevIndex::from_le_bytes(prev_index_bytes);

        // let script_sig =  // to do

        let mut seq_bytes = [0u8; 4];
        reader.read_exact(&mut seq_bytes)?;

        let sequence = Sequence::from_le_bytes(seq_bytes);

        Ok(Self {
            prev_tx,
            prev_index,
            script_sig,
            sequence,
        })
    }

    fn serialize(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut result: Vec<u8> = self.prev_tx.to_vec();
        result.reverse();

        let mut prev_index_ser = u32_to_little_endian(self.prev_index, 4)?;
        // let mut script_sig_ser = self.script_sig.serialize();
        let mut sequence_ser = u32_to_little_endian(self.sequence, 4)?;

        result.append(&mut prev_index_ser);
        // result.append(&mut script_sig_ser);
        result.append(&mut sequence_ser);

        Ok(result)
    }
}

impl Display for TxIn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut hex = String::new();

        write!(
            f,
            "{}: {}",
            hex::encode(self.prev_tx), 
            self.prev_index,
        )
    }
}

//---------------------
//       TxOut
//---------------------

type Amount = u64;
type ScriptPubkey = Vec<u8>;

#[derive(Hash, Debug, Clone)]
pub struct TxOut {
    // Bitcoin 금액
    // 단위: Satoshi
    // 1 Bitcoin == 100_000_000 Satoshi
    amount: Amount,

    // 잠금 script
    // <-> 해제 script (script_sig) 
    // scirpt_sig 와 동일한 smart contract script 로 쓰임 (?)
    // varint 처럼 가변길이 필드로 시작
    script_pubkey: ScriptPubkey,
}

impl TxOut {
    pub fn new(amount: Amount, script_pubkey: ScriptPubkey) -> Self {
        Self {
            amount,
            script_pubkey,
        }
    }

    pub fn parse<R: Read>(reader: R) -> Result<Self, Box<dyn Error>> {
        let mut amount_bytes = [0u8; 8];
        reader.read_exact(&mut amount_bytes)?;
        
        let amount = little_endian_to_u64(&amount_bytes);

        // let script_pubkey = Script.parse(S);

        Ok(Self { amount, script_pubkey })
    }

    pub fn serialize(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut result = Vec::<u8>::new();

        let mut amount = self.amount.to_le_bytes();
        result.append(&mut amount.to_vec());

        // let mut script_pubkey = self.script_pubkey.serialize(); // to do
        // result.append(&mut amount.to_vec());

        Ok(result)
    }
}

impl Display for TxOut {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.amount, hex::encode(self.script_pubkey))
    }
}



//---------------------
//       test
//---------------------
#[cfg(test)]
mod tx_test {
    use super::*;
    use std::prelude::*;


    #[test]
    fn parsing_test() -> Result<(), Box<dyn Error>>{
        let t1 = Tx::parse()?;
        println!("{:?}", &t1);
        Ok(())
    }
}

