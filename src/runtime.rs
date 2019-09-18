use std::rc::Rc;
use string_interner::DefaultStringInterner;
use string_interner::Sym;
use std::collections::HashMap;

pub struct VM {
    class_registry : HashMap<Sym, Instance>,
    // Represents the current call frame.
    frame : CallFrame
}

/*
Holds the current offset in the registry of the call frame as well as some
other useful information.
*/
pub struct CallFrame {
    offset : u16,
    /*
    Represents the current left and right operands as well as the result.
    The idea behind this is to reduce the amount of temporaries in the
    register.
    */
    left : Option<Instance>,
    right : Option<Instance>,
    result : Option<Instance>,
    // The previous call frame.
    previous : Box<CallFrame>
}

// OpCode instructions. All instructions should be 4 bytes at the most.
pub enum OpCode {
    /*
    Pulls instances from the registry at the specified location or from
    the result slot and sets them as the left or right operand.
    */
    SetLeft(u8),
    SetLeftX(u16),
    SetRight(u8),
    SetRightX(i16),
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
    CustomInstance(Box<CustomInstance>),
    //Represents a class object.
    Class(Box<Class>)
}

// Represents a class declared in Silicon code:
pub struct Class {
    canonical_name : str,
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
