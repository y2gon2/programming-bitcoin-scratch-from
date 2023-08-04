
use std::io::Read;

use num_bigint::BigUint;
use lazy_static::lazy_static;

use crate::models::helper::*;

pub const GENESIS_BLOCK_STR: &str = "010000000000000000000000000000000000000000000000000000000\
    0000000000000003ba3edfd7a7b12b27ac72c3e67768f617fc81bc3888a51323a9fb8aa4b1e5e4a29a\
    b5f49ffff001d1dac2b7c";
pub const TESTNET_GENESIS_BLOCK_STR: &str = "0100000000000000000000000000000000000000000000000\
    000000000000000000000003ba3edfd7a7b12b27ac72c3e67768f617fc81bc3888a51323a9fb8aa4b1\
    e5e4adae5494dffff001d1aa4ae18";

pub const LOWEST_BITS_STR: &str = "ffff001d";

const INITAIL_COEF: u32 = 0xffff;
const LOWEST_EXPONENT: u32 = 0x1d;

lazy_static!{
    // The formula : coefficient * 256**(exponent-3)
    pub static ref LOWEST: BigUint = BigUint::from(INITAIL_COEF) 
        * BigUint::from(256 as usize).pow(LOWEST_EXPONENT - 3);

    pub static ref GENESIS_BLOCK: Vec<u8> = str_to_vec_u8(GENESIS_BLOCK_STR);

    pub static ref TESTNET_GENESIS_BLOCK: Vec<u8> = str_to_vec_u8(TESTNET_GENESIS_BLOCK_STR);
}

pub struct Block {
    version: u32,
    prev_block: [u8; 32],  // little-endian
    merkle_root: [u8; 32], // little-endian
    timestamp: u32,
    bits: [u8; 4],
    nonce: [u8; 4],
    tx_hashes: Option<Vec<Vec<u8>>>,
}

impl Block {
    pub fn new(
        version: u32, 
        prev_block: [u8; 32], 
        merkle_root: [u8; 32],
        timestamp: u32,
        bits: [u8; 4],
        nonce: [u8; 4],
        tx_hashes: Option<Vec<Vec<u8>>>,
    ) -> Self {
        Self { 
            version, 
            prev_block, 
            merkle_root, 
            timestamp, 
            bits, 
            nonce, 
            tx_hashes,
        }
    }

    /// to do
    /// Takes a byte stream an parses a block. Returns a Block object
    pub fn parse<R: Read>(reader:&mut R) -> Self {
        // s.read(n) will read n bytes from the stream

        // version - 4 bytes, little endian, interpret as int   
        let mut buf_ver = [0u8; 4];
        let _ = reader.read_exact(&mut buf_ver);
        let version = u32::from_le_bytes(buf_ver);

        // prev_block - 32 bytes, little endian (use [::-1] to reverse)        
        let mut prev_block  = [0u8; 32];
        let _ = reader.read_exact(&mut prev_block);
        prev_block.reverse();
        
        // merkle_root - 32 bytes, little endian (use [::-1] to reverse)
        let mut merkle_root = [0u8; 32];
        let _ = reader.read_exact(&mut merkle_root);
        merkle_root.reverse();
        
        // timestamp - 4 bytes, little endian, interpret as int
        let mut buf_ts = [0u8; 4];
        let _ = reader.read_exact(&mut buf_ts);
        let timestamp = u32::from_le_bytes(buf_ts);

        // bits - 4 bytes
        let mut bits = [0u8; 4];
        let _ = reader.read_exact(&mut bits);
        
        // nonce - 4 bytes
        let mut nonce = [0u8; 4];
        let _ = reader.read_exact(&mut nonce);

        Self {
            version,
            prev_block,
            merkle_root,
            timestamp,
            bits,
            nonce,
            tx_hashes: None,
        }
    }

    /// Returns the 80 byte block header
    pub fn serialize(&mut self) -> Vec<u8> {
        let mut result = Vec::<u8>::new();

        // version - 4 bytes, little endian
        let mut ver_vec = u32::to_le_bytes(self.version).to_vec();
        result.append(&mut ver_vec);
    
        // prev_block - 32 bytes, little endian
        let mut prev_block_vec = self.prev_block.clone().to_vec();
        prev_block_vec.reverse();

        result.append(&mut prev_block_vec);
    
        // merkle_root - 32 bytes, little endian
        let mut merkle_root_vec = self.merkle_root.clone().to_vec();
        merkle_root_vec.reverse();

        result.append(&mut merkle_root_vec);
    
        // timestamp - 4 bytes, little endian
        let mut ts_vec = u32::to_le_bytes(self.timestamp).to_vec();
        result.append(&mut ts_vec);
    
        // bits - 4 bytes
        result.append(&mut self.bits.clone().to_vec());
    
        // nonce - 4 bytes
        result.append(&mut self.nonce.clone().to_vec());

        result
    }

    /// Returns the hash256 interpreted little endian of the block
    pub fn hash(&mut self) -> Vec<u8> {
        // serialize
        let s = self.serialize();
        
        // hash256
        let mut h256 = hash256(&s);
        
        // reverse
        h256.reverse();
        h256
    }

    /// Returns whether this block is signaling readiness for BIP9
    pub fn bif9(&self) -> bool {
        // BIP9 is signalled if the top 3 bits are 001
        // remember version is 32 bytes so right shift 29 (>> 29) and see if
        // that is 001
        return self.version >> 29 == 0b001
    }

    /// Returns whether this block is signaling readiness for BIP91
    pub fn bif91(&self) -> bool {
        // BIP91 is signalled if the 5th bit from the right is 1
        // shift 4 bits to the right and see if the last bit is 1
        return self.version >> 4 & 1 == 1
    }

    /// Returns whether this block is signaling readiness for BIP141
    pub fn bif141(&self) -> bool {
        // BIP91 is signalled if the 2nd bit from the right is 1
        // shift 1 bit to the right and see if the last bit is 1
        return self.version >> 1 & 1 == 1
    }

    /// Returns the proof-fo-work target based on the bits
    pub fn target(&self) -> BigUint {
        return bits_to_target(self.bits)
    }
    
    /// Returns the block difficulty based on the bits
    /// initail difficulty = 1
    pub fn difficulty(&self) -> BigUint {
        return LOWEST.clone() / self.target()
    }

    /// Returns whether this block satisfies proof of work
    pub fn check_pow(&mut self) -> bool {
        let h256 = hash256(&self.serialize());
        let proof = BigUint::from_bytes_le(&h256);

        return proof < self.target() 
    }


    /// Gets the merkle root of the tx_hashes and checks 
    /// that it's the same as the merkle root of this block.
    pub fn validate_merkle_root(&self) -> bool {
        let tx_hashes = self.tx_hashes
            .as_ref()
            .expect("Not exist tx_hashes in the Block");

        let reverse_hashes: Vec<Vec<u8>> = tx_hashes
            .iter()
            .map(|hash| hash.iter().rev().cloned().collect())
            .collect();
        
        let mut computed_root = merkle_root(reverse_hashes);
        computed_root.reverse();
    
        return self.merkle_root.to_vec() == computed_root
    }

    pub fn get_timestamp(&self) -> u32 {
        return self.timestamp
    }
}

#[cfg(test)]
mod block_test {
    use super::*;
    use std::io::Cursor;

    const RAW: &str = "020000208ec39428b17323fa0ddec8e887b4a7c53b8c0a0a220cfd0000000000000000005b07\
    50fce0a889502d40508d39576821155e9c9e3f5c3157f961db38fd8b25be1e77a759e93c0118a4ffd71d";

    #[test]
    fn difficulty_test() {
        let bits_str = "e9 3c 01 18";
        let bits_vec: Vec<u8> = bits_str
            .split_ascii_whitespace()
            .map(|c| u8::from_str_radix(c, 16).unwrap())
            .collect();

        let bits: [u8; 4] = match bits_vec.try_into() {
            Ok(bits) => bits,
            Err(_) => panic!("Vec length not equal to array length!"),
        };

        let exp = bits[3] as u32;
        let mut coef_array = bits.clone();
        coef_array[3] = 0u8;
        let coef = u32::from_le_bytes(coef_array);

        let target = BigUint::from(coef) * BigUint::from(256u32).pow(exp - 3);
        let diffeculty = LOWEST.clone() / target;
        println!("diffeculty :{}", diffeculty);
    }

    #[test]
    fn test_parse() {
        let block_raw = str_to_vec_u8(RAW);

        let mut cursor = Cursor::new(block_raw);
        let block = Block::parse(&mut cursor);

        assert!(block.version == 0x20000002u32);
        println!("block :{:?}", block.prev_block); 
        println!("check :{:?}", 
            str_to_vec_u8("000000000000000000fd0c220a0a8c3bc5a7b487e8c8de0dfa2373b12894c38e")
        );
        // block :[0, 0, 0, 0, 0, 0, 0, 0, 0, 253, 12, 34, 10, 10, 140, 59, 197, 167, 180, 135, 232, 200, 222, 13, 250, 35, 115, 177, 40, 148, 195, 142]
        // check :[0, 0, 0, 0, 0, 0, 0, 0, 0, 253, 12, 34, 10, 10, 140, 59, 197, 167, 180, 135, 232, 200, 222, 13, 250, 35, 115, 177, 40, 148, 195, 142]
    
        let check_merkle_root = str_to_vec_u8("be258bfd38db61f957315c3f9e9c5e15216857398d50402d5089a8e0fc50075b");
        for i in 0..32 {
            assert_eq!(block.merkle_root[i], check_merkle_root[i]);
        }

        assert_eq!(block.timestamp, 0x59a7771e);

        let check_bits = str_to_vec_u8("e93c0118");
        for i in 0..4 {
            assert_eq!(block.bits[i], check_bits[i]);
        }

        let check_nonce = str_to_vec_u8("a4ffd71d");
        for i in 0..4 {
            assert_eq!(block.nonce[i], check_nonce[i]);
        }
    }

    #[test]
    fn test_serialize() {
        let block_raw = str_to_vec_u8(RAW);
        let mut cursor = Cursor::new(block_raw.clone());
        let mut block = Block::parse(&mut cursor);

        assert_eq!(block.serialize(), block_raw);
    }

    #[test]
    fn test_hash() {
        let block_raw =  str_to_vec_u8(RAW);
        let mut cursor = Cursor::new(block_raw);
        let mut block = Block::parse(&mut cursor);

        assert_eq!(
            block.hash(),
            str_to_vec_u8("0000000000000000007e9e4c586439b0cdbe13b1370bdd9435d76a644d047523")
        );
    }

    #[test]
    fn test_bip9() {
        let raw1 = "020000208ec39428b17323fa0ddec8e887b4a7c53b8c0a0a220cfd0000000000000000005b0750fce\
            0a889502d40508d39576821155e9c9e3f5c3157f961db38fd8b25be1e77a759e93c0118a4ffd71d";
        let block_raw1 =  str_to_vec_u8(raw1);
        let mut cursor1 = Cursor::new(block_raw1);
        let block1 = Block::parse(&mut cursor1);

        assert!(block1.bif9());

        let raw2 = "0400000039fa821848781f027a2e6dfabbf6bda920d9ae61b63400030000000000000000ecae536a3\
            04042e3154be0e3e9a8220e5568c3433a9ab49ac4cbb74f8df8e8b0cc2acf569fb9061806652c27";
        let block_raw1 = str_to_vec_u8(raw2);
        let mut cursor2 = Cursor::new(block_raw1);
        let block2 = Block::parse(&mut cursor2);

        assert!(!block2.bif9());
    }

    #[test]
    fn test_bif91() {
        let raw1 = "1200002028856ec5bca29cf76980d368b0a163a0bb81fc192951270100000000000000003288f32a2\
            831833c31a25401c52093eb545d28157e200a64b21b3ae8f21c507401877b5935470118144dbfd1";
        let block_raw1 =  str_to_vec_u8(raw1);
        let mut cursor1 = Cursor::new(block_raw1);
        let block1 = Block::parse(&mut cursor1);

        assert!(block1.bif91());

        let raw2 = "020000208ec39428b17323fa0ddec8e887b4a7c53b8c0a0a220cfd0000000000000000005b0750fce\
            0a889502d40508d39576821155e9c9e3f5c3157f961db38fd8b25be1e77a759e93c0118a4ffd71d";
        let block_raw1 = str_to_vec_u8(raw2);
        let mut cursor2 = Cursor::new(block_raw1);
        let block2 = Block::parse(&mut cursor2);

        assert!(!block2.bif91());
    }

    #[test]
    fn test_bif141() {
        let raw1 = "020000208ec39428b17323fa0ddec8e887b4a7c53b8c0a0a220cfd0000000000000000005b0750fce0\
            a889502d40508d39576821155e9c9e3f5c3157f961db38fd8b25be1e77a759e93c0118a4ffd71d";
        let block_raw1 =  str_to_vec_u8(raw1);
        let mut cursor1 = Cursor::new(block_raw1);
        let block1 = Block::parse(&mut cursor1);

        assert!(block1.bif141());

        let raw2 = "0000002066f09203c1cf5ef1531f24ed21b1915ae9abeb691f0d2e0100000000000000003de0976428c\
            e56125351bae62c5b8b8c79d8297c702ea05d60feabb4ed188b59c36fa759e93c0118b74b2618";
        let block_raw1 = str_to_vec_u8(raw2);
        let mut cursor2 = Cursor::new(block_raw1);
        let block2 = Block::parse(&mut cursor2);

        assert!(!block2.bif141());
    }

    #[test]
    fn test_target() {
        use num_traits::Num;

        let raw = "020000208ec39428b17323fa0ddec8e887b4a7c53b8c0a0a220cfd0000000000000000005b0750fce0a\
            889502d40508d39576821155e9c9e3f5c3157f961db38fd8b25be1e77a759e93c0118a4ffd71d";
        let block_raw =  str_to_vec_u8(raw);
        let mut cursor = Cursor::new(block_raw);
        let block = Block::parse(&mut cursor);

        let check_raw = "13ce9000000000000000000000000000000000000000000";
        let checker = BigUint::from_str_radix(check_raw, 16).unwrap();

        assert_eq!(
            block.target(),
            checker
        );
    }

    #[test]
    fn test_difficulty() {
        let raw = "020000208ec39428b17323fa0ddec8e887b4a7c53b8c0a0a220cfd0000000000000000005b0750fce0\
            a889502d40508d39576821155e9c9e3f5c3157f961db38fd8b25be1e77a759e93c0118a4ffd71d";
        let block_raw =  str_to_vec_u8(raw);
        let mut cursor = Cursor::new(block_raw);
        let block = Block::parse(&mut cursor);

        assert_eq!(
            block.difficulty(),
            BigUint::from(888171856257u64)
        );
    }

    #[test]
    fn test_check_pow() {
        let raw1 = "04000000fbedbbf0cfdaf278c094f187f2eb987c86a199da22bbb20400000000000000007b7697b291\
            29648fa08b4bcd13c9d5e60abb973a1efac9c8d573c71c807c56c3d6213557faa80518c3737ec1";
        let block_raw1 =  str_to_vec_u8(raw1);
        let mut cursor1 = Cursor::new(block_raw1);
        let mut block1 = Block::parse(&mut cursor1);

        assert!(block1.check_pow());
        
        let raw2 = "04000000fbedbbf0cfdaf278c094f187f2eb987c86a199da22bbb20400000000000000007b7697b2912\
            9648fa08b4bcd13c9d5e60abb973a1efac9c8d573c71c807c56c3d6213557faa80518c3737ec0";
        let block_raw2 =  str_to_vec_u8(raw2);
        let mut cursor2 = Cursor::new(block_raw2);
        let mut block2 = Block::parse(&mut cursor2);

        assert!(!block2.check_pow());
    }

    #[test]
    fn test_validate_merkle_root() {
        let hashes = vec![
            str_to_vec_u8("f54cb69e5dc1bd38ee6901e4ec2007a5030e14bdd60afb4d2f3428c88eea17c1"),
            str_to_vec_u8("c57c2d678da0a7ee8cfa058f1cf49bfcb00ae21eda966640e312b464414731c1"),
            str_to_vec_u8("b027077c94668a84a5d0e72ac0020bae3838cb7f9ee3fa4e81d1eecf6eda91f3"),
            str_to_vec_u8("8131a1b8ec3a815b4800b43dff6c6963c75193c4190ec946b93245a9928a233d"),
            str_to_vec_u8("ae7d63ffcb3ae2bc0681eca0df10dda3ca36dedb9dbf49e33c5fbe33262f0910"),
            str_to_vec_u8("61a14b1bbdcdda8a22e61036839e8b110913832efd4b086948a6a64fd5b3377d"),
            str_to_vec_u8("fc7051c8b536ac87344c5497595d5d2ffdaba471c73fae15fe9228547ea71881"),
            str_to_vec_u8("77386a46e26f69b3cd435aa4faac932027f58d0b7252e62fb6c9c2489887f6df"),
            str_to_vec_u8("59cbc055ccd26a2c4c4df2770382c7fea135c56d9e75d3f758ac465f74c025b8"),
            str_to_vec_u8("7c2bf5687f19785a61be9f46e031ba041c7f93e2b7e9212799d84ba052395195"),
            str_to_vec_u8("08598eebd94c18b0d59ac921e9ba99e2b8ab7d9fccde7d44f2bd4d5e2e726d2e"),
            str_to_vec_u8("f0bb99ef46b029dd6f714e4b12a7d796258c48fee57324ebdc0bbc4700753ab1"),
        ];

        let stream = str_to_vec_u8("00000020fcb19f7895db08cadc9573e7915e3919f\
            b76d59868a51d995201000000000000acbcab8bcc1af95d8d563b77d24c3d19b18f1486383d75a508\
            5c4e86c86beed691cfa85916ca061a00000000");

        let mut stream_cursor = Cursor::new(stream);
        let mut block = Block::parse(&mut stream_cursor);
        block.tx_hashes = Some(hashes);

        assert!(block.validate_merkle_root());
    }
}