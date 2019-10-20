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
    Array(Rc<RefCell<Vec<Instance>>>, Rc<Type>),
    //Represents a custom class instance.
    //ClassInstance(Box<ClassInstance>),
    //Represents a class object.
    //Class(Box<Class>)
    //Represents a function.
    Func(Rc<Function>),
    Void
}

impl Instance {

    pub fn get_canonical_name(&self) -> Rc<String> {
        Rc::new(
            match self {
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
                Instance::Array(_, _) => "silicon.lang.Array",
                Instance::Func(_) => "silicon.lang.Func",
                Instance::Void => "silicon.lang.Void",
                _ => ""
            }.to_string()
        )
    }
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
            Instance::Array(array, _type) => {
                let mut array_string = format!("{}[", _type.get_canonical_name());
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
            Instance::Void => write!(f, "{}", "void")
        };
    }
}

#[derive(Debug)]
pub struct Type {
    pub(crate) canonical_name: Rc<String>,
    is_generic: bool,
    type_args: Vec<Rc<Type>>
}

impl Type {
    pub fn new(canonical_name: Rc<String>) -> Type {
        Type {
            canonical_name,
            is_generic: false,
            type_args: vec![]
        }
    }

    pub fn new_generic(canonical_name: Rc<String>) -> Type {
        Type {
            canonical_name,
            is_generic: true,
            type_args: vec![]
        }
    }

    pub fn get_canonical_name(&self) -> Rc<String> {
        let mut actual_name = format!("{}", self.canonical_name);
        if self.type_args.len() > 0 {
            actual_name.push_str("<");
            let mut i: usize = 0;
            for _type in &self.type_args {
                let type_name = format!("{}", _type.get_canonical_name());
                actual_name.push_str(type_name.as_str());
                if i < *&self.type_args.len() {
                    actual_name.push(',')
                }
            }
            actual_name.push_str(">");
        }

        Rc::new(actual_name)
    }

    pub fn reify(&self, type_args: Vec<Rc<Type>>) {
        if self.is_generic {
            if type_args.len() == self.type_args.len() {
                Type {
                    canonical_name: Rc::clone(&self.canonical_name),
                    is_generic: false,
                    type_args
                };
            }
            panic!("Error during reification process.")
        }
        panic!("Attempted to reify non-generic type.")

    }
}

impl Type {

    pub fn is(&self, instance: &Instance) -> bool {
        &*self.canonical_name == &*"silicon.lang.Object".to_string() || &*self.canonical_name == &*instance.get_canonical_name()
    }
}

#[derive(Debug)]
pub struct Variable {
    pub(crate) is_const: bool,
    pub(crate) stored: Instance,
    pub(crate) _type: Rc<Type>
}

impl Variable {

    pub(crate) fn new(is_const: bool, stored: Instance, _type: Rc<Type>) -> Variable {
        Variable {
            is_const,
            stored,
            _type
        }
    }

    pub(crate) fn set(&mut self, instance: Instance) {
        if self.is_const {
            panic!("Attempted to set constant variable!")
        }

        if self._type.is(&instance) {
            self.stored = instance;
            return;
        }
        panic!("Type mismatch!")
    }
}

#[derive(Clone, Debug)]
pub struct Function {
    pub(crate) arity: u8,
    pub(crate) params: Vec<Rc<Type>>,
    pub(crate) return_type: Rc<Type>,
    pub(crate) chunk: Rc<Chunk>
}

impl Function {
    pub(crate) fn new(arity: u8, params: Vec<Rc<Type>>, return_type: Rc<Type>,chunk: Chunk) -> Function {
        Function {
            arity,
            params,
            return_type,
            chunk: Rc::new(chunk)
        }
    }
}