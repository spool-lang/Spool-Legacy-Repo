use std::env;
use std::process;

use Silicon;
use Silicon::Config;
use std::path::PathBuf;
use crate::runtime::VM;
use crate::opcode::OpCode::*;
use crate::instance::{Instance, Instance::*};
use std::intrinsics::transmute;
use crate::opcode::Chunk;
use std::rc::Rc;

mod runtime;
mod opcode;
mod instance;

fn main() {
    let args: Vec<String> = env::args().collect();


    let mut vm = VM::new();
    let mut chunk = Chunk::new();
    &mut chunk.add_const(0, Byte(3));
    &mut chunk.set_register_size(1);

    println!("Writing to the chunk!");
    &mut chunk.write(Get(true, 0));
    &mut chunk.write(Get(true, 0));
    &mut chunk.write(Add);
    &mut chunk.write(Print);

    println!("Running the program!");
    vm.run_program(Rc::new(chunk));

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
