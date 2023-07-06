//! collection of functions


fn litte_endian_to_u32(four_u8: &[u8; 4]) -> u32 {
    let mut result: u32 = 0;

    for (idx, &byte) in four_u8.iter().enumerate() {
        result += (byte as u32) << (8 * idx);
    }
    result
} 


#[cfg(test)]
mod hleper_test {
    use super::litte_endian_to_u32;


    #[test]
    fn litte_endian() {
        let byte_sequence: [u8; 4] = [0xff, 0x00, 0x00, 0x00];
        let result: u32 = litte_endian_to_u32(&byte_sequence);

        println!("{}", result); // 255
    }
}