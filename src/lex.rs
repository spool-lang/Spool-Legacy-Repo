use std::process;
use crate::lex::FilterResult::New;
use crate::lex::FilterResult::Multiple;
use crate::lex::FilterResult::Keep;
use crate::lex::FilterResult::Drop;
use crate::lex::Token::Str;
use crate::lex::Token::AngleIn;
use crate::lex::Token::AngleOut;
use crate::lex::Token::SquareOut;
use crate::lex::Token::SquareIn;
use crate::lex::Token::ParenOut;
use crate::lex::Token::ParenIn;
use crate::lex::Token::CurlyOut;
use crate::lex::Token::CurlyIn;
use crate::lex::Token::SemiColin;
use crate::lex::Token::Func;
use crate::lex::Filter::StrFilter;
use crate::lex::Filter::Comment;
use crate::lex::Filter::WordFilter;
use crate::lex::Filter::Basic;
use crate::lex::Filter::Multiline;
use crate::lex::Token::Word;
use core::borrow::Borrow;
use crate::lex::Filter::FuncFilter;
use crate::lex::Token::FuncName;
use crate::lex::Filter::ParamFilter;
use crate::lex::Token::Colin;
use crate::lex::Token::Comma;

//Tokenizer and Lexer
pub struct Lexer {
    contents : String,
    filter : Filter
}

impl Lexer {
    pub fn new(contents : String, filter : Filter) -> Lexer {
        Lexer { contents, filter }
    }

    pub fn lex(&mut self) -> Vec<Token> {
        let chars : Vec<char> = self.contents.chars().collect();
        let mut output : Vec<Token> = vec![];
        let mut token : String = "".to_string();

        for ch in chars {
            let mut next = false;

            while !next {
                let result : (FilterResult, Option<Filter>, bool) = self.filter.filter(ch, token.clone());

                match result.0 {
                    New(t) => {
                        println!("{}", &t.to_string());
                        output.push(t);
                        token.clear()
                    },
                    Multiple(tokens) => {
                        for token in tokens {
                            println!("{}", &token.to_string());
                            output.push(token)
                        }
                        token.clear()
                    }
                    Keep => {
                        token.push(ch)
                    },
                    Drop => {
                        token.clear()
                    }
                }

                match result.1 {
                    Some(f) => self.filter = f,
                    None => {}
                }

                next = result.2
            }
        }

        return output;
    }
}

//Describes the result of filtering a token.
pub enum FilterResult {
    New(Token),
    Multiple(Vec<Token>),
    Keep,
    Drop
}

//filters
pub enum Filter {
    Basic,
    FuncFilter,
    ParamFilter,
    WordFilter,
    StrFilter,
    Comment,
    Multiline
}

impl Filter {

    pub fn filter(&self, c : char, tok : String) -> (FilterResult, Option<Filter>, bool) {

        let result : (FilterResult, Option<Filter>, bool) = match self {
            Basic => match c {
                ' ' => (Drop, None, true),
                '\t' => (Drop, None, true),
                '\n' => (Drop, None, true),
                '\r' => (Drop, None, true),
                '\u{C}' => (Drop, None, true),

                ';' => (New(SemiColin), None, true),
                '{' => (New(CurlyIn), None, true),
                '}' => (New(CurlyOut), None, true),
                '(' => (New(ParenIn), None, true),
                ')' => (New(ParenOut), None, true),
                '[' => (New(SquareIn), None, true),
                ']' => (New(SquareOut), None, true),
                '<' => (New(AngleIn), None, true),
                '>' => (New(AngleOut), None, true),
                '"' => (Drop, Some(StrFilter), true),
                '#' => (Keep, Some(Comment), true),
                _ => {
                    if c.is_ascii_alphabetic() {
                        (Keep, Some(WordFilter), true)
                    }
                    else {
                        println!("Error: {} is not a valid character!", c);
                        process::exit(5)
                    }
                }
            },
            FuncFilter => match c{
                '(' => (Multiple(vec![FuncName(tok), ParenIn]), Some(ParamFilter), true),
                _ => {
                    if c.is_ascii_alphabetic() {
                        (Keep, None, true)
                    }
                    else {
                        println!("Illegal character {} in function name!", c);
                        process::exit(5)
                    }
                }
            },
            ParamFilter => match c {
                ':' => {
                    if tok.is_empty() {
                        (New(Colin), None, true)
                    }
                    else {
                        (New(Word(tok)), None, false)
                    }
                },
                ',' => {
                    if tok.is_empty() {
                        (New(Comma), None, true)
                    }
                    else {
                        (New(Word(tok)), None, false)
                    }
                },
                ')' => (New(ParenOut), Some(Basic), false),
                _ => (Keep, None, false)
            }
            WordFilter => {
                let mut word_result : (FilterResult, Option<Filter>, bool);

                if !c.is_ascii_alphabetic() {
                    word_result = match tok.as_ref() {
                        "func" => (New(Func), Some(FuncFilter), true),
                        _ => (New(Word(tok)), Some(Basic), false)
                    }
                }
                else {
                    word_result = (Keep, None, true)
                }

                word_result

            }
            StrFilter => match c {
                '"' => (New(Str(tok.clone())), Some(Basic), true),
                _ => (Keep, None, true)
            },
            Comment => match c {
                '\n' => (Drop, Some(Basic), true),
                '\r' => (Drop, Some(Basic), true),
                _ => {
                    if tok == "##" {
                        (Drop, Some(Multiline), true)
                    }
                    else {
                        (Drop, None, true)
                    }
                }
            },
            Multiline => {
                if (tok.get((tok.len() - 1)..(tok.len())) == Some("#")) && (c == '#') {
                    (Drop, Some(Basic), true)
                }
                else {
                    (Drop, None, true)
                }
            }
        };

        return result
    }
}

//Enum of tokens:

pub enum Token {
    SemiColin,
    Colin,
    Comma,
    CurlyIn,
    CurlyOut,
    ParenIn,
    ParenOut,
    SquareIn,
    SquareOut,
    AngleIn,
    AngleOut,
    Word(String),
    FuncName(String),
    Str(String),
    Native,
    Func,
}

const SYMBOL_STRING : &str = "Symbol:";


impl Token {

    pub fn to_string(&self) -> String {

        let string : String = match self {
            Token::SemiColin => "Symbol ;".to_string(),
            Token::Colin => "Symbol :".to_string(),
            Token::Comma => "Symbol ,".to_string(),
            Token::CurlyIn => "Symbol {".to_string(),
            Token::CurlyOut => "Symbol }".to_string(),
            Token::ParenIn => "Symbol (".to_string(),
            Token::ParenOut => "Symbol )".to_string(),
            Token::SquareIn => "Symbol [".to_string(),
            Token::SquareOut => "Symbol ]".to_string(),
            Token::AngleIn => "Symbol <".to_string(),
            Token::AngleOut => "Symbol >".to_string(),
            Token::FuncName(name) => {
                let mut a_string = "Function: ".to_string();
                a_string.push_str(name);
                a_string
            }
            Token::Word(word) => {
                let mut a_string = "Word ".to_string();
                a_string.push_str(word);
                a_string
            },
            Token::Str(s) => {
                let mut a_string = "String literal ".to_string();
                a_string.push_str(s);
                a_string
            },
            Token::Native => "Keyword native".to_string(),
            Token::Func => "Keyword func".to_string()
        };

        return string
    }
}