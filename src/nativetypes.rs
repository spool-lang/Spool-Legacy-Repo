//Deprecated

enum NativeType {
    Byte(),
    Int16(),
    Int32(),
    Int64(),
    UByte(),
    UInt16(),
    UInt32(),
    UInt64(),
    String(str),
    Char(),
    Array(),
    Dictionary,
    Class(),
    Empty
}

impl NativeType {

    fn call(self, name : &str, arguments : Vec<NativeType>) -> NativeType {

        match self {
            NativeType::String() => string_type::call(self, name, arguments),
            _ => {panic!("Something went wrong when calling something!")}
        }

        return NativeType::Empty
    }
}

mod string_type {

    use super::NativeType;

    pub fn call(this : NativeType, name : &str, arguments : Vec<NativeType>) -> NativeType {

        match name {
            "" => {}
            _ => {panic!("The function [{}] does not exist on String.", name)}
        }

        return NativeType::Empty
    }

    pub fn concatenate(this : NativeType, name : &str, arguments : Vec<NativeType>) -> NativeType {

        if arguments.len() > 1 {
            panic!("Not enough arguments!")
        }
        else if arguments.len() < 1 {
            panic!("Too few arguments!")
        }

        match &arguments[0] {
            NativeType::String(contents) => {}
            _ => panic!("This type cannot be concatenated with a string!")
        }

        return this
    }
}

