use std::path::PathBuf;
use std::path::MAIN_SEPARATOR;
use std::fs::File;
use std::env;
use std::process;
use std::io::Read;
use crate::parse;
use crate::parse::Node;
use std::collections::HashMap;

use crate::lex;
use crate::lex::Token;
use crate::parse::FileNode;

//Responsible for managing the program itself.

pub struct Program {
    root_dir : PathBuf
}

impl Program {

    pub fn find(&self) {

    }
}

pub fn run(path : PathBuf) {

    let root : Program = Program {
        root_dir : root_directory(&path)
    };

    let mut main_class_file : File = open_script(&path);
    let mut contents : String = "".to_string();
    main_class_file.read_to_string(&mut contents);

    let tokens : Vec<Token> = lex::lex_string(contents);

    let mut program : FileNode = parse::build(tokens);
    program.run(HashMap::new());
    /*
    let mut program : ast::FileNode = ast::parse_file(contents);
    program.run(HashMap::new());

    */
    return;
}

fn root_directory(path : &PathBuf) -> PathBuf {

    let path = path.as_path();
    let path = path.parent().unwrap();
    let root = path.to_path_buf();
    return root

}

fn class_path(path : &PathBuf, import : String) -> PathBuf {
    let split = import.split(".");
    let components : Vec<&str> = split.collect();

    let mut class_path = path.clone();

    for component in components {
        class_path.push(component)
    }

    class_path.set_extension(".silicon");

    return class_path;
}

pub fn open_script(path : &PathBuf) -> File {
    let mut path = path.clone();
    path.set_extension("silicon");

    let file : File;
    match File::open(&path) {
        Ok(T) => file = T,
        Err(E) => {
            println!("File {:?} is not a valid script file!", path.to_str());
            process::exit(1)
        }
    }
    return file
}

//Data wrappers

pub enum Type {
    String(String),
    Num(u64)
}