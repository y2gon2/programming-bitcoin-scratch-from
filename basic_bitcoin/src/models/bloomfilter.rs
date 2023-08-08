
use crate::models::helper::*;

const BIP37_CONSTANT: u32 = 0xfba4c795;

pub struct BloomFilter {
    size: u32,
    bit_field: Vec<bool>,
    function_count: u32,
    tweak: u32,   
}

impl BloomFilter {
    pub fn new(size: u32, function_count: u32, tweak: u32) -> Self {
        Self {
            size,
            bit_field: vec![false; (size * 8) as usize],
            function_count,
            tweak,
        }
    }

    /// Add an item to the filter
    pub fn add(&mut self, item: Vec<u8>) {
        // iterate self.function_count number of times
        for i in 0..self.function_count {
            // BIP0037 spec seed is i*BIP37_CONSTANT + self.tweak
            let seed = i * BIP37_CONSTANT + self.tweak;

            // get the murmur3 hash given that seed
            let h = murmur3(item.clone(), seed);

            // set the bit at the hash mod the bitfield size (self.size*8)
            let bit = h % (self.size * 8);

            // set the bit field at bit to be 1
            self.bit_field[bit as usize] = true;
        }
    }

    pub fn filter_bytes(&self) ->  {
        return bit_fil
    }
}