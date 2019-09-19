use std::rc::Rc;
use string_interner::DefaultStringInterner;
use string_interner::Sym;
use std::collections::HashMap;
use std::slice::Chunks;
use crate::runtime::Instance::Int16;

pub struct VM {
    class_registry : HashMap<Sym, Instance>,
    // Represents the current call frame.
    pub frame : CallFrame,
    frame_stack : Vec<CallFrame>,
    pub chunk : Chunk,
    chunk_size : usize,
    pub register : HashMap<u16, &'static Instance>,
    pub operating_stack : Vec<Instance>
}

impl VM {

    pub fn new() -> VM {
        VM {
            class_registry: Default::default(),
            frame: CallFrame {
                offset: 0,
                operator_offset: 0,
                ip: 0
            },
            frame_stack: vec![],
            chunk: Chunk::new(),
            chunk_size: 0,
            register: Default::default(),
            operating_stack: vec![]
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
                        OpCode::Get(index) => {
                            let instance = self.register.get(&u16::from(*index));
                            match instance {
                                Some(thing) => self.operating_stack.push(thing.clone().to_owned()),
                                None => {panic!()}
                            }
                        }
                        OpCode::Add => {
                            self.add_operands()
                        },
                        _ => panic!("Unknown OpCode!")
                    }
                }
                None => break
            }
            pt = pt + 1;
        }

        return self
    }

    fn add_operands(&mut self) {
        let right = self.operating_stack.pop();
        let left = self.operating_stack.pop();

        if let (Some(left_i), Some(right_i)) = (left, right) {
            match (left_i, right_i) {
                (Int16(left_num), Int16(right_num)) => {
                    self.operating_stack.push(Int16(left_num + right_num))
                }
                _ => {}
            }
        }
    }

    pub fn get_current_result(&mut self) -> Instance {
        return match self.operating_stack.pop() {
            Some(instance) => instance,
            None => panic!()
        }
    }
}

/*
Holds the current offset in the registry of the call frame as well as some
other useful information.
*/
pub struct CallFrame {
    offset : usize,
    operator_offset :usize,
    ip : usize,
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

pub struct Chunk {
    op_codes : Vec<OpCode>,
    is_locked : bool
}

impl Chunk {
    fn new() -> Chunk {
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

    fn lock(&mut self) {
        self.is_locked = true;
    }

    fn get(&self, pt : usize) -> Option<&OpCode> {
        println!("Getting an OpCode!");
        return self.op_codes.get(pt)
    }
}

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
    Push(u8),
    PushX(u16),
}

// Represents instances created at runtime
#[derive(Clone)]
pub enum Instance {
    Bool(bool),
    Byte(i8),
    UByte(u8),
    Int16(i16),
    UInt16(u16),
    Int32(i32),
    UInt32(u32),
    Int64(i64),
    UInt64(u64),
    Int128(i128),
    UInt128(u128),
    //Fixed-point precision.
    //Decimal16(),
    //UDecimal16(),
    //Decimal32(),
    //UDecimal32(),
    //Decimal64(),
    //UDecimal64(),
    //Decimal128(),
    //UDecimal128(),
    Float32(f32),
    Float64(f64),
    //These are commented out for now but I would like to bring in the 'num' crate at some point
    //to introduce these types or make my own.
    //BigInt(),
    //UBigInt(),
    //BigFloat(),
    //BigDecimal(),
    //Complex(),
    Char(char),
    String(Rc<Sym>),
    Array(Vec<Instance>),
    //Represents a custom class instance.
    //CustomInstance(Box<CustomInstance>),
    //Represents a class object.
    //Class(Box<Class>)
}

// Represents a class declared in Silicon code:
pub struct Class {
    canonical_name : &'static str,
    field_info : Vec<FieldInfo>
}

pub struct FieldInfo {
    is_const : bool,
    modifier : AccessModifier,
}

pub enum AccessModifier {
    Public,
    Protected,
    Private,
    Internal
}

// Represents an instance of a class that is not built into the VM.
pub struct CustomInstance {
    class : Class,
    fields : Vec<Field>,
}

pub struct Field {
    field_info : FieldInfo,
    value : Instance,
}
