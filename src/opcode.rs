use std::collections::HashMap;
use crate::instance::Instance;
use crate::runtime::Register;

// OpCode instructions. All instructions should be 4 bytes at the most.
#[derive(Debug)]
pub enum OpCode {
    /*
    OpCodes for popping true or false onto the stack since a bool only
    has two possible values.
    */
    GetTrue,
    GetFalse,
    /*
    Tells the VM to pull an instance from the register at the specified
    location and move it to the stack. If the bool is true, it will
    grab from the constants table instead.
    */
    Get(bool, u16),
    /*
    Tells the VM to pop the top two values off of the stack (or just one
    if unary) and perform the specified operation on them.
    */
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
    IntNegate,
    LogicNegate,
    /*
    Pops the top two values on the stack and compares them, producing either
    `true` or `false`
    */
    Less,
    Greater,
    LessOrEq,
    GreaterOrEq,
    Eq,
    NotEq,
    /*
    Pops the top value off of the stack and pushes it to the register at
    the specified location.
    */
    Set(u16),
    /*
    Jumps the to the jump point at the specified index in the jump table.
    Also has a bool, which, if true will pop the top value off of the
    stack and check if it equals 'true' before jumping (if false, it
    won't jump).
    */
    Jump(bool, u16),
    /*
    Calls the current function on the stack.
    */
    Call,
    /*
    Pops the given number of elements off of the top of the stack and puts them into an array.
    */
    // Debug only.
    Print,
}

#[derive(Debug)]
pub struct Chunk {
    pub op_codes: Vec<OpCode>,
    pub is_locked: bool,
    pub jump_table: HashMap<u16, usize>,
    pub const_table: Register,
    pub register_size: u16,
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk {
            op_codes: vec![],
            is_locked: false,
            jump_table: Default::default(),
            const_table: Register::new(),
            register_size: 0
        }
    }

    pub fn write(&mut self, op : OpCode) {
        if self.is_locked {
            panic!("Attempted to write to locked chunk!")
        }
        self.op_codes.push(op)
    }

    pub fn add_const(&mut self, index: u16, constant: Instance) {
        if self.is_locked {
            panic!("Attempted to write to locked chunk!")
        }
        self.const_table.set(index, constant);
    }

    pub fn set_register_size(&mut self, size: u16) {
        self.register_size = size
    }

    pub fn lock(&mut self) {
        self.is_locked = true;
    }

    pub fn get(&self, pt : usize) -> Option<&OpCode> {
        println!("Getting an OpCode!");
        return self.op_codes.get(pt)
    }
}