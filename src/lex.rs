use std::ops::Deref;
use std::str::Chars;
use crate::lex::Tokens::Equality;
use nom::*;
use nom::IResult;
use std::str;
use std::sync::mpsc::channel;
use crate::lex::LexPattern::Alphanumeric;
use crate::lex::Tokens::PlusAssign;
use crate::lex::Tokens::Assign;
use crate::lex::Tokens::Increment;
use crate::lex::Tokens::Plus;
use std::str::from_utf8;
use crate::lex::Tokens::IdEquality;
use crate::lex::Tokens::AString;
use crate::lex::Tokens::Word;
use crate::lex::Tokens::Num;
use std::fs::File;
use crate::engine;
use std::io::Read;
use std::path::PathBuf;
use crate::lex::Tokens::Main;
use crate::lex::Tokens::BlockIn;
use crate::lex::Tokens::BlockOut;
use crate::lex::Tokens::Newline;
use crate::lex::Tokens::Var;
use crate::lex::Tokens::Const;
use crate::lex::Tokens::New;
use crate::lex::Tokens::Class;
use crate::lex::Tokens::Ctor;
use crate::lex::Tokens::Print;
use crate::lex::Tokens::Return;
use crate::lex::Tokens::ParenIn;
use crate::lex::Tokens::ParenOut;
use crate::lex::Tokens::Period;
use crate::lex::Tokens::Comma;
use crate::lex::Tokens::Colin;
use crate::lex::Tokens::Func;
use crate::lex::Tokens::ReturnType;

pub fn parse() {

    let mut test_input = r#"

    #This is a comment that lasts one line.

    ##
    This is a comment that covers multiple lines.
    ##

    main {
        var foo1 : Foo = new Foo()
        const foo2 : Foo = new Foo()

        foo1.greet()

        var fooString : String = foo2.giveMeString(1, 2)
    }

    class Foo {

        constructor() {

        }

        func greet() {
            print "Greetings!"
        }

        func giveMeString(left : Int, right : Int) -> String {
            return "I just returned something to you!"
        }

    }
    "#.to_string();

    let mut lexer = Lexer::new();
    //Comments.
    lexer.add_rule(LexPattern::String("#".to_string(), true), LexPattern::String('\n'.to_string(), true), 1);
    lexer.add_rule(LexPattern::String("##".to_string(), true), LexPattern::String("##".to_string(), true), 2);

    //Keywords.
    lexer.add_token(LexPattern::String("main ".to_string(), true), 5);
    lexer.add_token(LexPattern::String("var ".to_string(), true), 4);
    lexer.add_token(LexPattern::String("const ".to_string(), true), 6);
    lexer.add_token(LexPattern::String("new ".to_string(), true), 4);
    lexer.add_token(LexPattern::String("class ".to_string(), true), 6);
    lexer.add_token(LexPattern::String("constructor ".to_string(), true), 12);
    lexer.add_token(LexPattern::String("func ".to_string(), true), 5);
    lexer.add_token(LexPattern::String("print ".to_string(), true), 6);
    lexer.add_token(LexPattern::String("return ".to_string(), true), 7);

    //Brackets.
    lexer.add_token(LexPattern::String("{".to_string(), true), 1);
    lexer.add_token(LexPattern::String("}".to_string(), true), 1);
    lexer.add_token(LexPattern::String("(".to_string(), true), 1);
    lexer.add_token(LexPattern::String(")".to_string(), true), 1);

    //Operators.
    lexer.add_token(LexPattern::String("=".to_string(), true), 1);

    //Other symbols
    lexer.add_token(LexPattern::String(".".to_string(), true), 1);
    lexer.add_token(LexPattern::String(",".to_string(), true), 1);
    lexer.add_token(LexPattern::String(":".to_string(), true), 1);
    lexer.add_token(LexPattern::String("->".to_string(), true), 2);

    //Words and strings.
    lexer.add_rule(LexPattern::String('"'.to_string(), true), LexPattern::String('"'.to_string(), true), 1);
    lexer.add_rule(LexPattern::Alphabetic(true), LexPattern::Alphanumeric(true), 1);
    lexer.add_rule(LexPattern::Numeric(true), LexPattern::Alphanumeric(true), 1);

    //And newline
    lexer.add_token(LexPattern::String('\n'.to_string(), true), 1);
    lexer.add_token(LexPattern::String(" ".to_string(), true), 1);
    let results : Vec<Tokens> = lexer.lex(test_input);

    println!("Test lexer output!");
    for result in results {
        print!("{} ", result.to_string())
    }
    println!("End test lexer output!");
}

pub enum Tokens {
    //Keywords
    Main,
    Var,
    Const,
    New,
    Class,
    Ctor,
    Func,
    Print,
    Return,


    //Brackets
    BlockIn,
    BlockOut,
    ParenIn,
    ParenOut,

    //Other symbols
    Period,
    Comma,
    Colin,
    ReturnType,

    //Newline
    Newline,

    Plus,
    Increment,
    Assign,
    PlusAssign,
    Equality,
    IdEquality,
    AString(String),
    Word(String),
    Num(String),
}

impl Tokens {

    fn from_lexer(lex : String) -> Option<Tokens> {

        match lex.as_str() {
            //Keywords
            "main " => Some(Main),
            "var " => Some(Var),
            "const " => Some(Const),
            "new " => Some(New),
            "class " => Some(Class),
            "constructor " => Some(Ctor),
            "func " => Some(Func),
            "print " => Some(Print),
            "return " => Some(Return),

            //Brackets
            "{" => Some(BlockIn),
            "}" => Some(BlockOut),
            "(" => Some(ParenIn),
            ")" => Some(ParenOut),

            //Operators
            "=" => Some(Assign),

            //Other Symbols
            "." => Some(Period),
            "," => Some(Comma),
            ":" => Some(Colin),
            "->" => Some(ReturnType),

            //Newline
            "\n" => Some(Newline),

            "+" => Some(Plus),
            "++" => Some(Increment),
            "+=" => Some(PlusAssign),
            "==" => Some(Equality),
            "===" => Some(IdEquality),
            " " => None,

            //Words and strings.
            _ => {
                if lex.starts_with('"') {
                    let mut slice = lex.trim_start_matches('"');
                    slice = slice.trim_end_matches('"');
                    Some(AString(slice.to_string()))
                } else if lex.starts_with("#") || lex.starts_with("##") {
                    None
                } else {
                    match lex.clone().chars().nth(0) {
                        Some(c) => {
                            if c.is_ascii_alphabetic() {
                                Some(Word(lex))
                            } else if c.is_ascii_digit() {
                                Some(Num(lex))
                            } else {
                                panic!("Unknown Token!")
                            }
                        },
                        None => panic!("Unknown Token!")
                    }
                }
            }
        }
    }
}

impl ToString for Tokens {

    fn to_string(&self) -> String {

        let result = match self {
            //Keywords
            Main => "main".to_string(),
            Var => "var".to_string(),
            Const => "const".to_string(),
            New => "new".to_string(),
            Class => "class".to_string(),
            Ctor => "constructor".to_string(),
            Func => "func".to_string(),
            Print => "print".to_string(),
            Return => "return".to_string(),

            //Brackets
            BlockIn => "{".to_string(),
            BlockOut => "}".to_string(),
            ParenIn => "(".to_string(),
            ParenOut => ")".to_string(),

            //Operators
            Assign => "=".to_string(),

            //Other symbols
            Period => ".".to_string(),
            Comma => ",".to_string(),
            Colin => ":".to_string(),
            ReturnType => "->".to_string(),

            //Words and strings.
            AString(contents) => {
                let mut string_string: String = '"'.to_string();
                string_string.push_str(contents.as_str());
                string_string.push('"');
                string_string
            },
            Word(contents) => contents.clone(),
            Num(contents) => contents.clone(),

            //Newline
            Newline => "\n".to_string(),

            Plus => "+".to_string(),
            Increment => "++".to_string(),
            PlusAssign => "+=".to_string(),
            Equality => "==".to_string(),
            IdEquality => "===".to_string(),
            _ => "".to_string()
        };

        return result;
    }
}

#[derive(Clone)]
struct LexRule(LexPattern, LexPattern, usize);

#[derive(Clone)]
pub enum LexPattern {
    String(String, bool),
    Alphabetic(bool),
    Numeric(bool),
    Alphanumeric(bool),
    None
}

struct Lexer {
    rules : Vec<LexRule>,
    count : usize,
    current_token : String,
    chars : Vec<char>
}

impl Lexer {

    fn new() -> Lexer {
        Lexer {rules : vec![], count : 0, current_token : "".to_string(), chars : vec![]}
    }

    fn add_rule(&mut self, start : LexPattern, end : LexPattern, length : usize) {
        self.rules.push(LexRule(start, end, length));
    }

    fn add_token(&mut self, pat : LexPattern, length : usize) {
        self.rules.push(LexRule(pat, LexPattern::None, length));
    }

    fn next(&mut self) -> char {

        let mut result: char = ' ';

        if self.count < self.chars.len() {
            result = self.chars[self.count];
        }

        self.count += 1;
        return result
    }

    fn back(&mut self) {
        self.count -= 1;
    }

    fn look_ahead(&mut self, distance : usize) -> Vec<char> {
        let mut peeks: Vec<char> = vec![];

        let count = self.count;

        for i in 0..distance {

            if (self.count + i) < self.chars.len() {
                peeks.push(self.chars[self.count + i].clone())
            }
            else {
                peeks.push(' ')
            }
        }

        return peeks
    }

    fn lex(&mut self, input : String) -> Vec<Tokens> {

        let mut collected_chars : Vec<char> = input.chars().collect();
        &self.chars.append(&mut collected_chars);
        let len : usize = self.chars.len().clone();

        let mut lex_results : Vec<Tokens> = vec![];

        loop {
            let next_char = self.next();
            self.current_token.push(next_char);
            let clone_rules = self.rules.clone();
            let mut rule : &Option<LexRule> = &self.match_rules(clone_rules, 0);

            match &rule {
                Some(a_rule) => match &a_rule.1 {
                    LexPattern::None => {
                        let from_lex : Option<Tokens> = Tokens::from_lexer(self.current_token.clone());

                        match from_lex {
                            Some(tokens) => lex_results.push(tokens),
                            None => {}
                        }

                        self.current_token.clear();
                    },
                    LexPattern::String(pat, should) => {

                        loop {
                            let next_char = self.next();
                            self.current_token.push(next_char);
                            if self.current_token.ends_with(pat.as_str()) {
                                break;
                            }
                        }

                        let from_lex : Option<Tokens> = Tokens::from_lexer(self.current_token.clone());

                        match from_lex {
                            Some(tokens) => lex_results.push(tokens),
                            None => {}
                        }

                        self.current_token.clear();
                    },
                    LexPattern::Alphabetic(should) => {

                        loop {
                            let next_char = self.next();
                            if !(next_char.is_ascii_alphabetic()) {
                                self.back();
                                break;
                            }
                            self.current_token.push(next_char);
                        }

                        let from_lex : Option<Tokens> = Tokens::from_lexer(self.current_token.clone());

                        match from_lex {
                            Some(tokens) => lex_results.push(tokens),
                            None => {}
                        }

                        self.current_token.clear();

                    },
                    LexPattern::Alphanumeric(should) => {

                        loop {
                            let next_char = self.next();
                            if !(next_char.is_ascii_alphanumeric()) {
                                self.back();
                                break;
                            }
                            self.current_token.push(next_char);
                        }

                        let from_lex : Option<Tokens> = Tokens::from_lexer(self.current_token.clone());

                        match from_lex {
                            Some(tokens) => lex_results.push(tokens),
                            None => {}
                        }

                        self.current_token.clear();

                    },
                    LexPattern::Numeric(should) => {

                        loop {
                            let next_char = self.next();
                            if !(next_char.is_ascii_digit()) {
                                self.back();
                                break;
                            }
                            self.current_token.push(next_char);
                        }

                        let from_lex : Option<Tokens> = Tokens::from_lexer(self.current_token.clone());

                        match from_lex {
                            Some(tokens) => lex_results.push(tokens),
                            None => {}
                        }

                        self.current_token.clear();

                    },
                    _ => panic!("We're not there yet!")
                }
                None => {}
            }

            if self.count > self.chars.len() {
                break
            }
        }

        return lex_results;
    }

    fn match_rules(&mut self, rules : Vec<LexRule>, back_num : usize) -> Option<LexRule> {

        let mut matches : Vec<LexRule> = vec![];

        for rule in rules {

            let should_add : bool = match &rule.0 {
                LexPattern::String(pat, should) => {
                    let peek_vec : String = self.look_ahead(pat.len()).into_iter().collect();
                    let mut peek_token = self.current_token.clone();
                    peek_token.push_str(peek_vec.as_str());
                    if peek_token.starts_with(pat.as_str()) { *should } else { !should }
                },
                LexPattern::Alphabetic(should) => match &self.current_token.clone().chars().nth(0) {
                    Some(c) => { if c.is_ascii_alphabetic() { *should } else { !should } },
                    None => { !should }
                },
                LexPattern::Numeric(should) => match &self.current_token.clone().chars().nth(0) {
                    Some(c) => { if c.is_ascii_digit() { *should } else { !should } },
                    None => { !should }
                },
                LexPattern::Alphanumeric(should) => match &self.current_token.clone().chars().nth(0) {
                    Some(c) => { if c.is_ascii_alphanumeric() { *should } else { !should } },
                    None => { !should }
                },
                LexPattern::None => { panic!("Pattern type None should not be used here!") }
            };

            if should_add {
                matches.push(rule.clone())
            }

        }

        let mut longest_match: Option<LexRule> = None;
        let mut longest_len: usize = 0;

        for i in 0..matches.len() {

            if matches[i].2 > longest_len {
                longest_len = matches[i].2;
                longest_match = Some(matches[i].clone());
            }
        }

        for i in 1..longest_len {
            let next_token = self.next();
            self.current_token.push(next_token)
        }
        return longest_match
    }
}