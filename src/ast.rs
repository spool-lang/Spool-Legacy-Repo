use std::error::Error;
use std::fs;
use std::env;
use std::process;
use std::collections::*;
use crate::lex::Token::Str;
use crate::lex::Token::AngleIn;
use crate::lex::Token::AngleOut;
use crate::lex::Token::SquareOut;
use crate::lex::Token::SquareIn;
use crate::lex::Token::ParenOut;
use crate::lex::Token::ParenIn;
use crate::lex::Token::CurlyOut;
use crate::lex::Token::CurlyIn;
use crate::lex::Token::SemiColon;
use crate::lex::Token::Func;
use crate::lex::Token::Word;
use core::borrow::Borrow;
use crate::lex::Token::FuncName;
use crate::lex::Token::Colon;
use crate::lex::Token::Class;
use crate::lex::Token::Comma;
use crate::lex::Token;
use crate::ast::class::ClassStruct;

pub mod class {

    use super::field::*;

    trait ClassTrait {

    }

    pub struct ClassStruct {
        id : String,
        fields : Vec<Field>,
    }

    impl ClassStruct {

        pub fn new(id : String) -> ClassStruct {

            println!("Creating class {}.", id);

            ClassStruct {
                id,
                fields : vec![]
            }

        }


    }

}

pub mod field {

    pub struct Field {

    }
}

mod variables {

    struct Field {
        id : String
    }
}

pub fn build_ast(tokens : &mut Vec<Token>) {

    while !tokens.is_empty() {

        let token: Token = tokens[0].clone();

        match token {
            Class => {
                tokens.remove(0);
                let mut contents: Vec<Token> = vec![];

                let mut id : String;

                if let Word(string) = tokens[0].clone()  {
                    id = tokens[0].to_string()
                }
                else {
                    println!("{} is not a valid class name!", tokens[0].to_string());
                    process::exit(2)
                }

                loop {
                    let t : Token = tokens[0].clone();
                    tokens.remove(0);

                    match t {
                        CurlyOut => {
                            ClassStruct::new(id.to_string());
                            break
                        },
                        CurlyIn => {},
                        _ => contents.push(t)
                    }
                }
            },
            _ => {
                tokens.remove(0);
                return ();
            }
        }

        return;
    }
}