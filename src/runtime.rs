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
use std::convert::TryInto;

pub struct VM {
    class_registry: HashMap<Sym, Instance>,
    // Represents the current call frame.
    pub frame: CallFrame,
    frame_stack: Vec<CallFrame>,
    pub chunk: Chunk,
    chunk_size: usize,
    pub register: HashMap<u16, Instance>,
    pub stack: Vec<Instance>,
    pub jump_table: HashMap<u16, usize>,
    pub pc : usize
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
            stack: vec![],
            jump_table: Default::default(),
            pc: 0
        }
    }

    pub fn run(mut self) -> VM {
        self.chunk_size = self.chunk.op_codes.len();

        loop {
            let op = self.chunk.get(self.pc);
            println!("Position: {}", self.pc);

            match op {
                Some(code) => {
                    match code {
                        OpCode::Get(index) => self.push_stack(&u16::from(*index)),
                        OpCode::Set(index) => self.pop_stack(&u16::from(*index)),
                        OpCode::Add => self.add_operands(),
                        OpCode::Subtract => self.subtract_operands(),
                        OpCode::Multiply => self.multiply_operands(),
                        OpCode::Divide => self.divide_operands(),
                        OpCode::Power => self.pow_operands(),
                        OpCode::IntNegate => self.negate_operand(),
                        OpCode::LogicNegate => self.logic_negate_operand(),
                        OpCode::Jump(value, index) => if !value {self.jump(*index); continue} else if self.try_jump(*index) {continue},
                        OpCode::Blank => {},
                        _ => panic!("Unknown OpCode!")
                    }
                }
                None => break
            }
            self.pc += 1;
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

    fn pow_operands(&mut self) {
        let right = self.stack.pop();
        let left = self.stack.pop();

        if let (Some(left_i), Some(right_i)) = (left, right) {
            match (left_i, right_i) {
                (Int16(left_num), Int16(right_num)) => self.stack.push(Int16(left_num.pow(right_num.try_into().unwrap()))),
                _ => panic!("The operands cannot be added!")
            }
        }
    }

    fn negate_operand(&mut self) {
        let operand = self.stack.pop();
        if let Some(operand_i) = operand {
            match operand_i {
                Int16(num) => self.stack.push(Int16(-num)),
                _ => panic!("The operand cannot be negated!")
            }
        }
    }

    fn logic_negate_operand(&mut self) {
        let operand = self.stack.pop();
        if let Some(operand_i) = operand {
            match operand_i {
                Bool(value) => self.stack.push(Bool(!value)),
                _ => panic!("The operand cannot be negated!")
            }
        }
    }

    fn try_jump(&mut self, jump_index: u16) -> bool {
        let should_jump = !self.test_logic();
        if should_jump {
            self.jump(jump_index);
            return true
        }
        return false
    }

    fn jump(&mut self, jump_index: u16) {
        match self.jump_table.get(&jump_index) {
            Some(jump_point) => {self.pc = *jump_point; },
            None => panic!("Jump point does not exist")
        }
    }

    fn test_logic(&mut self) -> bool {
        let cond = self.stack.pop();
        if let Some(instance) = cond {
            return match instance {
                Bool(value) => value.clone(),
                _ => panic!()
            }
        }
        panic!()
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

    pub fn advance(&mut self) {
        self.ip += 1;
    }
}

pub enum ExecutionResult {
    OK,
    ERR,
    END
}
