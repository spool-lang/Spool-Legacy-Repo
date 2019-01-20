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

const STRING_ID: &str = "<native>string";
const EMPTY_ID: &str = "<native>empty";

//Enum of all possible types
enum ValueType {
    String(String),
    Empty
}

impl ValueType {

    pub fn type_string(&self) -> String {

        let id = match self {
            ValueType::String(var) => STRING_ID,
            ValueType::Empty => EMPTY_ID
        };

        return id.to_string()
    }
}

//Trait for functions
trait Func {
    fn execute(&self, args : Vec<ValueType>) -> ValueType {
        let params : Vec<Parameter> = self.get_params();
        let mut mapped_args : HashMap<String, &ValueType> = HashMap::new();

        if (args.is_empty()) && (params.is_empty()) {
            return self.run_code(mapped_args)
        }

        if params.len() > args.len(){
            println!("Error: Too few arguments!");
            process::exit(2)
        }
        else if params.len() < args.len(){
            println!("Error: Too may arguments");
            process::exit(2)
        }

        for i in 0..args.len() {
            let arg: &ValueType = match args.get(i) {
                Some(T) => T,
                None => {
                    println!("Engine error!");
                    process::exit(22)
                }
            };
            let param : &Parameter = params.get(i).unwrap();

            if arg.type_string() != param.id {
                println!("Error: argument mismatch!");
                process::exit(3)
            } else {
                mapped_args.insert(param.name.clone(), arg);
            }
        }

        return self.run_code(mapped_args);
    }

    fn get_params(&self) -> Vec<Parameter>;

    fn run_code(&self, args : HashMap<String, &ValueType>) -> ValueType;
}

//Parameter struct
struct Parameter {
    id: String,
    name: String
}

//Legacy code!

/*
impl LegacyFunction for PrintFunction {

    fn get_params(&self) -> &HashMap<String, LegacyParameter> {
        let params = &self.params;
        return params;
    }

    fn run_func(&self, args: Vec<Box<Value>>) -> Box<&dyn Value> {
        let the_box : Box<Value> = *(args.get(0).unwrap());
        let string_value : StringValue = (*the_box as StringValue);
        let message : String = string_value.value;
        println!("{}", message);
        let value : &Value = &Empty {};
        let boxed_val : Box<&dyn Value> = Box::new(value);
        return boxed_val
    }
}
*/