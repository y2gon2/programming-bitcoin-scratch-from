
use std::io::{Read};
use byteorder::LittleEndian;

use crate::models::helper::*;

pub struct Block {
    version: u32,
    prev_block: [u8; 32],  // little-endian
    merkle_root: [u8; 32], // little-endian
    timestamp: u32,
    bits: [u8; 4],
    nonce: [u8; 4],
}

impl Block {
    pub fn new(
        version: u32, 
        prev_block: [u8; 32], 
        merkle_root: [u8; 32],
        timestamp: u32,
        bits: [u8; 4],
        nonce: [u8; 4],
    ) -> Self {
        Self { 
            version, 
            prev_block, 
            merkle_root, 
            timestamp, 
            bits, 
            nonce, 
        }
    }

    /// to do
    /// Takes a byte stream an parses a block. Returns a Block object
    pub fn parse<R: Read>(reader:&mut R) -> Self {
        // s.read(n) will read n bytes from the stream

        // version - 4 bytes, little endian, interpret as int   
        let mut buf_ver = [0u8; 4];
        reader.read_exact(&mut buf_ver);
        let version = u32::from_le_bytes(buf_ver);

        // prev_block - 32 bytes, little endian (use [::-1] to reverse)        
        let mut prev_block  = [0u8; 32];
        reader.read_exact(&mut prev_block);
        prev_block.reverse();
        
        // merkle_root - 32 bytes, little endian (use [::-1] to reverse)
        let mut merkle_root = [0u8; 32];
        reader.read_exact(&mut merkle_root);
        merkle_root.reverse();
        
        // timestamp - 4 bytes, little endian, interpret as int
        let mut buf_ts = [0u8; 4];
        reader.read_exact(&mut buf_ts);
        let timestamp = u32::from_le_bytes(buf_ts);

        // bits - 4 bytes
        let mut bits = [0u8; 4];
        reader.read_exact(&mut bits);
        
        // nonce - 4 bytes
        let mut nonce = [0u8; 4];
        reader.read_exact(&mut nonce);

        Self {
            version,
            prev_block,
            merkle_root,
            timestamp,
            bits,
            nonce,
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



}
