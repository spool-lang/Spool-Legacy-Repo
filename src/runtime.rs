use std::rc::Rc;
use std::collections::HashMap;
use std::slice::Chunks;
use crate::opcode::{OpCode, Chunk};
use crate::instance::{
    Instance,
    Instance::*
};
use std::convert::TryInto;

pub struct VM {
    class_registry: HashMap<String, Instance>,
    chunk_size: usize,
    pub register: Register,
    pub stack: Vec<Instance>,
    pub pc : usize,
    jumped: bool
}

impl VM {

    pub fn new() -> VM {
        VM {
            class_registry: Default::default(),
            chunk_size: 0,
            register: Register::new(),
            stack: vec![],
            pc: 0,
            jumped: false,
        }
    }

    pub fn run_program(&mut self, chunk: Rc<Chunk>, frame: Rc<CallFrame>) {
        self.chunk_size = chunk.register_size as usize;
        loop {
            let op = chunk.get(self.pc);
            match op {
                Some(code) => self.execute_instruction(code, Rc::clone(&chunk), Rc::clone(&frame)),
                None => return
            }
            if !self.jumped {self.pc += 1}
            self.jumped = false
        }
    }

    pub fn execute_instruction(&mut self, op_code: &OpCode, chunk: Rc<Chunk>, frame: Rc<CallFrame>) {
        match op_code {
            OpCode::GetTrue => self.stack.push(Bool(true)),
            OpCode::GetFalse => self.stack.push(Bool(false)),
            OpCode::Get(get_const, index) => self.push_stack(*index, *get_const, chunk, frame),
            OpCode::Set(index) => self.pop_stack(*index, chunk, frame),
            OpCode::Add => self.add_operands(frame.stack_offset),
            OpCode::Subtract => self.subtract_operands(frame.stack_offset),
            OpCode::Multiply => self.multiply_operands(frame.stack_offset),
            OpCode::Divide => self.divide_operands(frame.stack_offset),
            OpCode::Power => self.pow_operands(frame.stack_offset),
            OpCode::IntNegate => self.negate_operand(frame.stack_offset),
            OpCode::LogicNegate => self.logic_negate_operand(frame.stack_offset),
            OpCode::Less => self.compare_operand_size(false, false, frame.stack_offset),
            OpCode::LessOrEq => self.compare_operand_size(false, true, frame.stack_offset),
            OpCode::Greater => self.compare_operand_size(true, false, frame.stack_offset),
            OpCode::GreaterOrEq => self.compare_operand_size(true, true, frame.stack_offset),
            OpCode::Eq => self.equate_operands(false, frame.stack_offset),
            OpCode::NotEq => self.equate_operands(true, frame.stack_offset),
            OpCode::Jump(value, index) => if !value {self.jump(*index, chunk); self.jumped = true} else if self.try_jump(*index, chunk, frame.stack_offset) {self.jumped = true},
            OpCode::Call => self.call(frame),
            OpCode::Args(num) => self.add_arguments(*num),
            OpCode::Print => println!("And the value is... {:#?}", self.get_stack_top(frame.stack_offset)),
            _ => panic!("Unknown OpCode!")
        }
    }

    fn push_stack(&mut self, index: u16, get_const: bool, chunk: Rc<Chunk>, frame: Rc<CallFrame>) {
        let instance = if get_const {
            chunk.const_table.get(index)
        } else {
            self.register.get((&index + &frame.register_offset))
        };
        self.stack.push(instance);
    }

    fn pop_stack(&mut self, index: u16, chunk: Rc<Chunk>, frame: Rc<CallFrame>) {
        let register_offset = frame.register_offset;
        let instance = self.get_stack_top(frame.stack_offset);
        if index < chunk.register_size {
            self.register.set(index + register_offset, instance);
            return;
        }
    }

    fn add_operands(&mut self, stack_offset: usize) {
        let right = self.get_stack_top(stack_offset);
        let left = self.get_stack_top(stack_offset);

        match (left, right) {
            (Byte(left_num), Byte(right_num)) => self.stack.push(Byte(left_num + right_num)),
            (UByte(left_num), UByte(right_num)) => self.stack.push(UByte(left_num + right_num)),
            (Int16(left_num), Int16(right_num)) => self.stack.push(Int16(left_num + right_num)),
            (UInt16(left_num), UInt16(right_num)) => self.stack.push(UInt16(left_num + right_num)),
            _ => panic!("The operands cannot be added!")
        }
    }

    fn subtract_operands(&mut self, stack_offset: usize) {
        let right = self.get_stack_top(stack_offset);
        let left = self.get_stack_top(stack_offset);

        match (left, right) {
            (Byte(left_num), Byte(right_num)) => self.stack.push(Byte(left_num - right_num)),
            (UByte(left_num), UByte(right_num)) => self.stack.push(UByte(left_num - right_num)),
            (Int16(left_num), Int16(right_num)) => self.stack.push(Int16(left_num - right_num)),
            (UInt16(left_num), UInt16(right_num)) => self.stack.push(UInt16(left_num - right_num)),
            _ => panic!("The operands cannot be added!")
        }
    }

    fn multiply_operands(&mut self, stack_offset: usize) {
        let right = self.get_stack_top(stack_offset);
        let left = self.get_stack_top(stack_offset);

        match (left, right) {
            (Byte(left_num), Byte(right_num)) => self.stack.push(Byte(left_num * right_num)),
            (UByte(left_num), UByte(right_num)) => self.stack.push(UByte(left_num * right_num)),
            (Int16(left_num), Int16(right_num)) => self.stack.push(Int16(left_num * right_num)),
            (UInt16(left_num), UInt16(right_num)) => self.stack.push(UInt16(left_num * right_num)),
            _ => panic!("The operands cannot be added!")
        }
    }

    fn divide_operands(&mut self, stack_offset: usize) {
        let right = self.get_stack_top(stack_offset);
        let left = self.get_stack_top(stack_offset);

        match (left, right) {
            (Byte(left_num), Byte(right_num)) => self.stack.push(Byte(left_num / right_num)),
            (UByte(left_num), UByte(right_num)) => self.stack.push(UByte(left_num / right_num)),
            (Int16(left_num), Int16(right_num)) => self.stack.push(Int16(left_num / right_num)),
            (UInt16(left_num), UInt16(right_num)) => self.stack.push(UInt16(left_num / right_num)),
            _ => panic!("The operands cannot be added!")
        }
    }

    fn pow_operands(&mut self, stack_offset: usize) {
        let right = self.get_stack_top(stack_offset);
        let left = self.get_stack_top(stack_offset);

        match (left, right) {
            (Byte(left_num), Byte(right_num)) => self.stack.push(Byte(left_num.pow(right_num.try_into().unwrap()))),
            (UByte(left_num), UByte(right_num)) => self.stack.push(UByte(left_num.pow(right_num.try_into().unwrap()))),
            (Int16(left_num), Int16(right_num)) => self.stack.push(Int16(left_num.pow(right_num.try_into().unwrap()))),
            (UInt16(left_num), UInt16(right_num)) => self.stack.push(UInt16(left_num.pow(right_num.try_into().unwrap()))),
            _ => panic!("The operands cannot be added!")
        }
    }

    fn negate_operand(&mut self, stack_offset: usize) {
        let operand = self.get_stack_top(stack_offset);
        match operand {
            Byte(num) => self.stack.push(Byte(-num)),
            Int16(num) => self.stack.push(Int16(-num)),
            _ => panic!("The operand cannot be negated!")
        }
    }

    fn logic_negate_operand(&mut self, stack_offset: usize) {
        let operand = self.get_stack_top(stack_offset);
        match operand {
            Bool(value) => self.stack.push(Bool(!value)),
            _ => panic!("The operand cannot be negated!")
        }
    }

    fn compare_operand_size(&mut self, flip_operator: bool, equal: bool, stack_offset: usize) {
        let right = self.get_stack_top(stack_offset);
        let left = self.get_stack_top(stack_offset);
        match (left, right) {
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

    fn equate_operands(&mut self, negate: bool, stack_offset: usize) {
        let right = self.get_stack_top(stack_offset);
        let left = self.get_stack_top(stack_offset);
        match (left, right) {
            (Int16(left_num), Int16(right_num)) => self.stack.push(Bool((left_num == right_num) && !negate)),
            (UInt16(left_num), UInt16(right_num)) => self.stack.push(Bool((left_num == right_num) && !negate)),
            (Bool(left_val), Bool(right_val)) => self.stack.push(Bool((left_val == right_val) && !negate)),
            _ => self.stack.push(Bool(false))
        }
    }

    fn try_jump(&mut self, jump_index: u16, chunk: Rc<Chunk>, stack_offset: usize) -> bool {
        let should_jump = !self.test_logic(stack_offset);
        if should_jump {
            self.jump(jump_index, chunk);
            return true
        }
        return false
    }

    fn jump(&mut self, jump_index: u16, chunk: Rc<Chunk>) {
        match chunk.jump_table.get(&jump_index) {
            Some(jump_point) => {self.pc = *jump_point; },
            None => panic!("Jump point does not exist")
        }
    }

    fn test_logic(&mut self, stack_offset: usize) -> bool {
        let cond = self.get_stack_top(stack_offset);
        return match cond {
            Bool(value) => value.clone(),
            _ => panic!()
        };
        panic!()
    }

    pub fn add_arguments(&mut self, count: u8) {
        if count == 0 {
            return;
        }

        //println!("Register before: {:?}", self.register);
        //println!("Stack before: {:?}", self.stack);
        let mut args: Vec<Instance> = self.stack.drain(self.stack.len() - count as usize..).collect();
        let offset = self.chunk_size;
        let mut arg_index = 0;
        for index in 0..count {
            let next_arg = args.pop();
            self.register.set((index + offset as u8) as u16, next_arg.unwrap())
        }
        //println!("Register after: {:?}", self.register);
        //println!("Stack after: {:?}", self.stack);
    }

    pub fn call(&mut self, previous_frame: Rc<CallFrame>) {
        let option = self.stack.pop();
        if let Some(Func(func)) = option {
            let chunk = Rc::clone(&func.chunk);
            let stack_offset = self.stack.len();
            let register_offset = (self.chunk_size + previous_frame.register_offset as usize) as u16;
            //println!("Register offset: {}", register_offset);
            let new_frame = CallFrame::new_with_offset((register_offset) as u16, stack_offset);
            let previous_pc = self.pc;
            self.pc = 0;
            self.run_program(chunk, Rc::new(new_frame));
            self.stack.truncate(stack_offset);
            //println!("Register before: {:?}", self.register);
            self.register.truncate(register_offset);
            //println!("Register after: {:?}", self.register);
            self.pc = previous_pc;
        }
    }

    pub fn get_stack_top(&mut self, stack_offset: usize) -> Instance {
        if self.stack.len() - stack_offset <= 0 {
            panic!("The stack was empty!")
        }

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
    register_offset: u16,
    stack_offset: usize,
}

impl CallFrame {
    pub fn new() -> CallFrame {
        CallFrame {
            register_offset: 0,
            stack_offset: 0,
        }
    }

    pub fn new_with_offset(register_offset: u16, stack_offset: usize) -> CallFrame {
        CallFrame {
            register_offset,
            stack_offset
        }
    }
}

#[derive(Debug)]
pub struct Register {
    internal: HashMap<u16, Instance>,
    size: u16,
}

impl Register {
    pub fn new() -> Register {
        Register {
            internal: Default::default(),
            size: 0
        }
    }

    pub fn set(&mut self, index: u16, instance: Instance) {
        if !(self.internal.contains_key(&index)) {
            self.size = index + 1;
        }
        self.internal.insert(index, instance);
    }

    pub fn get(&self, index: u16) -> Instance {
        match self.internal.get(&index) {
            Some(instance) => {return instance.to_owned()},
            None => panic!("Register slot `{}` was empty.", index)
        };
    }

    pub fn truncate(&mut self, to_size: u16) {
        if to_size == self.size {
            return;
        }

        let mut to_clear = self.size - 1;
        loop {
            self.internal.remove(&to_clear);
            self.size -= 1;
            to_clear = to_clear - 1;
            if to_size == self.size {
                return;
            }
        }
    }
}
