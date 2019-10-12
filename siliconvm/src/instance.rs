use std::rc::Rc;
use crate::opcode::Chunk;
use std::cell::RefCell;
use std::fmt::{Display, Formatter, Error};
use std::fmt;

// Represents instances created at runtime
#[derive(Clone, Debug)]
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
    Str(Rc<String>),
    Array(Rc<RefCell<Vec<Instance>>>),
    //Represents a custom class instance.
    //ClassInstance(Box<ClassInstance>),
    //Represents a class object.
    //Class(Box<Class>)
    //Represents a function.
    Func(Rc<Function>)
}

impl Display for Instance {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        return match self {
            Instance::Bool(boolean) => write!(f, "{}", boolean),
            Instance::Byte(byte) => write!(f, "{}b", byte),
            Instance::UByte(ubyte) => write!(f, "{}ub", ubyte),
            Instance::Int16(int16) => write!(f, "{}i16", int16),
            Instance::UInt16(uint16) => write!(f, "{}u16", uint16),
            Instance::Int32(int32) => write!(f, "{}i32", int32),
            Instance::UInt32(uint32) => write!(f, "{}ui32", uint32),
            Instance::Int64(int64) => write!(f, "{}i64", int64),
            Instance::UInt64(uint64) => write!(f, "{}u64", uint64),
            Instance::Int128(int128) => write!(f, "{}i128", int128),
            Instance::UInt128(uint128) => write!(f, "{}u128", uint128),
            Instance::Float32(float32) => write!(f, "{}f32", float32),
            Instance::Float64(float64) => write!(f, "{}f64", float64),
            Instance::Char(character) => write!(f, "'{}'", character),
            Instance::Str(string) => write!(f, "\"{}\"", string),
            Instance::Array(array) => {
                let mut array_string = "[".to_string();
                let borrowed = array.borrow_mut();

                if !borrowed.is_empty() {
                    for i in 0..borrowed.len() {
                        match borrowed.get(i) {
                            Some(instance) => {
                                let item_string = format!("{}", instance);
                                array_string.push_str(item_string.as_str());
                                if i != borrowed.len() - 1 {
                                    array_string.push_str(", ")
                                }
                            }
                            None => panic!("Could not format array!")
                        }

                    }
                }

                write!(f, "{}]", array_string)
            },
            Instance::Func(func) => write!(f, "<function>{}", ""),
        };
    }
}

pub struct Type {
    canonical_name: String
}

impl Type {
    pub fn new(canonical_name: String) -> Type {
        Type {
            canonical_name
        }
    }
}

impl Type {

    pub fn is(&self, instance: Instance) -> bool {
        self.canonical_name == match instance {
            Instance::Bool(_) => "silicon.lang.Boolean",
            Instance::Byte(_) => "silicon.lang.Byte",
            Instance::UByte(_) => "silicon.lang.UByte",
            Instance::Int16(_) => "silicon.lang.Int16",
            Instance::UInt16(_) => "silicon.lang.UInt16",
            Instance::Int32(_) => "silicon.lang.Int32",
            Instance::UInt32(_) => "silicon.lang.UInt32",
            Instance::Int64(_) => "silicon.lang.Int64",
            Instance::UInt64(_) => "silicon.lang.UInt64",
            Instance::Int128(_) => "silicon.lang.Int128",
            Instance::UInt128(_) => "silicon.lang.UInt128",
            Instance::Float32(_) => "silicon.lang.Float32",
            Instance::Float64(_) => "silicon.lang.Float64",
            Instance::Char(_) => "silicon.lang.Char",
            Instance::Str(_) => "silicon.lang.String",
            Instance::Array(_) => "silicon.lang.Array",
            Instance::Func(_) => "silicon.lang.Func",
            _ => ""
        }.to_string() || self.canonical_name == "silicon.lang.Object".to_string()
    }
}

#[derive(Debug)]
pub struct Variable {
    pub(crate) is_const: bool,
    pub(crate) stored: Instance
}

impl Variable {

    pub(crate) fn new(is_const: bool, stored: Instance) -> Variable {
        Variable {
            is_const,
            stored
        }
    }
}

#[derive(Clone, Debug)]
pub struct Function {
    pub(crate) arity: u8,
    pub(crate) chunk: Rc<Chunk>
}

impl Function {
    pub(crate) fn new(args: u8, chunk: Chunk) -> Function {
        Function {
            arity: args,
            chunk: Rc::new(chunk)
        }
    }
}