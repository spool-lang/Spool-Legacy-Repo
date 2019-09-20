use std::rc::Rc;
use string_interner::DefaultStringInterner;
use string_interner::Sym;
use crate::opcode::Chunk;

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
    Str(Rc<Sym>),
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

pub struct Function {
    arity: u8,
    chunk: Chunk
}

impl Function {
    fn get_chunk(&self, arg_count: u8) -> &Chunk {
        if arg_count == self.arity {
            return &self.chunk
        }
        panic!("Argument count does not match the arity!")
    }
}