use std::error::Error;
use std::fs;
use std::env;
use std::process;
use std::path::PathBuf;
use std::fs::File;
use std::io::Read;
use std::alloc::System;
use crate::engine::Program;
use std::collections::HashMap;
use std::string::String;


//Represents a loaded script file.
pub struct Script {
    identifier : String,
    imports : HashMap<String, Script>,
    fields : HashMap<String, ScriptValue>,
    functions : HashMap<String, Function>
}

impl Script {

    fn new(program : Program, contents : Vec<Vec<String>>) -> Script {

    }

}

//Trait for values
pub trait Value<T> {
    fn get_value() -> T;
}

//Represents a variable or constant.
pub struct ScriptValue {
    identifier : String,
    is_const : bool
}

//Trait to be used for all functions.
pub trait Function {
    fn execute_pre(&self, arguments : Vec<Value<T>>) {
        let ids = self.get_ids();
        let mut args : HashMap<String, Value<T>> = HashMap::new();

        for i in ids.len() {
            args.insert(ids[i], arguments[i])
        }


    }

    fn get_ids(&self) -> &Vec<String>;

    fn execute(&self, args : HashMap<String, Value<T>>);
}

//Represents a function.
pub struct ScriptFunction {
    identifier : String
}

pub enum ScriptType {
    Class,
    Singleton,
    Enum,
    Interface,
    Annotation,
    Unknown
}

/*
    Code for native functions and variables.
*/

pub struct NativeFunctions {
    functions : HashMap<String, Function>
}

impl NativeFunctions {
    fn new() -> NativeFunctions {

        let mut natives = HashMap::new();

        natives.insert("println", PrintFunc {arg_ids : vec!["message".to_string()]});

        NativeFunctions {
            functions: natives
        }
    }
}

//Print function
pub struct PrintFunc {
    arg_ids : Vec<String>
}

impl Function for PrintFunc {
    fn get_ids(&self) -> &Vec<String> {
        &self.arg_ids
    }

    fn execute(&self, args : Vec<Value<T>>) {
        let mut output : String;
        if args[&0] as String {
            println!("{}", args[&0])
        }
        else {
            println!("Expected string!");
            process::exit(2)
        }
    }
}