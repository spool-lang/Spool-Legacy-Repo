use std::env;
use std::process;

use Silicon;
use Silicon::Config;
use std::path::PathBuf;
use crate::runtime::{VM, CallFrame};
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
    vm.register.insert(0, Int16(0));
    vm.register.insert(2, Int16(1));

    let mut func_chunk = Chunk::new();
    func_chunk.write(Get(false, 0));
    func_chunk.write(Print);
    func_chunk.register_size = 1;
    let func = Function::new(0, func_chunk);

    let mut chunk = Chunk::new();
    chunk.add_const(0, Func(Rc::new(func)));
    chunk.write(Get(true, 0));
    chunk.write(Call);
    chunk.write(Get(false, 0));
    chunk.write(Print);
    chunk.register_size = 2;


    vm.run_program(Rc::new(chunk), Rc::new(CallFrame::new()));

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
