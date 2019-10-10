use std::collections::HashMap;
use crate::instance::Instance;
use crate::runtime::Register;

// OpCode instructions. All instructions should be 4 bytes at the most.
#[derive(Debug)]
pub enum OpCode {
    GetTrue,
    GetFalse,
    Get(bool, u16),
    Set(u16),
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
    IntNegate,
    LogicNegate,
    Less,
    Greater,
    LessOrEq,
    GreaterOrEq,
    Eq,
    NotEq,
    Concat,
    Jump(bool, u16),
    Call(u16),
    Args(u8),
    Return(bool),
    InitArray(u16),
    IndexGet,
    IndexSet,
    EnterScope(u16),
    ExitScope,
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
        self.const_table.set(index,constant);
    }

    pub fn set_register_size(&mut self, size: u16) {
        self.register_size = size
    }

    pub fn lock(&mut self) {
        self.is_locked = true;
    }

    pub fn get(&self, pt : usize) -> Option<&OpCode> {
        return self.op_codes.get(pt)
    }
}