use std::env;
use std::process;

use silicon;
use silicon::Config;
use std::path::PathBuf;
use crate::runtime::{VM, CallFrame};
use crate::opcode::OpCode::*;
use crate::instance::{Instance, Instance::*, Function, Type};
use std::intrinsics::transmute;
use crate::opcode::Chunk;
use std::rc::Rc;
use std::collections::HashSet;
use crate::string_pool::StringPool;
use std::cell::RefCell;

mod runtime;
mod opcode;
mod instance;
mod string_pool;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut vm = VM::new();

    let mut chunk = Chunk::new();
    chunk.add_const(0, Bool(true));
    chunk.add_const(1, Bool(false));
    chunk.add_const(2, Byte(0));
    chunk.write(Get(true, 0));
    chunk.write(Is(0));
    chunk.write(Print);
    chunk.write(Get(true, 0));
    chunk.write(Declare(false, 0));
    chunk.write(Get(false, 0));
    chunk.write(Print);
    chunk.write(Get(true, 1));
    chunk.write(Set(0));
    chunk.write(Get(false, 0));
    chunk.write(Print);
    chunk.write(Get(true, 2));
    chunk.write(Set(0));

    vm.execute_chunk(Rc::new(chunk), Rc::new(RefCell::new(CallFrame::new())), vec![], vec![]);

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
