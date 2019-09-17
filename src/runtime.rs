use std::rc::Rc;
use string_interner::DefaultStringInterner;
use string_interner::Sym;

pub struct VM {

}

// OpCode instructions to follow. All instructions should be 4 bytes at the most.
pub enum OpCode {

}

// Represents instances created at runtime
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
    //UDecimal128()
    Float32(f32),
    Float64(f64),
    //These are commented out for now but I would like to bring in the 'num' crate at some point
    //to introduce these types.
    //BigInt(),
    //UBigInt(),
    //BigFloat(),
    //BigDecimal(),
    //Complex(),
    Char(char),
    String(Rc<Sym>),
    Array(Vec<Instance>),
    //Represents a custom class instance.
    CustomInstance(Box<CustomInstance>),
}

// Represents a class declared in Silicon code:
pub struct Class {
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

//Represents a custom class instance.
pub struct CustomInstance {
    class : Class,
    fields : Vec<Field>,

}

pub struct Field {
    field_info : FieldInfo,
    value : Instance,
}
