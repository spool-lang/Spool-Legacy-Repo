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

    let pooled_string = vm.string_pool.pool_string("Hello, world!".to_string());

    let mut func_chunk = Chunk::new();
    func_chunk.write(Get(false, 0));
    func_chunk.write(Print);
    let func = Function::new(1, func_chunk);

    let mut chunk = Chunk::new();
    chunk.add_const(0, Func(Rc::new(func)));
    chunk.add_const(1, Str(pooled_string));
    chunk.set_register_size(1);
    chunk.write(Get(true, 1));
    chunk.write(Get(true, 0));
    chunk.write(Call);
    chunk.write(Get(true, 1));
    chunk.write(Set(true, 0));
    chunk.write(Get(false, 0));

    let test = Bool(true);
    let bool_type = Type::new("silicon.lang.Boolean".to_string());
    let is = bool_type.is(test);
    println!("Is it an instance? {}", is);

    vm.execute_chunk(Rc::new(chunk), Rc::new(RefCell::new(CallFrame::new())), vec![]);

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
