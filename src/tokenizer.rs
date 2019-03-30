use std::ops::Deref;
use std::str::Chars;
use crate::tokenizer::Tokens::Equality;
use nom::*;
use nom::IResult;
use std::str;
use std::sync::mpsc::channel;
use crate::tokenizer::LexPattern::Alphanumeric;

pub fn parse() {

    let mut test_lexer = Lexer::new();
    test_lexer.add_rule(LexRule(LexPattern::String("+".to_string(), true), LexPattern::None, 1));
    let results = test_lexer.lex("+".to_string());

    print!("Test lexer output!");
    for result in results {
        print!("{}", result.clone())
    }
    print!("End test lexer output!");
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

#[derive(Clone)]
struct LexRule(LexPattern, LexPattern, usize);

impl LexRule {

    pub fn dup(&self) -> LexRule {

        let start = self.0.clone();

        let end = self.1.clone();

        return LexRule(start, end, self.2.clone())
    }
}

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

    fn add_rule(&mut self, rule : LexRule) {
        self.rules.push(rule);
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

    fn lex(&mut self, input : String) -> Vec<String> {

        let mut collected_chars : Vec<char> = input.chars().collect();
        &self.chars.append(&mut collected_chars);
        let len : usize = self.chars.len().clone();
        let mut rule : &LexRule;

        let mut lex_results : Vec<String> = vec![];

        loop {
            let next_char = self.next();
            self.chars.push(next_char);

            let clone_rule = self.rules.clone();
            let rule = &self.match_rules(clone_rule, 0)[0];

            self.count += 1;

            match rule.1 {
                LexPattern::None => {
                    lex_results.push(self.current_token.clone());
                    self.current_token.clear();
                },
                _ => panic!("We're not there yet!")
            }

            if self.count > self.chars.len() {
                break
            }
        }

        return lex_results;
    }

    fn match_rules(&mut self, rules : Vec<LexRule>, back_num : usize) -> Vec<LexRule> {

        let mut matches : Vec<LexRule> = vec![];

        for rule in &self.rules {

            let should_add : bool = match &rule.0 {
                LexPattern::String(pat, should) => { if self.current_token.as_str().starts_with(pat) {*should} else { !should } },
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

        if matches.len() <= 1 {
            for i  in 0..(back_num - (matches[0].2 - 1)) {
                self.back();
            }

            return matches;
        }
        else {
            let next_char = self.next();
            self.chars.push(next_char);
            return self.match_rules(matches, back_num + 1);
        }
    }
}