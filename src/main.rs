use std::env;
use std::process;

use Silicon;
use Silicon::Config;
use std::path::PathBuf;
use crate::runtime::VM;
use crate::runtime::Instance::Int16;
use crate::runtime::OpCode::{Add, Get};

mod engine;
mod parse;
mod lex;

mod runtime;

fn main() {
    let args: Vec<String> = env::args().collect();


    let mut vm = VM::new();

    &mut vm.register.insert(0, &Int16(8));
    &mut vm.register.insert(1, &Int16(8));

    println!("Writing to the chunk!");
    &mut vm.chunk.write(Get(0));
    &mut vm.chunk.write(Get(1));
    &mut vm.chunk.write(Add);

    println!("Running the program!");
    vm = vm.run();
    let result = vm.get_current_result();
    if let Int16(i) = result {
        print!("And the result is... {}", i)
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
