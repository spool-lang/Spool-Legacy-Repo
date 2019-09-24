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

    let mut func_chunk = Chunk::new();
    func_chunk.set_register_size(1);
    func_chunk.write(Get(false, 0));
    func_chunk.write(Get(false, 0));
    func_chunk.write(Multiply);
    func_chunk.write(Return(true));
    let func = Function::new(0, func_chunk);

    let mut chunk = Chunk::new();
    chunk.set_register_size(1);
    chunk.add_const(0, Func(Rc::new(func)));
    chunk.add_const(1,Int16(50));
    chunk.add_const(2,Int16(5));
    chunk.add_const(3,Int16(23));
    chunk.add_const(4, Byte(3));
    chunk.write(Get(true, 1));
    chunk.write(Get(true, 1));
    chunk.write(Get(true, 3));
    chunk.write(Get(true, 1));
    chunk.write(Get(true, 1));
    chunk.write(Get(true, 1));
    chunk.write(Get(true, 1));
    chunk.write(Get(true, 1));
    chunk.write(InitArray(8));
    chunk.write(Set(0));
    chunk.write(Get(false, 0));
    chunk.write(Print);
    chunk.write(Get(false, 0));
    chunk.write(Get(true, 2));
    chunk.write(IndexGet);
    chunk.write(Print);
    chunk.write(Get(false, 0));
    chunk.write(Get(true, 2));
    chunk.write(Get(true, 4));
    chunk.write(IndexSet);
    chunk.write(Get(false, 0));
    chunk.write(Print);

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
