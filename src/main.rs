use std::env;
use std::process;

use Silicon;
use Silicon::Config;
use std::path::PathBuf;
use crate::runtime::VM;
use crate::opcode::OpCode::{Add, Get, Set, Divide, Multiply, Subtract};
use crate::instance::{Instance, Instance::*};

mod runtime;
mod opcode;
mod instance;

fn main() {
    let args: Vec<String> = env::args().collect();


    let mut vm = VM::new();

    &mut vm.register.insert(0, Int16(8));
    &mut vm.register.insert(1, Int16(16));
    &mut vm.register.insert(2, Int16(4));
    &mut vm.register.insert(4, Int16(12));
    &mut vm.register.insert(5, Int16(10));

    println!("Writing to the chunk!");
    &mut vm.chunk.write(Get(0));
    &mut vm.chunk.write(Get(1));
    &mut vm.chunk.write(Add);
    &mut vm.chunk.write(Get(2));
    &mut vm.chunk.write(Add);
    &mut vm.chunk.write(Set(3));
    &mut vm.chunk.write(Get(3));
    &mut vm.chunk.write(Get(0));
    &mut vm.chunk.write(Add);
    &mut vm.chunk.write(Get(4));
    &mut vm.chunk.write(Divide);
    &mut vm.chunk.write(Get(5));
    &mut vm.chunk.write(Multiply);
    &mut vm.chunk.write(Get(0));
    &mut vm.chunk.write(Subtract);


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
