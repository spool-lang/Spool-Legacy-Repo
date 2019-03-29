use std::ops::Deref;
use std::str::Chars;
use crate::tokenizer::Tokens::Equality;

pub fn parse() {

    let contents = "+ = ==".to_string();
    let mut chars = contents.chars();

    let mut current_token = "".to_string();
    let mut tokens : Vec<Tokens> = vec![];

    let mut count : usize = 0;
    let limit : usize = chars.clone().count();

    loop {

        match &chars.nth(count) {
            Some(c) => {current_token.push(c.clone())},
            None => {}
        }

        println!("{}", current_token.clone());

        if current_token.starts_with("+") {
            tokens.push(Tokens::Plus);
            current_token = "".to_string();
        } else if current_token.as_str() == " " {
            current_token = "".to_string()
        } else if current_token.starts_with("=") {

            match &chars.nth(count + 1) {
                Some(c) => {current_token.push(c.clone())},
                None => {}
            }

            if current_token.as_str() == "==" {
                tokens.push(Tokens::Equality);
                current_token.clear();
                count += 1
            } else {
                tokens.push(Tokens::Assign);
                current_token.clear();
            }
        }

        if count >= limit {break}

        count += 1;
    }

    println!("Stringifying tokens!");
    for token in tokens {
        println!("{}", token.to_string())
    }
}

pub enum Tokens {
    Plus,
    Assign,
    Equality
}

impl ToString for Tokens {

    fn to_string(&self) -> String {

        match self {
            Tokens::Plus => "+".to_string(),
            Tokens::Assign => "=".to_string(),
            Equality => "==".to_string()
        }
    }
}
