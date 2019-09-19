// OpCode instructions. All instructions should be 4 bytes at the most.
pub enum OpCode {
    /*
    Tells the VM to pull an instance from the register at the specified
    location and move it to the stack. X variant is used when the
    location is greater than 255.
    */
    Get(u8),
    GetX(u16),
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
    Pops the top value off of the stack and pushes it to the register at
    the specified location. X variant is used when the location is
    greater than 255.
    */
    Set(u8),
    SetX(u16),
}

pub struct Chunk {
    pub op_codes : Vec<OpCode>,
    pub is_locked : bool
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk {
            op_codes: vec![],
            is_locked: false
        }
    }

    pub fn write(&mut self, op : OpCode) {
        if self.is_locked {
            panic!("Attempted to write to locked chunk!")
        }
        self.op_codes.push(op)
    }

    pub fn lock(&mut self) {
        self.is_locked = true;
    }

    pub fn get(&self, pt : usize) -> Option<&OpCode> {
        println!("Getting an OpCode!");
        return self.op_codes.get(pt)
    }
}