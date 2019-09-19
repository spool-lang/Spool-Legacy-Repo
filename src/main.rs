use std::env;
use std::process;

use Silicon;
use Silicon::Config;
use std::path::PathBuf;
use crate::runtime::VM;
use crate::opcode::OpCode::*;
use crate::instance::{Instance, Instance::*};

mod runtime;
mod opcode;
mod instance;

fn main() {
    let args: Vec<String> = env::args().collect();


    let mut vm = VM::new();

    &mut vm.register.insert(0, Bool(true));
    &mut vm.register.insert(1, Int16(2));
    &mut vm.register.insert(2, Int16(1));

    println!("Writing to the chunk!");
    &mut vm.chunk.write(Get(0));
    &mut vm.chunk.write(LogicNegate);

    println!("Running the program!");
    vm = vm.run();
    let result = vm.get_current_result();
    if let Int16(i) = result {
        print!("And the result is... {}", i)
    }
    else if let Bool(value) = result {
        print!("And the result is... {}", value)
    }

    /*
    if args.len() >= 2 {
        //engine::run(PathBuf::from(&args[1]))
    }
    else {
        println!("Please specify the path to the main script!");
        process::exit(1);
    }

    println!("Program execution finished successfully.");
    */
}
