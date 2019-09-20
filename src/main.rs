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
    &mut vm.chunk.add_const(0, Int128(64));

    &mut vm.chunk.jump_table.insert(0, 5);

    println!("Writing to the chunk!");
    &mut vm.chunk.write(GetFalse);
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
