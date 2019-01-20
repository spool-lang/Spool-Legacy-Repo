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

//Represents something that can be passed around.
trait Value {
    fn get_id(&self) -> &str;
}

//Represents a string value.
pub struct StringValue {
    name: String
}

impl Value for StringValue {

    fn get_id(&self) -> &str {
        "<native>string"
    }
}

//Represents a boolean value.

//Represents a byte value.

//Represents a short value.

//Represents an integer value.

//Represents a long value.

//Represents a float value.

//Represents a double value.

//Represents an empty value.
struct Empty {

}

impl Value for Empty {
    fn get_id(&self) -> &str {
        return "<native>:empty"
    }
}


//Represents a loaded script file.


//Represents a class instance.


//Represents a singleton instance.


//Represents an annotation instance.

/*
    Functions
*/

//Trait implemented by script functions and native functions.
trait Function {
    fn execute(&self, args : Vec<Box<Value>>) -> &Value {
        let params : HashMap<String, Parameter> = self.get_params();
        let mut result : Value;

        if (args.is_empty()) && (params.is_empty()) {
            result = self.execute(vec![]);
            return result
        }

        if params.len() < args.len() {
            println!("Error: Too many arguments!");
            process::exit(2)
        }
        else if params.len() > args.len() {
            println!("Error: Too few arguments!");
            process::exit(2)
        }

        let mut i : i32 = 0;

        for param in params {

            if (args.get(i) as Value).get_type() != (param.1 as Parameter).param_type {
                println!("Error: argument type was not the same as parameter type!");
                process::exit(3)
            }

        }

        result = self.run_func(args)
    }

    fn get_params(&self) -> HashMap<String, Parameter>;

    fn run_func(&self, args : Vec<Box<Value>>) -> &Value;
}

//Represents a parameter.
struct Parameter{
    param_type : String
}

//Representation of a script function.
struct ScriptFunction {
    parameters : HashMap<String, Parameter>
}

//Native Functions.

//Print function.
struct PrintFunction {
    params : HashMap<String, Parameter>
}

impl Function for PrintFunction {

    fn get_params(&self) -> HashMap<String, Parameter> {
        return *self.params;
    }

    fn run_func(&self, things: Vec<Box<Value>>) -> &Value {
        let message : Box<Value> = (*(things.get(0).unwrap() as Box<Value>) as StringValue).name;
        println!("{}", message)
    }
}
