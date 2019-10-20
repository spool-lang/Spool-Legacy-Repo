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
    let mut func_chunk = Chunk::new();
    func_chunk.set_register_size(1);
    func_chunk.write(Get(false, 0));
    func_chunk.write(Print);

    let string_type = Rc::new(Type::new(Rc::from("silicon.lang.String".to_string())));
    let func = Function::new(1, vec![string_type], func_chunk);

    let mut chunk = Chunk::new();
    chunk.set_register_size(1);
    chunk.add_const(0, Func(Rc::from(func)));
    chunk.add_const(1, Bool(true));
    chunk.write(Get(true, 1));
    chunk.write(Get(true, 0));
    chunk.write(Call);

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
