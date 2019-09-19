use std::rc::Rc;
use string_interner::DefaultStringInterner;
use string_interner::Sym;
use std::collections::HashMap;
use std::slice::Chunks;
use crate::opcode::{OpCode, Chunk};
use crate::instance::{
    Instance,
    Instance::*
};

pub struct VM {
    class_registry : HashMap<Sym, Instance>,
    // Represents the current call frame.
    pub frame : CallFrame,
    frame_stack : Vec<CallFrame>,
    pub chunk : Chunk,
    chunk_size : usize,
    pub register : HashMap<u16, Instance>,
    pub stack: Vec<Instance>
}

impl VM {

    pub fn new() -> VM {
        VM {
            class_registry: Default::default(),
            frame: CallFrame {
                register_offset: 0,
                stack_offset: 0,
                ip: 0
            },
            frame_stack: vec![],
            chunk: Chunk::new(),
            chunk_size: 0,
            register: Default::default(),
            stack: vec![]
        }
    }

    pub fn run(mut self) -> VM {
        self.chunk_size = self.chunk.op_codes.len();

        let mut pt = 0;
        loop {
            let op = self.chunk.get(pt);
            println!("Position: {}", pt);

            match op {
                Some(code) => {
                    match code {
                        OpCode::Get(index) => self.push_stack(&u16::from(*index)),
                        OpCode::Set(index) => self.pop_stack(&u16::from(*index)),
                        OpCode::Add => self.add_operands(),
                        OpCode::Subtract => self.subtract_operands(),
                        OpCode::Multiply => self.multiply_operands(),
                        OpCode::Divide => self.divide_operands(),
                        _ => panic!("Unknown OpCode!")
                    }
                }
                None => break
            }
            pt = pt + 1;
        }

        return self
    }

    fn push_stack(&mut self, index : &u16) {
        let instance = self.register.get(index);
        match instance {
            Some(thing) => self.stack.push(thing.clone().to_owned()),
            None => {panic!("Register slot {} was empty. Aborting program", index)}
        }
    }

    fn pop_stack(&mut self, index: &u16) {
        match self.stack.pop() {
            Some(instance) => self.register.insert(*index, instance),
            None => panic!("The stack was empty!")
        };
    }

    fn add_operands(&mut self) {
        let right = self.stack.pop();
        let left = self.stack.pop();

        if let (Some(left_i), Some(right_i)) = (left, right) {
            match (left_i, right_i) {
                (Int16(left_num), Int16(right_num)) => {
                    self.stack.push(Int16(left_num + right_num))
                }
                _ => panic!("The operands cannot be added!")
            }
        }
    }

    fn subtract_operands(&mut self) {
        let right = self.stack.pop();
        let left = self.stack.pop();

        if let (Some(left_i), Some(right_i)) = (left, right) {
            match (left_i, right_i) {
                (Int16(left_num), Int16(right_num)) => {
                    self.stack.push(Int16(left_num - right_num))
                }
                _ => panic!("The operands cannot be added!")
            }
        }
    }

    fn multiply_operands(&mut self) {
        let right = self.stack.pop();
        let left = self.stack.pop();

        if let (Some(left_i), Some(right_i)) = (left, right) {
            match (left_i, right_i) {
                (Int16(left_num), Int16(right_num)) => {
                    self.stack.push(Int16(left_num * right_num))
                }
                _ => panic!("The operands cannot be added!")
            }
        }
    }

    fn divide_operands(&mut self) {
        let right = self.stack.pop();
        let left = self.stack.pop();

        if let (Some(left_i), Some(right_i)) = (left, right) {
            match (left_i, right_i) {
                (Int16(left_num), Int16(right_num)) => {
                    self.stack.push(Int16(left_num / right_num))
                }
                _ => panic!("The operands cannot be added!")
            }
        }
    }

    pub fn get_current_result(&mut self) -> Instance {
        return match self.stack.pop() {
            Some(instance) => instance,
            None => panic!("The stack was empty!")
        }
    }
}

/*
Holds the current offset in the registry of the call frame as well as some
other useful information.
*/
pub struct CallFrame {
    register_offset: usize,
    stack_offset: usize,
    ip: usize,
}

impl CallFrame {
    pub fn current_position(&self) -> usize {
        return self.ip
    }
}

pub enum ExecutionResult {
    OK,
    ERR,
    END
}
