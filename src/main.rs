use std::env;
use std::process;

use Silicon;
use Silicon::Config;
use std::path::PathBuf;
use crate::runtime::VM;
use crate::opcode::OpCode::*;
use crate::instance::{Instance, Instance::*};
use std::intrinsics::transmute;

mod runtime;
mod opcode;
mod instance;

fn main() {
    let args: Vec<String> = env::args().collect();


    let mut vm = VM::new();

    &mut vm.register.insert(0, Int16(0));
    &mut vm.register.insert(1, Int16(0));
    &mut vm.register.insert(2, Int16(1));

    println!("Writing to the chunk!");
    &mut vm.chunk.write(Get(0));
    &mut vm.chunk.write(Get(1));
    &mut vm.chunk.write(Eq);
    &mut vm.chunk.write(Print);
    &mut vm.chunk.write(Get(0));
    &mut vm.chunk.write(Get(2));
    &mut vm.chunk.write(Eq);
    &mut vm.chunk.write(Print);
    &mut vm.chunk.write(Get(0));
    &mut vm.chunk.write(Get(2));
    &mut vm.chunk.write(NotEq);
    &mut vm.chunk.write(Print);
    &mut vm.chunk.write(Get(0));
    &mut vm.chunk.write(Get(2));
    &mut vm.chunk.write(Less);
    &mut vm.chunk.write(Print);
    &mut vm.chunk.write(Get(2));
    &mut vm.chunk.write(Get(0));
    &mut vm.chunk.write(Less);
    &mut vm.chunk.write(Print);
    &mut vm.chunk.write(Get(2));
    &mut vm.chunk.write(Get(0));
    &mut vm.chunk.write(GreaterOrEq);
    &mut vm.chunk.write(Print);
    &mut vm.chunk.write(Get(1));
    &mut vm.chunk.write(Get(0));
    &mut vm.chunk.write(GreaterOrEq);
    &mut vm.chunk.write(Print);

    println!("Running the program!");
    vm = vm.run();

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
