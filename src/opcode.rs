// OpCode instructions. All instructions should be 4 bytes at the most.
pub enum OpCode {
    /*
    Pulls instances from the registry at the specified location or from
    the result slot and puts them onto the operating stack.
    */
    Get(u8),
    GetX(u16),
    /*
    Operates on the left and right operands and places the result in the
    result slot. If either the left or right slot is empty, the desired
    left or right value was most likely a result of the previous
    operation and will be pulled from the result slot.
    */
    Add,
    Subtract,
    Multiply,
    Divide,
    /*
    Pushes the Instance in the result slot back into the registry at the
    specified location.
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