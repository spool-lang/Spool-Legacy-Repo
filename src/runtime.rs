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
                        OpCode::GetTrue => self.stack.push(Bool(true)),
                        OpCode::GetFalse => self.stack.push(Bool(false)),
                        OpCode::Get(get_const, index) => self.push_stack(*index, *get_const),
                        OpCode::Set(index) => self.pop_stack(*index),
                        OpCode::Add => self.add_operands(),
                        OpCode::Subtract => self.subtract_operands(),
                        OpCode::Multiply => self.multiply_operands(),
                        OpCode::Divide => self.divide_operands(),
                        OpCode::Power => self.pow_operands(),
                        OpCode::IntNegate => self.negate_operand(),
                        OpCode::LogicNegate => self.logic_negate_operand(),
                        OpCode::Less => self.compare_operand_size(false, false),
                        OpCode::LessOrEq => self.compare_operand_size(false, true),
                        OpCode::Greater => self.compare_operand_size(true, false),
                        OpCode::GreaterOrEq => self.compare_operand_size(true, true),
                        OpCode::Eq => self.equate_operands(false),
                        OpCode::NotEq => self.equate_operands(true),
                        OpCode::Jump(value, index) => if !value {self.jump(*index); continue} else if self.try_jump(*index) {continue},
                        OpCode::Print => println!("And the value is... {:#?}", self.get_current_result()),
                        _ => panic!("Unknown OpCode!")
                    }
                }
                None => break
            }
            self.pc += 1;
        }

        return self
    }

    fn push_stack(&mut self, index: u16, get_const: bool) {
        let instance = if get_const {self.chunk.const_table.get(&index)} else { self.register.get(&index) };
        match instance {
            Some(thing) => self.stack.push(thing.clone().to_owned()),
            None => {panic!("Register slot {} was empty. Aborting program", index)}
        }
    }

    fn pop_stack(&mut self, index: u16) {
        match self.stack.pop() {
            Some(instance) => self.register.insert(index, instance),
            None => panic!("The stack was empty!")
        };
    }

    fn add_operands(&mut self) {
        let right = self.stack.pop();
        let left = self.stack.pop();

        if let (Some(left_i), Some(right_i)) = (left, right) {
            match (left_i, right_i) {
                (Byte(left_num), Byte(right_num)) => self.stack.push(Byte(left_num + right_num)),
                (UByte(left_num), UByte(right_num)) => self.stack.push(UByte(left_num + right_num)),
                (Int16(left_num), Int16(right_num)) => self.stack.push(Int16(left_num + right_num)),
                (UInt16(left_num), UInt16(right_num)) => self.stack.push(UInt16(left_num + right_num)),
                _ => panic!("The operands cannot be added!")
            }
        }
    }

    fn subtract_operands(&mut self) {
        let right = self.stack.pop();
        let left = self.stack.pop();

        if let (Some(left_i), Some(right_i)) = (left, right) {
            match (left_i, right_i) {
                (Byte(left_num), Byte(right_num)) => self.stack.push(Byte(left_num - right_num)),
                (UByte(left_num), UByte(right_num)) => self.stack.push(UByte(left_num - right_num)),
                (Int16(left_num), Int16(right_num)) => self.stack.push(Int16(left_num - right_num)),
                (UInt16(left_num), UInt16(right_num)) => self.stack.push(UInt16(left_num - right_num)),
                _ => panic!("The operands cannot be added!")
            }
        }
    }

    fn multiply_operands(&mut self) {
        let right = self.stack.pop();
        let left = self.stack.pop();

        if let (Some(left_i), Some(right_i)) = (left, right) {
            match (left_i, right_i) {
                (Byte(left_num), Byte(right_num)) => self.stack.push(Byte(left_num * right_num)),
                (UByte(left_num), UByte(right_num)) => self.stack.push(UByte(left_num * right_num)),
                (Int16(left_num), Int16(right_num)) => self.stack.push(Int16(left_num * right_num)),
                (UInt16(left_num), UInt16(right_num)) => self.stack.push(UInt16(left_num * right_num)),
                _ => panic!("The operands cannot be added!")
            }
        }
    }

    fn divide_operands(&mut self) {
        let right = self.stack.pop();
        let left = self.stack.pop();

        if let (Some(left_i), Some(right_i)) = (left, right) {
            match (left_i, right_i) {
                (Byte(left_num), Byte(right_num)) => self.stack.push(Byte(left_num / right_num)),
                (UByte(left_num), UByte(right_num)) => self.stack.push(UByte(left_num / right_num)),
                (Int16(left_num), Int16(right_num)) => self.stack.push(Int16(left_num / right_num)),
                (UInt16(left_num), UInt16(right_num)) => self.stack.push(UInt16(left_num / right_num)),
                _ => panic!("The operands cannot be added!")
            }
        }
    }

    fn pow_operands(&mut self) {
        let right = self.stack.pop();
        let left = self.stack.pop();

        if let (Some(left_i), Some(right_i)) = (left, right) {
            match (left_i, right_i) {
                (Byte(left_num), Byte(right_num)) => self.stack.push(Byte(left_num.pow(right_num.try_into().unwrap()))),
                (UByte(left_num), UByte(right_num)) => self.stack.push(UByte(left_num.pow(right_num.try_into().unwrap()))),
                (Int16(left_num), Int16(right_num)) => self.stack.push(Int16(left_num.pow(right_num.try_into().unwrap()))),
                (UInt16(left_num), UInt16(right_num)) => self.stack.push(UInt16(left_num.pow(right_num.try_into().unwrap()))),
                _ => panic!("The operands cannot be added!")
            }
        }
    }

    fn negate_operand(&mut self) {
        let operand = self.stack.pop();
        if let Some(operand_i) = operand {
            match operand_i {
                Byte(num) => self.stack.push(Byte(-num)),
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

    fn compare_operand_size(&mut self, flip_operator: bool, equal: bool) {
        let right = self.stack.pop();
        let left = self.stack.pop();
        if let (Some(left_i), Some(right_i)) = (left, right) {
            match (left_i, right_i) {
                (Byte(left_num), Byte(right_num)) => {
                    let mut cond = left_num < right_num;
                    if flip_operator {cond = !cond}
                    if equal {cond = cond || (left_num == right_num)}
                    self.stack.push(Bool(cond))
                },
                (UByte(left_num), UByte(right_num)) => {
                    let mut cond = left_num < right_num;
                    if flip_operator {cond = !cond}
                    if equal {cond = cond || (left_num == right_num)}
                    self.stack.push(Bool(cond))
                },
                (Int16(left_num), Int16(right_num)) => {
                    let mut cond = left_num < right_num;
                    if flip_operator {cond = !cond}
                    if equal {cond = cond || (left_num == right_num)}
                    self.stack.push(Bool(cond))
                },
                (UInt16(left_num), UInt16(right_num)) => {
                    let mut cond = left_num < right_num;
                    if flip_operator {cond = !cond}
                    if equal {cond = cond || (left_num == right_num)}
                    self.stack.push(Bool(cond))
                },
                _ => panic!("Cannot compare the size of the operands!")
            }
        }
    }

    fn equate_operands(&mut self, negate: bool) {
        let right = self.stack.pop();
        let left = self.stack.pop();
        if let (Some(left_i), Some(right_i)) = (left, right) {
            match (left_i, right_i) {
                (Int16(left_num), Int16(right_num)) => self.stack.push(Bool((left_num == right_num) && !negate)),
                (UInt16(left_num), UInt16(right_num)) => self.stack.push(Bool((left_num == right_num) && !negate)),
                (Bool(left_val), Bool(right_val)) => self.stack.push(Bool((left_val == right_val) && !negate)),
                _ => self.stack.push(Bool(false))
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
        match self.chunk.jump_table.get(&jump_index) {
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
