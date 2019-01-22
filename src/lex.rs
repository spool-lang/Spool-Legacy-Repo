use std::process;

//Tokenizer and Lexer
pub struct Lexer {
    contents : String,
    filter : &'static Filter
}

impl Lexer {
    pub fn new(contents : String, filter : &'static Filter) -> Lexer {
        Lexer { contents, filter }
    }

    pub fn lex(&mut self) -> Vec<Token> {
        let chars : Vec<char> = self.contents.chars().collect();
        let mut output : Vec<Token> = vec![];
        let mut token : String = "".to_string();

        for ch in chars {
            let mut next = false;

            while !next {
                let result : TokenResult = self.filter.filter_char(self, &token, &ch);

                match result {
                    TokenResult::New(t) => {
                        output.push(t);
                        token.clear();
                        next = true;
                    },
                    TokenResult::Keep => {
                        token.push(ch);
                        next = true
                    },
                    TokenResult::Split(t) => {
                        output.push(t);
                        next = false
                    },
                    TokenResult::Retry => {
                        next = false
                    }
                    TokenResult::Drop => {
                        next = true
                    }
                }
            }
        }

        return output;
    }

    fn set_filter(&mut self, filter : &'static Filter) {
        self.filter = filter;
    }
}

//Trait to be used by different filters.
pub trait Filter {
    fn filter_char(&self, mut lex : &mut Lexer, token : &String, c : &char) -> TokenResult;
}

//Describes the result of filtering a token.
enum TokenResult {
    New(Token),
    Split(Token),
    Retry,
    Keep,
    Drop
}

//Function aliases for TokenResult.
fn keep() -> TokenResult {
    TokenResult::Keep
}

fn new(token : Token) -> TokenResult {
    TokenResult::New(token)
}

fn split(token : Token) -> TokenResult {
    TokenResult::Split(token)
}

fn retry() -> TokenResult {
    TokenResult::Retry
}

fn drop() -> TokenResult {
    TokenResult::Drop
}

//Filters

pub const BASIC_FILTER: BasicFilter = BasicFilter {};
const ID_FILTER : IdFilter = IdFilter {};
const STRING_FILTER : StringFilter = StringFilter {};
const COMMENT_FILTER : CommentFilter = CommentFilter {};
const MULTI_LINE : MultilineCommentFilter = MultilineCommentFilter {};
const ESCAPE_FILTER : EscapeFilter = EscapeFilter {};

pub struct BasicFilter;

impl Filter for BasicFilter {
    fn filter_char(&self, mut lex : &mut Lexer, token : &String, c : &char) -> TokenResult {

        let result : TokenResult = match c {
            ' ' => drop(),
            '\t' => drop(),
            '\n' => drop(),
            '\r' => drop(),
            '\u{C}' => drop(),

            ';' => new(Token::SemiColin),
            '_' => keep(),
            '{' => new(Token::CurlyIn),
            '}' => new(Token::CurlyOut),
            '(' => new(Token::ParenIn),
            ')' => new(Token::ParenOut),
            '[' => new(Token::SquareIn),
            ']' => new(Token::SquareOut),
            '<' => new(Token::AngleIn),
            '>' => new(Token::AngleOut),
            '!' => keep(),
            '"' => {
                lex.set_filter(&STRING_FILTER);
                drop()
            },
            '#' => {
                lex.set_filter(&COMMENT_FILTER);
                drop()
            },
            _ => {
                if c.is_alphabetic() {
                    lex.set_filter(&ID_FILTER);
                    keep()
                }
                else {
                    println!("Error: {} is not a valid character!", c);
                    process::exit(5)
                }
            }
        };

        return result
    }
}

pub struct IdFilter;

impl Filter for IdFilter {

    fn filter_char(&self, mut lex: &mut Lexer, token: &String, c: &char) -> TokenResult {

        if c.is_alphabetic() {
            return keep();
        }
        else {
            return split(Token::String(token.to_string()));
            lex.set_filter(&BASIC_FILTER);
        }
    }
}

pub struct StringFilter;

impl Filter for StringFilter {
    fn filter_char(&self, mut lex : &mut Lexer, token : &String, c : &char) -> TokenResult {

        let result : TokenResult = match c {
            '"' => {
                lex.set_filter(&BASIC_FILTER);
                new(Token::String(token.to_string()))
            },
            _ => keep()
        };

        return result
    }
}

pub struct CommentFilter;

impl Filter for CommentFilter {

    fn filter_char(&self, mut lex : &mut Lexer, token : &String, c : &char) -> TokenResult {

        let result : TokenResult = match c {
            '\n' => {
                lex.set_filter(&BASIC_FILTER);
                drop()
            },
            '\r' => {
                lex.set_filter(&BASIC_FILTER);
                drop()
            },
            '#' => {
                lex.set_filter(&MULTI_LINE);
                drop()
            }
            _ => drop()
        };

        return result
    }
}

pub struct MultilineCommentFilter;

impl Filter for MultilineCommentFilter {
    fn filter_char(&self, mut lex : &mut Lexer, token : &String, c : &char) -> TokenResult {

        if let '#' = c {
            lex.set_filter(&ESCAPE_FILTER)
        }

        return drop()
    }
}

pub struct EscapeFilter;

impl Filter for EscapeFilter {
    fn filter_char(&self, mut lex : &mut Lexer, token : &String, c : &char) -> TokenResult {

        if let '#' = c {
            lex.set_filter(&BASIC_FILTER)
        }

        return drop();
    }
}

//Enum of tokens:

pub enum Token {
    SemiColin,
    CurlyIn,
    CurlyOut,
    ParenIn,
    ParenOut,
    SquareIn,
    SquareOut,
    AngleIn,
    AngleOut,
    Identifier(String),
    String(String)
}

impl Token {

    pub fn to_string(&self) -> &str {

        let string = match self {
            Token::SemiColin => ";",
            Token::CurlyIn => "{",
            Token::CurlyOut => "}",
            Token::ParenIn => "(",
            Token::ParenOut => ")",
            Token::SquareIn => "[",
            Token::SquareOut => "]",
            Token::AngleIn => "<",
            Token::AngleOut => ">",
            Token::Identifier(id) => id,
            Token::String(s) => s
        };

        return string
    }
}