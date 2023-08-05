
use std::fmt::Display;

use crate::models::helper::*;
use crate::models::network::Message;

//------------------------------------------------------------------------------
//                                 MerkleTree
//------------------------------------------------------------------------------

#[derive(Debug)]
pub struct MerkleTree {
    total: usize,
    max_depth: usize,
    nodes: Vec<Vec<Option<Vec<u8>>>>,

    // merkle tree hash 를 연산할 때, 모든 node 에 대한 hashing 이 필요하지 않을 수 있다.
    // 이런 경우 필요한 부분만 hashing 을 처리하기 위해 현재 iterator 의 위치를 알아야 할 필요가 있다.
    // current_depth 와 current_index 는 이런 작업을 진행하기 위해 필요 (ex. DFS 탐색 진행)
    current_depth: usize,
    current_index: usize,
}

impl MerkleTree {
    pub fn new(total: usize) -> Self {
        let max_depth = (total as f32).log2().ceil() as usize;

        let mut nodes = Vec::<Vec<Option<Vec<u8>>>>::new();
        for depth in 0..max_depth + 1 {
            let mut num_items: usize = total / 2usize.pow(max_depth as u32 - depth as u32);

            if total % 2usize.pow(max_depth as u32 - depth as u32) > 0 { num_items += 1; }

            let level_hashes: Vec<Option<Vec<u8>>> = vec![None; num_items];
            nodes.push(level_hashes);
        }

        Self {
            total,
            max_depth,
            nodes,
            current_depth: 0usize,
            current_index: 0usize,
        }
    }

    /// reduce depth by 1 and halve the index
    /// 만약 양쪽 자식 node 가 None 이 아니라면 부모 hash 값을 구하고 그 부모 위치로 올라간다.
    pub fn up(&mut self) {
        if self.current_depth > 0 { self.current_depth -= 1; }
        self.current_index /= 2;
    }

    /// increase depth by 1 and double the index
    /// 만약 왼쪽 자식 node hash 값이 없다면 그곳의 hash 값을 구하기 위해 왼쪽 아래로 내려감
    pub fn left(&mut self) {
        self.current_depth += 1;
        self.current_index *= 2;
    }

    /// increase depth by 1 and double the index + 1
    /// 만약 오른쪽 자식 node hash 값이 없다면 그곳의 hash 값을 구하기 위해 오른쪽 아래로 내려감
    pub fn right(&mut self) {
        self.current_depth += 1;
        self.current_index = self.current_index * 2 + 1;
    }

    /// 최상단 root 값을 가저옴
    pub fn root(&self) -> Option<Vec<u8>> {
        return self.nodes[0][0].clone()
    }

    pub fn set_current_node(&mut self, value: Option<Vec<u8>>) {
        self.nodes[self.current_depth][self.current_index] = value;
    }

    pub fn get_current_node(&self) -> Option<Vec<u8>> {
        return self.nodes[self.current_depth][self.current_index].clone()
    }

    pub fn get_left_node(&self) -> Option<Vec<u8>> {
        return self.nodes[self.current_depth + 1][self.current_index * 2].clone()
    }

    pub fn get_right_node(&self) -> Option<Vec<u8>> {
        return self.nodes[self.current_depth + 1][self.current_index * 2 + 1].clone()
    }

    pub fn is_leaf(&self) -> bool {
        return self.current_depth == self.max_depth
    }

    pub fn right_exists(&self) -> bool {
        return self.nodes[self.current_depth + 1].len() > self.current_index * 2 + 1
    }

    /// Merkle tree 의 root hash 값을 계산 
    pub fn popular_tree(&self, flag_bis: Vec<u8>, hashes: Vec<[u8; 32]>) {
        while self.root() == None {
            if self.is_leaf() {

            }
        }
    }


}

impl Display for MerkleTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();

        for (depth, level) in self.nodes.iter().enumerate() {
            let mut items = String::new();

            for (idx, h) in level.iter().enumerate() {
                let mut short = String::new();

                match h {
                    None => short = "None".to_string(),
                    Some(v) => {
                        let msg: String = v[0..4].iter().map(|c| format!("{:2x}", c)).collect();
                        short = format!("{}...", msg)
                    }, 
                }

                if depth == self.current_depth as usize && idx == self.current_index as usize {
                    items += &format!("*{}*", short); 
                } else {
                    items += &format!(" {} ", short);
                }
            }
            result += &items;
            result += "\n";
        }  

        write!(f, "{}", result)
    }
}

//---------------------------------- test -------------------------------------

#[cfg(test)]
mod merkle_block {
    use super::*;

    const HASHES: [&str; 16] = [
        "9745f7173ef14ee4155722d1cbf13304339fd00d900b759c6f9d58579b5765fb",
        "5573c8ede34936c29cdfdfe743f7f5fdfbd4f54ba0705259e62f39917065cb9b",
        "82a02ecbb6623b4274dfcab82b336dc017a27136e08521091e443e62582e8f05",
        "507ccae5ed9b340363a0e6d765af148be9cb1c8766ccc922f83e4ae681658308",
        "a7a4aec28e7162e1e9ef33dfa30f0bc0526e6cf4b11a576f6c5de58593898330",
        "bb6267664bd833fd9fc82582853ab144fece26b7a8a5bf328f8a059445b59add",
        "ea6d7ac1ee77fbacee58fc717b990c4fcccf1b19af43103c090f601677fd8836",
        "457743861de496c429912558a106b810b0507975a49773228aa788df40730d41",
        "7688029288efc9e9a0011c960a6ed9e5466581abf3e3a6c26ee317461add619a",
        "b1ae7f15836cb2286cdd4e2c37bf9bb7da0a2846d06867a429f654b2e7f383c9",
        "9b74f89fa3f93e71ff2c241f32945d877281a6a50a6bf94adac002980aafe5ab",
        "b3a92b5b255019bdaf754875633c2de9fec2ab03e6b8ce669d07cb5b18804638",
        "b5c0b915312b9bdaedd2b86aa2d0f8feffc73a2d37668fd9010179261e25e263",
        "c9d52c5cb1e557b92c84c52e7c4bfbce859408bedffc8a5560fd6e35e10b8800",
        "c555bc5fc3bc096df0a0c9532f07640bfb76bfe4fc1ace214b8b228a1297a4c2",
        "f9dbfafc3af3400954975da24eb325e326960a25b87fffe23eef3e7ed2fb610e",
    ];

    #[test]
    fn t1() {
        let a = (u32::MAX as f32).log2();
        let b = a.ceil();
        println!("{}", b);
    }

    #[test]
    fn test_new() {
        let tree = MerkleTree::new(9);
        assert_eq!(tree.nodes[0].len(), 1);
        assert_eq!(tree.nodes[1].len(), 2);
        assert_eq!(tree.nodes[2].len(), 3);
        assert_eq!(tree.nodes[3].len(), 5);
        assert_eq!(tree.nodes[4].len(), 9);

        println!("{}", tree);
    } 

    #[test]
    fn test_dfs_hashing() {
        let mut tree = MerkleTree::new(HASHES.len());

        for (idx, h) in HASHES.iter().enumerate() {
            tree.nodes[4][idx] = Some(str_to_vec_u8(*h));
        }

        println!("{}", &tree);
        println!("----------------------------------------------------------------------------");

        while tree.root() == None {
            if tree.is_leaf() { 
                tree.up(); 
            } else {
                let left_hash = tree.get_left_node();
                let right_hash = tree.get_right_node();

                if left_hash == None { tree.left(); }
                else if right_hash == None { tree.right(); }
                else {
                    tree.set_current_node(
                        Some(merkle_parent(left_hash.unwrap(), right_hash.unwrap()))
                    );
                    tree.up();
                }
            }

            println!(
                "current_depth: {}, current_index: {} current_node_value: {:?}", 
                tree.current_depth, 
                tree.current_index,
                tree.nodes[tree.current_depth][tree.current_index].clone()
            );
        }

        println!("{}", &tree);
    }
}

//------------------------------------------------------------------------------
//                                 MerkleBlock
//------------------------------------------------------------------------------

/// full node 에 대한 정보를 가진 merkle block stream 을 처리하는 struct
/// block header 정보와 함께 뒤 4개 field 정보는  포함 증명에 대한 정보를 포함
/// 
/// * Merkle Block Stream 의 예
/// 00000020df3b053dc46f162a9b00c7f0d5124e2676d47bbe7c5d0793a500000000000000ef44
/// 5fef2ed495c275892206ca533e7411907971013ab83e3b47bd0d692d14d4dc7c835b67d8001a
/// c157e670bf0d00000aba412a0d1480e370173072c9562becffe87aa661c1e4a6dbc305d38ec5
/// dc088a7cf92e6458aca7b32edae818f9c2c98c37e06bf72ae0ce80649a38655ee1e27d34d942
/// 1d940b16732f24b94023e9d572a7f9ab8023434a4feb532d2adfc8c2c2158785d1bd04eb99df
/// 2e86c54bc13e139862897217400def5d72c280222c4cbaee7261831e1550dbb8fa82853e9fe5
/// 06fc5fda3f7b919d8fe74b6282f92763cef8e625f977af7c8619c32a369b832bc2d051ecd9c7
/// 3c51e76370ceabd4f25097c256597fa898d404ed53425de608ac6bfe426f6e2bb457f1c55486
/// 6eb69dcb8d6bf6f880e9a59b3cd053e6c7060eeacaacf4dac6697dac20e4bd3f38a2ea2543d1
/// ab7953e3430790a9f81e1c67f5b58c825acf46bd02848384eebe9af917274cdfbb1a28a5d58a
/// 23a17977def0de10d644258d9c54f886d47d293a411cb6226103b55635
/// 
/// 00000020 (4 bytes, LE)      : version
/// df3b..00 (32bytes, LE)      : previous block
/// ef44..d4 (32bytes, LE)      : merkle root
/// dc7c835b (4 bytes, LE)      : timestamp
/// 67d8001a (4 bytes, BE)      : bits
/// c157e670 (4 bytes, BE)      : nonce
/// bf0d0000 (4 bytes, LE)      : number of total transactions (해당 merkle tree 를 가진 node 수)
/// 0a       (varint     )      : number of hashes
/// ba41..41 (32bytes * n, LE)  : hashes (n: number of hashes)
/// 03b55635 (rest of bytes  )  : flag bits (each bit for DFS searching node and match its hashed value)
///
/// * flag bit 
/// 조건a. 0 : Node 의 hash 값이 hashed 에 포함되어 있음 (이미 알려져 있는 node hash 값)
/// 조건b. 1 : Node 의 hash 값이 hashed 에 포함되어 있지 않음 
///           (light node 가 계산해야 하는 node, 검증 대상의 부모 nodes 또는 root node)
/// 조건c. 1 : 검증 대상 node. hashed 에 포함되어 있으나 최종 root hash 값을 구하여 
///           해당 block 이 실제로 포함되는 것인지 확인해야 할 대상
/// 
/// 모든 node 에 대한 hash 값이 대상인 것은 아님. 검증 대상이 아닌 node 에 대한 부분 tree 의 경우
/// 해당 부분의 최상단 node hash 값만 있어도 검증이 가능.
/// 
///                              1. Root
///           2. Habcdefgh                     3. Hijklmnop
///        Habcd          Hefgh          4. Hijkl           9 Hmnop          
///     Hab    Hcd      Hef    Hgh     5. Hij   6. Hkl    10.Hmn    13.Hop      
///   Ha  Hb  Hc  Hd  He  Hf  Hg  Hh  Hi  Hj  7.Hk 8.Hl 11.Hm 12Hn  H0  Hp
///                                             |               |
///                                        검증필요 block  검증필요 block
/// 
/// 위 기준에서 1 ~ 13 의 flag bit 를 표시하면 다음과 같다.
/// 
///   조건     : b  a  b  b  b  b  c  a  b  b  a  c  a
///   node No. : 1  2  3  4  5  6  7  8  9 10 11 12 13
///   bit      : 1  0  1  1  0  1  1  0  1  1  0  1  0
///    
pub struct MerkleBlock {
    version: u32,
    prev_block: [u8; 32],
    merkle_root: [u8; 32],
    timestamp: u32,
    bits: [u8; 4],
    nonce: [u8; 4],
    total: u32,
    hashes: Vec<[u8; 32]>,
    flags: Vec<u8>,
}

impl MerkleBlock {
    fn new(
        version: u32,
        prev_block: [u8; 32],
        merkle_root: [u8; 32],
        timestamp: u32,
        bits: [u8; 4],
        nonce: [u8; 4],
        total: u32,
        hashes: Vec<[u8; 32]>,
        flags: Vec<u8>,
    ) -> Self {
        Self {
            version, 
            prev_block, 
            merkle_root,
            timestamp,
            bits,
            nonce,
            total,
            hashes,
            flags
        }
    }
}

impl Message for MerkleBlock {
    fn command(&self) -> Vec<u8> {
        vec![b'm', b'e', b'r', b'k', b'l', b'e', b'b', b'l', b'o', b'c', b'k']
    }   

    /// Takes a byte stream and parses a merkle block. Returns a Merkle Block object
    fn parse<R: std::io::Read>(reader: &mut R) -> Self where Self: Sized {
        let mut version_buf = [0u8; 4];
        let _ = reader.read_exact(&mut version_buf);
        let version = u32::from_le_bytes(version_buf);

        let mut prev_block = [0u8; 32];
        let _ = reader.read_exact(&mut prev_block);
        prev_block.reverse();

        let mut merkle_root = [0u8; 32];
        let _ = reader.read_exact(&mut merkle_root);
        merkle_root.reverse();

        let mut ts_buf = [0u8; 4];
        let _ = reader.read_exact(&mut ts_buf);
        let timestamp = u32::from_le_bytes(ts_buf);

        let mut bits = [0u8; 4];
        let _ = reader.read_exact(&mut bits);

        let mut nonce = [0u8; 4];
        let _ = reader.read_exact(&mut nonce);

        let mut total_buf = [0u8; 4];
        let _ = reader.read_exact(&mut total_buf);
        let total = u32::from_le_bytes(total_buf);

        let mut num_hashes = read_varint(reader).unwrap();
        let mut hashes = Vec::<[u8; 32]>::new();

        for _ in 0..num_hashes {
            let mut buf = [0u8; 32];
            let _ = reader.read_exact(&mut buf);
            
            buf.reverse();
            hashes.push(buf);
        }

        let mut flags = Vec::<u8>::new();
        let _ = reader.read_to_end(&mut flags);

        return MerkleBlock::new(
            version,
            prev_block,
            merkle_root,
            timestamp,
            bits,
            nonce,
            total,
            hashes,
            flags
        )
    }

    fn serialize(&self) -> Vec<u8> {
        return Vec::<u8>::new()
    }
}

impl Display for MerkleBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = format!("{}\n", self.total);

        for hash in self.hashes.iter() {
            let s: String = hash.iter().map(|u| format!("\t{:2x}\n", u)).collect();
            result += &s;
        }

        let flag: String = self.flags.iter().map(|u| format!("{:2x}", u)).collect();
        
        result += &flag;
        write!(f, "{}", result)
    }
}


//---------------------------------- test -------------------------------------

#[cfg(test)]
mod merkle_block_test {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_parse() {
        let merkle_block_str = "00000020df3b053dc46f162a9b00c7f0d5124e2676d47bbe7c5d0793a5\
            00000000000000ef445fef2ed495c275892206ca533e7411907971013ab83e3b47bd0d692d14d4dc7c83\
            5b67d8001ac157e670bf0d00000aba412a0d1480e370173072c9562becffe87aa661c1e4a6dbc305d38e\
            c5dc088a7cf92e6458aca7b32edae818f9c2c98c37e06bf72ae0ce80649a38655ee1e27d34d9421d940b\
            16732f24b94023e9d572a7f9ab8023434a4feb532d2adfc8c2c2158785d1bd04eb99df2e86c54bc13e13\
            9862897217400def5d72c280222c4cbaee7261831e1550dbb8fa82853e9fe506fc5fda3f7b919d8fe74b\
            6282f92763cef8e625f977af7c8619c32a369b832bc2d051ecd9c73c51e76370ceabd4f25097c256597f\
            a898d404ed53425de608ac6bfe426f6e2bb457f1c554866eb69dcb8d6bf6f880e9a59b3cd053e6c7060e\
            eacaacf4dac6697dac20e4bd3f38a2ea2543d1ab7953e3430790a9f81e1c67f5b58c825acf46bd028483\
            84eebe9af917274cdfbb1a28a5d58a23a17977def0de10d644258d9c54f886d47d293a411cb6226103b55635";

        let stream_vec = str_to_vec_u8(merkle_block_str);
        let mut stream = Cursor::new(stream_vec);
        let mb = MerkleBlock::parse(&mut stream);

        let version: u32 = 0x20000000;
        assert_eq!(mb.version, version);

        let root_str = "ef445fef2ed495c275892206ca533e7411907971013ab83e3b47bd0d692d14d4";
        let mut merkle_root = str_to_vec_u8(root_str);
        merkle_root.reverse();
        assert_eq!(mb.merkle_root.to_vec(), merkle_root);

        let prev_block_str = "df3b053dc46f162a9b00c7f0d5124e2676d47bbe7c5d0793a500000000000000";
        let mut prev_block = str_to_vec_u8(prev_block_str);
        prev_block.reverse();
        assert_eq!(mb.prev_block.to_vec(), prev_block);

        let ts_str = "dc7c835b";
        let mut ts_vec = str_to_vec_u8(ts_str);
        let ts: [u8; 4] = ts_vec.try_into().unwrap();
        assert_eq!(mb.timestamp, u32::from_le_bytes(ts)); 

        let bits: [u8; 4] = [0x67, 0xd8, 0x00, 0x1a];
        assert_eq!(mb.bits, bits);

        let nonce: [u8; 4] = [0xc1, 0x57, 0xe6, 0x70];
        assert_eq!(mb.nonce, nonce);

        let total = u32::from_le_bytes([0xbf, 0x0d, 0x00, 0x00]);
        assert_eq!(mb.total, total);

        let hashes = [
            str_to_vec_u8("ba412a0d1480e370173072c9562becffe87aa661c1e4a6dbc305d38ec5dc088a"),
            str_to_vec_u8("7cf92e6458aca7b32edae818f9c2c98c37e06bf72ae0ce80649a38655ee1e27d"),
            str_to_vec_u8("34d9421d940b16732f24b94023e9d572a7f9ab8023434a4feb532d2adfc8c2c2"),
            str_to_vec_u8("158785d1bd04eb99df2e86c54bc13e139862897217400def5d72c280222c4cba"),
            str_to_vec_u8("ee7261831e1550dbb8fa82853e9fe506fc5fda3f7b919d8fe74b6282f92763ce"),
            str_to_vec_u8("f8e625f977af7c8619c32a369b832bc2d051ecd9c73c51e76370ceabd4f25097"),
            str_to_vec_u8("c256597fa898d404ed53425de608ac6bfe426f6e2bb457f1c554866eb69dcb8d"),
            str_to_vec_u8("6bf6f880e9a59b3cd053e6c7060eeacaacf4dac6697dac20e4bd3f38a2ea2543"),
            str_to_vec_u8("d1ab7953e3430790a9f81e1c67f5b58c825acf46bd02848384eebe9af917274c"),
            str_to_vec_u8("dfbb1a28a5d58a23a17977def0de10d644258d9c54f886d47d293a411cb62261"),
        ];

        assert_eq!(mb.hashes.len(), hashes.len());

        for i in 0..hashes.len() {
            let mut h = hashes[i].clone();
            h.reverse();
            assert_eq!(mb.hashes[i].to_vec(), h);
        }

        let flags: Vec<u8> = vec![0x03, 0xb5, 0x56, 0x35];
        assert_eq!(mb.flags, flags);
    }
}