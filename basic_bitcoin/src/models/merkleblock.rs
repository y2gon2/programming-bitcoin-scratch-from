
use std::fmt::Display;

use crate::models::helper::*;

//------------------------------------------------------------------------------
//                                 MerkleTree
//------------------------------------------------------------------------------

pub struct MerkleTree {
    total: u32,
    max_depth: u8,
    nodes: Vec<Vec<Option<Vec<u8>>>>,
    current_depth: u8,
    current_index: u32,
}

impl MerkleTree {
    pub fn new(total: u32) -> Self {
        let max_depth = (total as f32).log2().ceil() as u8;

        let mut nodes = Vec::<Vec<Option<Vec<u8>>>>::new();
        for depth in 0..max_depth + 1 {
            let mut num_items: usize = (total / 2u32.pow(max_depth as u32 - depth as u32)) as usize;

            if total % 2u32.pow(max_depth as u32 - depth as u32) > 0 { num_items += 1; }

            let level_hashes: Vec<Option<Vec<u8>>> = vec![None; num_items];
            nodes.push(level_hashes);
        }

        Self {
            total,
            max_depth,
            nodes,
            current_depth: 0u8,
            current_index: 0u32,
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
                        let msg: String = v[0..8].iter().map(|c| c.to_string()).collect();
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


#[cfg(test)]
mod merkle_block {
    use super::*;

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

}
