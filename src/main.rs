use std::env;
use std::process;

use Silicon;
use Silicon::Config;
use std::path::PathBuf;
use crate::runtime::VM;
use crate::opcode::OpCode::*;
use crate::instance::{Instance, Instance::*, Function};
use std::intrinsics::transmute;
use crate::opcode::Chunk;
use std::rc::Rc;

mod runtime;
mod opcode;
mod instance;

fn main() {
    let args: Vec<String> = env::args().collect();


    let mut vm = VM::new();

    let mut func_chunk = Chunk::new();
    func_chunk.add_const(0, Byte(6));

    func_chunk.write(Get(true, 0));
    func_chunk.write(Print);
    let func = Function::new(0, func_chunk);

    let mut chunk = Chunk::new();
    chunk.add_const(0, Func(Rc::new(func)));
    chunk.add_const(1, Byte(3));

    chunk.write(Get(true, 0));
    chunk.write(Call);
    chunk.write(Get(true, 1));
    chunk.write(Print);

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
