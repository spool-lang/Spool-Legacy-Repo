use std::rc::Rc;
use std::collections::HashMap;
use std::slice::Chunks;
use crate::opcode::{OpCode, Chunk};
use crate::instance::{
    Instance,
    Instance::*
};
use std::convert::TryInto;
use crate::runtime::InstructionResult::{Return, Continue, ReturnWith, ExitScope};
use std::cell::RefCell;
use crate::string_pool::StringPool;

pub struct VM {
    class_registry: HashMap<String, Instance>,
    pub string_pool: StringPool,
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
            string_pool: StringPool::new(),
            chunk_size: 0,
            register: Register::new(true),
            stack: vec![],
            pc: 0,
            jumped: false,
        }
    }

    pub fn execute_chunk(&mut self, chunk: Rc<Chunk>, frame: Rc<RefCell<CallFrame>>) -> InstructionResult {
        self.chunk_size = chunk.register_size as usize;
        loop {
            let op = chunk.get(self.pc);
            match op {
                Some(code) => {
                    let result = self.execute_instruction(code, Rc::clone(&chunk), Rc::clone(&frame));
                    match result {
                        Continue => {},
                        Return | ExitScope => return result,
                        ReturnWith(instance) => return ReturnWith(instance),
                    }
                },
                None => return Return
            }
            if !self.jumped {self.pc += 1}
            self.jumped = false
        }
    }

    pub fn execute_instruction(&mut self, op_code: &OpCode, chunk: Rc<Chunk>, frame: Rc<RefCell<CallFrame>>) -> InstructionResult {
        match op_code {
            OpCode::GetTrue => self.stack.push(Bool(true)),
            OpCode::GetFalse => self.stack.push(Bool(false)),
            OpCode::Get(get_const, index) => self.push_stack(*index, *get_const, chunk, frame),
            OpCode::Set(index) => self.pop_stack(*index, chunk, frame),
            OpCode::Add => self.add_operands(frame.borrow().stack_offset),
            OpCode::Subtract => self.subtract_operands(frame.borrow().stack_offset),
            OpCode::Multiply => self.multiply_operands(frame.borrow().stack_offset),
            OpCode::Divide => self.divide_operands(frame.borrow().stack_offset),
            OpCode::Power => self.pow_operands(frame.borrow().stack_offset),
            OpCode::IntNegate => self.negate_operand(frame.borrow().stack_offset),
            OpCode::LogicNegate => self.logic_negate_operand(frame.borrow().stack_offset),
            OpCode::Less => self.compare_operand_size(false, false, frame.borrow().stack_offset),
            OpCode::LessOrEq => self.compare_operand_size(false, true, frame.borrow().stack_offset),
            OpCode::Greater => self.compare_operand_size(true, false, frame.borrow().stack_offset),
            OpCode::GreaterOrEq => self.compare_operand_size(true, true, frame.borrow().stack_offset),
            OpCode::Eq => self.equate_operands(false, frame.borrow().stack_offset),
            OpCode::NotEq => self.equate_operands(true, frame.borrow().stack_offset),
            OpCode::Concat => self.concat(frame.borrow().stack_offset),
            OpCode::Jump(value, index) => if !value {self.jump(*index, chunk); self.jumped = true} else if self.try_jump(*index, chunk, frame.borrow().stack_offset) {self.jumped = true},
            OpCode::Call(args) => self.call(*args, frame),
            OpCode::Args(num) => self.add_arguments(*num),
            OpCode::Return(return_instance) => if *return_instance { return ReturnWith(self.get_stack_top(frame.borrow().stack_offset)) } else { return Return }
            OpCode::InitArray(size) => self.make_array(*size, frame.borrow().stack_offset),
            OpCode::IndexGet => self.index_get(frame.borrow().stack_offset),
            OpCode::IndexSet => self.index_set(frame.borrow().stack_offset),
            OpCode::EnterScope(size) => self.enter_scope(*size, frame),
            OpCode::ExitScope => self.exit_scope(frame),
            OpCode::Print => println!("{}", self.get_stack_top(frame.borrow().stack_offset)),
        };
        return Continue
    }

    fn push_stack(&mut self, index: u16, get_const: bool, chunk: Rc<Chunk>, frame: Rc<RefCell<CallFrame>>) {
        let instance = if get_const {
            chunk.get_const(index)
        } else {
            self.register.get((&index + &frame.borrow().register_access_offset))
        };
        self.stack.push(instance);
    }

    fn pop_stack(&mut self, index: u16, chunk: Rc<Chunk>, frame: Rc<RefCell<CallFrame>>) {
        let register_offset = frame.borrow().register_declare_offset;
        let instance = self.get_stack_top(frame.borrow().stack_offset);
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
        for index in 0..count {
            let next_arg = args.pop();
            self.register.set((index + offset as u8) as u16, next_arg.unwrap())
        }
        //println!("Register after: {:?}", self.register);
        //println!("Stack after: {:?}", self.stack);
    }

    pub fn call(&mut self, args: u16, previous_frame: Rc<RefCell<CallFrame>>) {
        let option = self.stack.pop();
        if let Some(Func(func)) = option {
            let chunk = Rc::clone(&func.chunk);
            let stack_offset = self.stack.len();
            let register_offset = (self.chunk_size + previous_frame.borrow().register_access_offset as usize) as u16;
            //println!("Register offset: {}", register_offset);
            let new_frame = CallFrame::new_with_offset((register_offset) as u16, (register_offset) as u16, stack_offset);
            let previous_pc = self.pc;
            self.pc = 0;
            let result = self.execute_chunk(chunk, Rc::new(RefCell::new(new_frame)));
            self.stack.truncate(stack_offset);
            //println!("Register before: {:?}", self.register);
            self.register.truncate(register_offset);
            //println!("Register after: {:?}", self.register);
            self.pc = previous_pc;
            match result {
                InstructionResult::ReturnWith(instance) => self.stack.push(instance),
                _ => {}
            }
        }
    }

    pub fn make_array(&mut self, array_size: u16, stack_offset: usize) {
        let mut array : Vec<Instance> = self.split_stack(array_size as usize, stack_offset);
        self.stack.push(Array(Rc::new(RefCell::new(array))));
    }

    pub fn index_get(&mut self, stack_offset: usize) {
        let index = self.get_stack_top(stack_offset);
        let indexable = self.get_stack_top(stack_offset);

        match indexable {
            Array(vec) => {
                let mut index_num = 0;
                match index {
                    Byte(num) => index_num = num as usize,
                    UByte(num) => index_num = num as usize,
                    Int16(num) => index_num = num as usize,
                    UInt16(num) => index_num = num as usize,
                    _ => panic!("Invalid array index.")
                };
                match vec.borrow().get(index_num) {
                    Some(instance) => self.stack.push(instance.to_owned()),
                    None => panic!("No instance found at the given index.")
                }
            },
            Str(string) => {
                let mut index_num = 0;
                match index {
                    Byte(num) => index_num = num as usize,
                    UByte(num) => index_num = num as usize,
                    Int16(num) => index_num = num as usize,
                    UInt16(num) => index_num = num as usize,
                    _ => panic!("Invalid string index.")
                };
                match string.chars().nth(index_num) {
                    Some(c) => self.stack.push(Char(c)),
                    None => panic!("Invalid string index.")
                }
            }
            _ => panic!("The instance is not indexable!")
        }
    }

    pub fn index_set(&mut self, stack_offset: usize) {
        let item = self.get_stack_top(stack_offset);
        let index = self.get_stack_top(stack_offset);
        let array = self.get_stack_top(stack_offset);

        match array {
            Array(mut vec) => {
                let mut index_num = 0;
                match index {
                    Byte(num) => index_num = num as usize,
                    UByte(num) => index_num = num as usize,
                    Int16(num) => index_num = num as usize,
                    UInt16(num) => index_num = num as usize,
                    _ => panic!("Invalid array index.")
                };
                vec.borrow_mut().remove(index_num);
                vec.borrow_mut().insert(index_num, item)
            },
            _ => panic!("The instance is not indexable!")
        }
    }

    pub fn concat(&mut self, stack_offset: usize) {
        let right = self.get_stack_top(stack_offset);
        let left = self.get_stack_top(stack_offset);

        match left {
            Str(string) => {
                let new_string = match right {
                    Str(s) => format!("{}{}", string, s),
                    Char(c) => format!("{}{}", string, c),
                    _ => format!("{}{}", string, right)
                };
                let pooled_string = self.string_pool.pool_string(new_string);
                self.stack.push(Str(pooled_string))
            }
            _ => panic!("Cannot concat operands!")
        }
    }

    pub fn enter_scope(&mut self, additional_size: u16, frame: Rc<RefCell<CallFrame>>) {
        frame.borrow_mut().scope_allocations.push(additional_size);
        frame.borrow_mut().register_declare_offset += additional_size
    }

    pub fn exit_scope(&mut self, frame: Rc<RefCell<CallFrame>>) {
        let allocation = frame.borrow_mut().scope_allocations.pop();
        if let Some(amount) = allocation {
            self.register.truncate(self.register.size - amount);
            frame.borrow_mut().register_declare_offset -= amount;
            return;
        }
        println!("Exited inner scope that did not exist.")
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

    pub fn split_stack(&mut self, amount: usize, stack_offset: usize) -> Vec<Instance> {
        let mut vec : Vec<Instance> = vec![];
        for i in 0..amount {
            let next = self.get_stack_top(stack_offset);
            vec.push(next)
        }
        vec.reverse();
        return vec
    }
}

/*
Holds the current offset in the registry of the call frame as well as some
other useful information.
*/
pub struct CallFrame {
    register_access_offset: u16,
    register_declare_offset: u16,
    stack_offset: usize,
    scope_allocations: Vec<u16>
}

impl CallFrame {
    pub fn new() -> CallFrame {
        CallFrame {
            register_access_offset: 0,
            register_declare_offset: 0,
            stack_offset: 0,
            scope_allocations: vec![]
        }
    }

    pub fn new_with_offset(register_access_offset: u16, register_declare_offset: u16,stack_offset: usize) -> CallFrame {
        CallFrame {
            register_access_offset,
            register_declare_offset,
            stack_offset,
            scope_allocations: vec![]
        }
    }
}

#[derive(Debug)]
pub struct Register {
    internal: HashMap<u16, Instance>,
    size: u16,
    flag: bool
}

impl Register {
    pub fn new(flag: bool) -> Register {
        Register {
            internal: Default::default(),
            size: 0,
            flag: flag
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
            Some(instance) => {
                return instance.to_owned()
            },
            None => panic!("Register slot `{}` was empty", index)
        };
    }

    pub fn truncate(&mut self, to_size: u16) {
        if to_size == self.size || self.size == 0 {
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

pub enum InstructionResult{
    Continue,
    Return,
    ReturnWith(Instance),
    ExitScope,
}