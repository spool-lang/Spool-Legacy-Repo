use std::env;
use std::process;

use silicon;
use silicon::Config;
use std::path::PathBuf;
use crate::runtime::{VM, CallFrame};
use crate::opcode::OpCode::*;
use crate::instance::{Instance, Instance::*, Function};
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
    let pooled_str1 = vm.string_pool.pool_string("Hello!".to_string());
    let pooled_str2 = vm.string_pool.pool_string("How are you?".to_string());

    let mut chunk = Chunk::new();
    chunk.set_register_size(2);
    chunk.add_const(0, Str(pooled_str1));
    chunk.add_const(1, Str(pooled_str2));
    chunk.write(Get(true, 0));
    chunk.write(Set(0));
    chunk.write(EnterScope(1));
    chunk.write(Get(true, 1));
    chunk.write(Set(0));
    chunk.write(Get(false, 0));
    chunk.write(Print);
    chunk.write(Get(false, 1));
    chunk.write(Print);
    chunk.write(ExitScope);
    chunk.write(Get(false, 0));
    chunk.write(Print);
    // Uncommenting this line will crash the program.
    //chunk.write(Get(false, 1));

    vm.execute_chunk(Rc::new(chunk), Rc::new(RefCell::new(CallFrame::new())));

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
