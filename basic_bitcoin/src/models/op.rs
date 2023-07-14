use std::ops::{Deref, DerefMut};
use sha2::{Sha256, Sha512, Digest};


#[derive(Clone, Debug)]
pub struct Stack(Vec<Vec<u8>>);

impl Deref for Stack {
    type Target = Vec<Vec<u8>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Stack {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0        
    }
}


/// op_dub  : 해당 element 를 복사하여 stack 위에 저장
/// op_code_functions : 118 
pub fn op_dup(stack: &mut Stack) -> bool {
    if stack.len()< 1 { return false; }

    let element = stack[stack.len() - 1].clone();
    stack.push(element);
    true
}

/// op_hash256 : stack element 를 가져와서 hash256 hashing 된 값을 stack 에 저장
/// op_code_function: 170 
pub fn po_hash256(stack: &mut Stack) -> bool {
    if stack.len() < 1 { return false; }

    if let Some(element) = stack.pop() {
        let mut hasher = Sha256::new();
        hasher.update(element);
        stack.push(hasher.finalize().to_vec());
    }
    true
}


/// op_
/// op_code_function: 


/// op_
/// op_code_function: 


/// op_
/// op_code_function: 


/// op_
/// op_code_function: 


/// op_
/// op_code_function: 


/// op_
/// op_code_function: 


/// op_
/// op_code_function: 


/// op_
/// op_code_function: 


/// op_
/// op_code_function: 


/// op_
/// op_code_function: 


/// op_
/// op_code_function: 


/// op_
/// op_code_function: 


/// op_
/// op_code_function: 


/// op_
/// op_code_function: 


/// op_
/// op_code_function: 


/// op_
/// op_code_function: 


/// op_
/// op_code_function: 


/// op_
/// op_code_function: 


/// op_
/// op_code_function: 


/// op_
/// op_code_function: 


/// op_
/// op_code_function: 


/// op_
/// op_code_function: 


/// op_
/// op_code_function: 
fn ex() {}


#[cfg(test)]
mod op_test {
    use super::*;

    #[test]
    fn sha256_test() {
        // create a Sha256 object
        let mut hasher = Sha256::new();

        // write input message
        hasher.update(b"hello world");

        // read hash digest and consume hasher
        let result = hasher.finalize();

        println!("{:?}", result);
    }
}