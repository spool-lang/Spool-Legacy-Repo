//Tokenizer and Lexer
pub struct Lexer {
    contents : String,
    filter : &'static Filter
}

impl Lexer {
    pub fn new(contents : String, filter : &'static Filter) -> Lexer {
        Lexer { contents, filter }
    }

    pub fn lex(&mut self) -> Vec<String> {
        let chars : Vec<char> = self.contents.chars().collect();
        let mut output : Vec<String> = vec![];
        let mut token : String = "".to_string();

        for ch in chars {
            let result : TokenResult = self.filter.filter_char(&ch, self);

            match result {
                TokenResult::Drop => {
                    if !(token.is_empty()) {
                        output.push(token.clone());
                        token.clear();
                    }
                },
                TokenResult::New => {
                    if !(token.is_empty()) {
                        output.push(token.clone());
                        token.clear();
                    }
                    token.push(ch);
                }
                TokenResult::Keep => {
                    token.push(ch);
                }
            }
        }

        output.push(token);
        return output;
    }

    fn set_filter(&mut self, filter : &'static Filter) {
        self.filter = filter;
    }
}

//Trait to be used by different filters.
pub trait Filter {
    fn filter_char(&self, c : &char, mut lex : &mut Lexer) -> TokenResult;
}

//Describes the result of filtering a token.
enum TokenResult {
    New,
    Keep,
    Drop
}

//Function aliases for TokenResult.
fn keep() -> TokenResult {
    TokenResult::Keep
}

fn new() -> TokenResult {
    TokenResult::New
}

/*
fn retry() -> TokenResult {
    TokenResult::Retry
}
*/

fn drop() -> TokenResult {
    TokenResult::Drop
}

//Filters

pub const BASIC_FILTER: BasicFilter = BasicFilter {};
const STRING_FILTER : StringFilter = StringFilter {};
const COMMENT_FILTER : CommentFilter = CommentFilter {};
const MULTI_LINE : MultilineCommentFilter = MultilineCommentFilter {};
const ESCAPE_FILTER : EscapeFilter = EscapeFilter {};

pub struct BasicFilter;

impl Filter for BasicFilter {
    fn filter_char(&self, c : &char, mut lex : &mut Lexer) -> TokenResult {

        let result : TokenResult = match c {
            ' ' => drop(),
            '\t' => drop(),
            '\n' => drop(),
            '\r' => drop(),
            '\u{C}' => drop(),

            ';' => new(),
            '_' => keep(),
            '{' => new(),
            '}' => new(),
            '(' => new(),
            '[' => new(),
            ']' => new(),
            '<' => new(),
            '>' => new(),
            '!' => keep(),
            '"' => {
                lex.set_filter(&STRING_FILTER);
                drop()
            },
            '#' => {
                lex.set_filter(&COMMENT_FILTER);
                drop()
            }
            _ => keep()
        };

        return result
    }
}

pub struct StringFilter;

impl Filter for StringFilter {
    fn filter_char(&self, c: &char, mut lex : &mut Lexer) -> TokenResult {

        let result : TokenResult = match c {
            '"' => {
                lex.set_filter(&BASIC_FILTER);
                drop()
            },
            _ => keep()
        };

        return result
    }
}

pub struct CommentFilter;

impl Filter for CommentFilter {

    fn filter_char(&self, c: &char, mut lex: &mut Lexer) -> TokenResult {

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
        };

        return result
    }
}

pub struct MultilineCommentFilter;

impl Filter for MultilineCommentFilter {
    fn filter_char(&self, c: &char, mut lex: &mut Lexer) -> TokenResult {

        if let '#' = c {
            lex.set_filter(&ESCAPE_FILTER)
        }

        return drop()
    }
}

pub struct EscapeFilter;

impl Filter for EscapeFilter {
    fn filter_char(&self, c: &char, mut lex: &mut Lexer) -> TokenResult {

        if let '#' = c {
            lex.set_filter(&BASIC_FILTER)
        }

        return drop();
    }
}

//Enum of tokens:

enum Token {
    SemiColin,
    Underscore,

}