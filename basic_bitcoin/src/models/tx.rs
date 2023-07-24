use std::error::Error;
use std::fmt::Display;
use std::hash::Hash;
use std::collections::hash_map::HashMap;
use std::io::{Read, Cursor};
use sha2::{Sha256, Digest};
use hex;
use byteorder::{ByteOrder, LittleEndian};
use anyhow::Result;


use crate::models::helper::*;

use super::script::Script;

//---------------------
//         Tx
//---------------------

#[derive(Hash, Debug, Clone)]
pub struct Tx {
    version: u32,   // transaction version
    tx_ins: Option<Vec<TxIn>>,    // 사용할 bitcoin 정의
    tx_outs: Option<Vec<TxOut>>,   // bitcoin 이 전달될 목적지
    locktime: Option<u32>,  // tranaction 유효 시점
    testnet: bool,   // transaction 을 검증하기 위해서 어떤 Network (ex. testnet, mainnet 등) 에서 발생했는지 명
}

impl Tx {
    pub fn new(
        version: u32, 
        tx_ins: Option<Vec<TxIn>>, 
        tx_outs: Option<Vec<TxOut>>, 
        locktime: Option<u32>, 
        testnet: bool
    ) -> Self {
        Self {
            version,
            tx_ins,
            tx_outs,
            locktime,
            testnet,
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
    #[allow(unused_variables)]
    pub fn parse<R: Read>(reader: &mut R, testnet: bool) -> Result<Self, Box<dyn Error>>{
        
        // Stream 에서 version(4 bytes, little-endian) 읽기
        let mut version_bytes = [0u8; 4];
        reader.read_exact(&mut version_bytes)?;
        let version = u32::from_le_bytes(version_bytes);

        // Stream 에서  num_inputs (helper::varint) 읽기
        let num_inputs = read_varint(reader)?;

        // Parse num_inputs number of TxIns (검토 및 재작성) 
        let mut inputs = Vec::with_capacity(num_inputs as usize);
        for _ in 0..num_inputs {
            let input = TxIn::parse(reader)?;
            inputs.push(input);
        }

        // Stream 에서 num_outputs (hepler::varint) 읽기
        let num_outputs = read_varint(reader)?;

        // Parse num_outputs number of TxIns (검토 및 재작성)
        let mut outputs = Vec::with_capacity(num_outputs as usize);
        for _ in 0..num_outputs {
             let output = TxOut::parse(reader)?;
             outputs.push(output);
        }

        // Stream 에서 locktime (4 bytes, little-endian) 읽기
        let mut locktime_bytes = [0u8; 4];
        reader.read_exact(&mut locktime_bytes)?;
        let locktime = u32::from_le_bytes(locktime_bytes);

        Ok(Self {
            version,
            tx_ins: Some(inputs),   
            tx_outs: Some(outputs),  
            locktime: Some(locktime), 
            testnet: false,
        })
    }

    pub fn serialize(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut result = Vec::<u8>::new();

        let mut version_serde = u32_to_little_endian(self.version, 4)?;
        result.append(&mut version_serde);

        let mut tx_ins_serde = Vec::<u8>::new();

        if let Some(internal_tx_ins) = &self.tx_ins {
            let mut num_of_input = encode_varint(internal_tx_ins.len() as u32)?;
            tx_ins_serde.append(&mut num_of_input);

            for tx_in in internal_tx_ins.iter() {
                tx_ins_serde.append(&mut tx_in.serialize()?);
            }      
        };
        result.append(&mut tx_ins_serde);

        let mut tx_outs_serde = Vec::<u8>::new();

        if let Some(internal_tx_outs) = &self.tx_outs {
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

    /// Bitcoin 의 일반적 수수료 산정: 해당 거래의 입력 총합 - 출력 총합
    /// 해당 수수료는 채굴자에게 지급 (해당 내용은 구현 X)
    pub fn fee(&self, tx_fetcher: &mut TxFetcher) -> Result<u64, Box<dyn Error>> {
        let (mut input_sum, mut output_sum) = (0u64, 0u64);

        if let Some(tx_ins) = &self.tx_ins {
            for tx_in in tx_ins {
                input_sum += tx_in.value(tx_fetcher)?;
            }
        }

        if let Some(tx_outs) = &self.tx_outs {
            for tx_out in tx_outs {
                output_sum += tx_out.amount;
            }
        }

        Ok(input_sum - output_sum)
    }

    /// ****  TO DO  ****
    ///
    /// transaction 의 일부 또는 전체를 hashing 하고 그 hash value 를 상요하여
    /// transaction 의 유효성을 검증하기 위해 디지털 서명을 생성하거나 검증하는데 사용
    /// 
    /// 1. Hashing : transaction data(일부 또는 전체) fmf 특정 hash 함수(ex.SH256)을
    ///             사용하여 변환, 고정된 크기의 유일한 문자열로 변환
    /// 2. 서명생성 : hash 값을 transaction 을 생성하는 개인키와 함께 사용하여 디지털 서명 생성
    ///             이 서명은 transaction 전송시 함께 보내지면 이를 통해 해당 transaction이
    ///             해당 개인키 소유자에 의해 생성되었음을 증명할 수 있음.
    /// 3. 서명검증 : transaction 을 수신하는 측은 같은 'sig_hash' method 로 transaction data 에서
    ///             동일한 hash 값을 생성, 그 hash 값과 transaction 에 첨부된 디지털 서명,
    ///             그리고 transaction 에서 참조하는 공개키를 사용하여 서명을 검증
    ///             서명이 유요하다면 해당 transaction 이 해당 개인키 소유자에 의해
    ///             유효하게 생성되었음을 보장할 수 있음.
    // pub fn sig_hash(&self, input_index: usize, redeem_script: usize) -> Result<u32, Box<dyn Error>>{
    //     let mut buf = [0; 4];
    //     LittleEndian::write_u32(&mut buf, self.version);

    //     let mut s = buf.to_vec();
    //     s.append(&mut encode_varint(self.tx_ins.unwrap().len() as u32)?); 

    //     for (idx, tx_in) in self.tx_ins.unwrap().iter().enumerate() {
    //         if idx == input_index {

    //         }
    //     }

    //     Err("TO_DO".into())
    // }

    /// ****  TO DO  ****
    pub fn verify_input(&self, input_index:  usize) -> bool {
        false
    }

    pub fn verify(&self) -> bool {
        // if self.fee() < 0 {
        //     return false
        // } 
        let length = self.tx_ins.as_ref().map(|v| v.len()).unwrap_or(0);
        for i in 0..length {
            if !self.verify_input(i) {
                return false;
            }
        }
        true
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
            "tx: {} {{\nversion: {}\ntx_ins:{}\ntx_outs:{}\nlocktime: {}\n}}",
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

type PrevTx = Vec<u8>;
type PrevIndex = u32;
type ScriptSig = super::script::Script; 
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
        sequence: Option<Sequence>,
    ) -> Self {
        Self {
            prev_tx,
            prev_index,
            script_sig,
            sequence: sequence.unwrap_or(0xffffffff),
        }
    } 

    pub fn parse<R: Read>(reader: &mut R) -> Result<Self, Box<dyn Error>> {
        
        // prev_tx (32bytes, little-endian)
        let mut prev_tx = Vec::<u8>::new();
        reader.read_exact(&mut prev_tx)?;
        prev_tx.reverse();

        // prev_index : u32 from 4 bytes, little-endian
        let mut prev_index_bytes = [0u8; 4];
        reader.read_exact(&mut prev_index_bytes)?;
        
        let  prev_index = PrevIndex::from_le_bytes(prev_index_bytes);

        let script_sig =  Some(Script::parse(reader)?);

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

    pub fn serialize(&self) -> Result<Vec<u8>, Box<dyn Error>> {
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

    pub fn fetch_tx(&self, tx_fetcher: &mut TxFetcher) -> Result<Tx, Box<dyn Error>>{
        let tx = TxFetcher::fetch(
            tx_fetcher, 
            hex::encode(&self.prev_tx), 
            false, 
            false
        )?;
        Ok(tx)
    }

    pub fn value(&self, tx_fetcher: &mut TxFetcher) -> Result<Amount, Box<dyn Error>> {
        let tx = self.fetch_tx(tx_fetcher)?;
        let tx_outs = tx.tx_outs.unwrap();
        Ok(tx_outs[self.prev_index as usize].amount)
    }

    pub fn script_pubkey(&self, tx_fetchre: &mut TxFetcher) -> Result<ScriptPubkey, Box<dyn Error>>{
        let tx = self.fetch_tx(tx_fetchre)?;
        let tx_outs = tx.tx_outs.unwrap();
        Ok(tx_outs[self.prev_index as usize].script_pubkey.clone())
    }
}

impl Display for TxIn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\n{}: {}",
            hex::encode(self.prev_tx.clone()), 
            self.prev_index,
        )
    }
}

//---------------------
//       TxOut
//---------------------

type Amount = u64;
type ScriptPubkey = super::script::Script;

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

    pub fn parse<R: Read>(reader: &mut R) -> Result<Self, Box<dyn Error>> {
        let mut amount_bytes = [0u8; 8];
        reader.read_exact(&mut amount_bytes)?;
        
        let amount = little_endian_to_u64(&amount_bytes);

        let script_pubkey = Script::parse(reader)?;

        Ok(Self { amount, script_pubkey })
    }

    pub fn serialize(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut result = Vec::<u8>::new();

        let amount = self.amount.to_le_bytes();
        result.append(&mut amount.to_vec());

        // let mut script_pubkey = self.script_pubkey.serialize(); // to do
        // result.append(&mut amount.to_vec());

        Ok(result)
    }
}

impl Display for TxOut {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\n{}: {}", self.amount, self.script_pubkey)
    }
}

//---------------------
//     TxFetcher
//---------------------
// Transaction Fetcher 필요성
// 특정 bitcoin transaction 의 정보를 검색 (retrieval) 하는데 필요한 작업을 수행
// transaction ID 를 사용하여 해당 transaction 의 상세 정보를 bitcoin network (또는 testnet)
// 에서 가져옴.
//
// 주요 작업
// 1. transaction 확인 : 특정 transaction 이 성공적으로 처리되었는지 확인하기 위해,
//                       해당 transaction 의 상태를 확인
// 2. transaction detail 확인: transaction input, output, fee, locktime 등의 세부정보 확인
// 3. transaction history 분석: 특정 주소화 관련된 transaction history 분석을 위해 transaction 검색 
// 4. transaction fee 관련 : transaction 을 생성할 때, 수수료를 명시적을 설정해야 하며, 보통 거래 용랑에 비례함.
//                          수수료 계산을 위해 transaction 정보 (input/output 수, script 복잡성 등)
//                          필요하므로 이때 fechter 사용

type Cache = HashMap<String, Tx>;
// key  : transaction ID 
// value: Tx instance 

pub struct TxFetcher {
    cache: Cache,
}

impl TxFetcher {
    pub fn get_url(&self, testnet: bool) -> &str {
        if testnet{
            return "https://blockstream.info/testnet/api"
        } else {
            return "https://blockstream.info/api"
        }
    }

    // fresh == true 이면 cache 를 무시하고 항상 새로운 데이터를 가져옴
    // fresh == false 를 기본으로 하며, cache data 가 있으면 해당 데이터를 사용하고, 
    // 없는 경우에만 새로운 데이터를 가져옴
    // ur1 = "https://blockchain.info/rawtx/{tx_id}?format=hex"
    // -> Blockchain.info API를 통해 트랜잭션의 원시 데이터를 가져오기 위한 엔드포인트
    pub fn fetch(&mut self, tx_id: String, testnet: bool, fresh: bool) -> Result<Tx, Box<dyn Error>> {
        let mut tx = if fresh || !self.cache.contains_key(&tx_id) {
            let url = format!("{}/tx/{}/hex", self.get_url(testnet), &tx_id);
            let response = reqwest::blocking::get(&url)?.text()?;
            let mut raw = hex::decode(response.trim())?;

            // 5 번째 byte 를 확인하는 이유
            // version field (0~4 번째 bytes) 를 확인하여 transaction 의 형식을 지정하는데 사용할 수 있으나
            // 해당 구현에서는 Segwit (Segregated Witness) flag 를 확인 
            // 이는 bitcoin protocol 개선을 위한 일환으로 도입된 flag field
            // Segwit flag byte 의 위치가 5번째이며, 해당 byte 가 0x00 이 면 SegWit transaction 임을 의미
            // 따라서 해당 byte 가 0 일 때와 아닐때 각각 다른 처리가 필요함.
            let tx = if raw[4] == 0 {

                // 실제로 제거할 때는 SegWit 5번째 byte 와 함께 marker field (6번째, 1byte)도
                // 함께 제거한다. 
                // marker field 의 값은 항상 0 이며, SegWit transaction 임을 나타내는 역할을 하므로
                // SegWit 와 함께 삭제 된다.  
                raw.splice(4..6, []);

                // Cursor struct 를 사용하여 in-memory Buffer 에 입력된 raw transation data 를 넣음. 
                let mut tx = Tx::parse(&mut Cursor::new(raw.clone()), testnet)?;
                tx.locktime = Some(LittleEndian::read_u32(&raw[raw.len() - 4..]));
                tx
            } else {
                Tx::parse(&mut Cursor::new(raw), testnet)?
            };

            if tx.id()? != tx_id {
                let msg = format!("not the same id: {} vs {}", tx.id()?, tx_id);
                return Err(msg.into());
            }
            self.cache.insert(tx_id.to_string(), tx.clone());
            tx
        } else {
            // 기존에 받은 tx_id 를 그대로 쓸 경우 cache 내 에서 찾아서 사용
            self.cache.get(&tx_id).unwrap().clone()
        };

        tx.testnet = testnet;
        self.cache.insert(tx_id.clone(), tx.clone());

        Ok(tx.clone())
    } 
}



//---------------------
//       test
//---------------------
#[cfg(test)]
mod tx_test {
    use super::*;


    #[test]
    fn parsing_test() -> Result<(), Box<dyn Error>>{
        // let t1 = Tx::parse()?;
        // println!("{:?}", &t1);
        Ok(())
    }

    #[test]
    fn reqwest_get() -> Result<(), reqwest::Error> {
        // async {
        //     let body = reqwest::get("https://www.rust-lang.org").await?;
        //     let content = body.text().await?;
        //     println!("{}", body);
        // };

        Ok(())
    }

    #[test]
    fn bincode_test() {
        use bincode::{serialize, deserialize};
        use serde::{Serialize, Deserialize};

        #[derive(Serialize, Deserialize, PartialEq, Debug)]
        struct TestStruct {
            a: u32,
            b: u64,
        }

        let test_struct = TestStruct { a: 5, b: 10 };
        let encoded: Vec<u8> = serialize(&test_struct).unwrap();
        let decoded: TestStruct = deserialize(&encoded).unwrap();
        assert_eq!(test_struct, decoded);    
    }    
    
    #[test]
    fn bincode_test2() {
        use std::io::Cursor;
        use bincode::{serialize_into, deserialize_from};
        use serde::{Serialize, Deserialize};

        #[derive(Serialize, Deserialize, Debug, PartialEq)]
        struct Testc {
            a: u32,
            b: u32,
        }

        let testc = Testc {a: 5, b: 10};
        let mut buffer = Cursor::new(Vec::new());
        serialize_into(&mut buffer, &testc).unwrap();

        buffer.set_position(0);

        let decode = deserialize_from(&mut buffer).unwrap();

        assert!(testc == decode);
    }
}

