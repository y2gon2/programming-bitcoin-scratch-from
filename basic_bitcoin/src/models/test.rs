use std::ops::{Deref, DerefMut};




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

#[cfg(test)]
mod op_test {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn hashmap_fn() {
        let a: Vec<u8> = vec![1, 2, 3];
        let b: Vec<u8> = vec![11, 22, 33];
        
        let mut stack = Stack(vec![a, b]);

        let c: Vec<u8> = vec![9, 8, 7];
        let d: Vec<u8> = vec![99, 88, 77];
        
        let mut alt_stack = Stack(vec![c, d]);

        pub enum FnTypesTest {
            StackOnly(Box<dyn Fn(&mut Stack) -> bool>),
            WithElement(Box<dyn Fn(&mut Stack, &mut Vec<u8>) -> bool>),
            WithAltStack(Box<dyn Fn(&mut Stack, &mut Stack) -> bool>),
            WithSeqOthers(Box<dyn Fn(&mut Stack, u32, u32) -> bool>),
        }

        fn f1(stack: &mut Stack) -> bool {
            stack.pop();
            true
        }

        fn f2(stack: &mut Stack, v: &mut Vec<u8>) -> bool {
            stack.push(v.clone());
            true
        }

        fn f3(stack: &mut Stack, alt_stack: &mut Stack) -> bool {
            stack.append(alt_stack);
            true
        }

        fn f4(stack: &mut Stack, n1: u32, n2: u32) -> bool {
            println!("stack:{:?}\tn1:{}\tn2:{}", stack, n1, n2);
            true
        }

        let hash: HashMap<u8, FnTypesTest> = HashMap::from([
            (1, FnTypesTest::StackOnly(Box::new(f1))),
            (2, FnTypesTest::WithElement(Box::new(f2))),
            (3, FnTypesTest::WithAltStack(Box::new(f3))),
            (4, FnTypesTest::WithSeqOthers(Box::new(f4))),
        ]);

        for i in 1..=4 as u8 {
            match hash.get(&i) {
                Some(FnTypesTest::StackOnly(func)) => {
                    func(&mut stack);
                    println!("1: {:?}", stack);
                },
                Some(FnTypesTest::WithElement(func)) => {
                    func(&mut stack, &mut vec![6, 7, 8]);
                    println!("2: {:?}", stack);
                }
                Some(FnTypesTest::WithAltStack(func)) => {
                    func(&mut stack, &mut alt_stack);
                    println!("3: {:?}", stack);
                },
                Some(FnTypesTest::WithSeqOthers(func)) => {
                    func(&mut stack, 200, 201);
                    println!("4: {:?}", stack);
                },
                None => {();},
            }
            
        }
    }   

    #[test]
    fn t1() {
        fn function1(arg: i32) -> i32 {
            arg + 1
        }
        
        fn function2(arg: i32) -> i32 {
            arg * 2
        }
        let functions: Vec<Box<dyn Fn(i32) -> i32>> = vec![
            Box::new(function1),
            Box::new(function2),
        ];
        
        for func in functions {
            println!("{}", func(2));  // prints "3" and "4" respectively
        }
    }

    #[test]
    fn op_test1() {
        use crate::models::op::*;

        let mut stack = Stack::new();  

        stack.op_6();     
        stack.op_1add();

        let decoded = decode_num(&stack.stack_pop());
        assert!(7 == decoded);
    }

    #[test]
    fn op_test2() {
        use crate::models::op::*;

        let mut stack = Stack::new();  

        stack.op_16();
        stack.op_6();
        stack.op_negate();     
        stack.op_add();
        stack.op_10();
        stack.op_mul();
        stack.op_15();
        stack.op_mul();

        //let decoded = decode_num(&stack.stack_pop()); // test 용으로 직접 element 꺼내기

        let encoded = encode_num(1500); 
        stack.stack_push(encoded); // test 용으로 직접 element 쌓기

        assert!(stack.op_equal());
    }

    #[test]
    fn scirpt_display() {
        use crate::models::script::*;

        let cmd_vec = vec![
            Cmd::OpCode(0x56),
            Cmd::OpCode(0x76),
            Cmd::OpCode(0x87),
            Cmd::OpCode(0x93),
            Cmd::OpCode(0x95),
            Cmd::BytesData(vec![1, 100, 200, 255]),
            ];
    
        let script = Script::new(Some(cmd_vec));

        println!("{}", script); // op_6 op_dup op_equal op_add op_mul 0164c8ff
    }

    #[test]
    fn creat_tx() {
        
    }
}