use tokesies::*;
use std::str::Chars;
use std::collections::HashMap;
use crate::engine;
use std::process::id;

pub fn parse_file(contents : String) -> FileNode {

    let tokens : Vec<Token> = FilteredTokenizer::new(MyFilter{}, contents.as_str()).collect::<Vec<Token>>();

    let mut token_strings : Vec<String> = vec![];
    for t in tokens {
        token_strings.push(t.term().to_string())
    }

    let mut token_stream : TokenStream = TokenStream {tokens : token_strings, count : 0};
    let mut file_node : FileNode = FileNode::new();

    file_node.parse(&mut token_stream);

    return file_node
}

//Tokenizer for turning the contents of the file into a more easily digestible stream of tokens.
pub struct MyFilter;

impl filters::Filter for MyFilter {
    fn on_char(&self, c: &char) -> (bool, bool) {

        match *c {
            ' ' => keep(),
            '\t' => drop(),
            '\n' => keep(),
            '\r' => drop(),
            '\u{C}' => drop(),

            ';' => keep(),
            '_' => part(),
            '{' => keep(),
            '}' => keep(),
            '(' => keep(),
            ')' => keep(),
            '[' => keep(),
            ']' => keep(),
            '<' => keep(),
            '>' => keep(),
            '!' => keep(),
            '#' => keep(),
            '=' => keep(),
            ':' => keep(),
            _ => match c.is_ascii_alphanumeric() {
                true => part(),
                false => match c.is_ascii_digit() {
                    true => part(),
                    false => keep()
                }
            }
        }
    }
}

//Working with Tuples got too tedious.
fn keep() -> (bool, bool) {
    return (true, true)
}

fn drop() -> (bool, bool) {
    return (true, false)
}

fn part() -> (bool, bool) {
    return (false, false)
}


pub struct TokenStream {
    tokens : Vec<String>,
    count : u64
}

impl TokenStream {

    pub fn next(&mut self) -> Option<String> {

        if (self.count as usize) >= self.tokens.len() {
            return None
        }
        else {
            let next_token : String = self.tokens[self.count as usize].clone();
            self.count += 1;
            println!("{}", next_token.clone());
            return Some(next_token)
        }
    }

    pub fn back(&mut self) {
        self.count -= 1;
    }
}


//A trait shared by all nodes in the syntax tree.
pub trait Node {

    fn parse(&mut self, stream : &mut TokenStream);

    fn run(&mut self, mut data : HashMap<String, engine::Type>) -> HashMap<String, engine::Type>;
}

//Represents a silicon file.
pub struct FileNode {
    children : Vec<Box<Node>>,
    main_functions : Vec<FunctionNode>
}

impl FileNode {

    fn new() -> FileNode {
        FileNode {children : vec![], main_functions : vec![]}
    }
}

impl Node for FileNode {


    fn parse(&mut self, stream : &mut TokenStream) {

        loop {
            match stream.next() {
                Some(next_tok) => match next_tok.as_str() {
                    "main" => {
                        let mut main_func : FunctionNode = FunctionNode::new_main();
                        main_func.parse(stream);
                        self.main_functions.push(main_func)
                    }
                    _ => { if !(next_tok.clone().trim().is_empty()) { panic!("Unexpected Token [{:?}]!", next_tok) } }
                }
                None => return
            }
        }
    }

    fn run(&mut self, data : HashMap<String, engine::Type>) -> HashMap<String, engine::Type> {
        self.main_functions[0].run(data)
    }
}

//Represents a function.

pub struct FunctionNode {
    children : Vec<Box<Node>>,
    identifier : String
}

impl FunctionNode {

    fn new() -> FunctionNode {
        FunctionNode { children : vec![], identifier : "".to_string()}
    }

    fn new_main() -> FunctionNode {
        FunctionNode { children : vec![], identifier : "<main>".to_string()}
    }
}

impl Node for FunctionNode {

    fn parse(&mut self, stream : &mut TokenStream) {
        loop {
            match stream.next() {
                Some(next_tok) => match next_tok.as_str() {
                    "{" => break,
                    _ => { if !(next_tok.clone().trim().is_empty()) { panic!("Unexpected Token [{:?}]!", next_tok) } }
                }
                None => panic!("Unexpected EOF!")
            }
        }

        loop {
            match stream.next() {
                Some(next_tok) => match next_tok.as_str() {
                    "}" => return,
                    "print" => {
                        let mut print_node : PrintNode = PrintNode::new();
                        print_node.parse(stream);
                        self.children.push(Box::new(print_node))
                    },
                    "var" => {
                        let mut var_node : VariableNode = VariableNode::new_var();
                        var_node.parse(stream);
                        self.children.push(Box::new(var_node))
                    }
                    _ => { if !(next_tok.clone().trim().is_empty()) { panic!("Unexpected Token [{:?}]!", next_tok) } }
                }
                None => panic!("Unexpected EOF!")
            }
        }
    }

    fn run(&mut self, mut data: HashMap<String, engine::Type>) -> HashMap<String, engine::Type> {

        for mut child in &mut self.children {
            data = child.run(data)
        }
        return data
    }
}

//Node representing a variable
pub struct VariableNode {
    constant : bool,
    id : String,
    var_type : String,
    val : Option<Box<Node>>
}

impl VariableNode {

    fn new_var() -> VariableNode {
        VariableNode {constant : false, id : "".to_string(), var_type : "".to_string(), val : None}
    }

    fn new_const() -> VariableNode {
        VariableNode {constant : false, id : "".to_string(), var_type : "".to_string(), val : None}
    }
}

impl Node for VariableNode {

    fn parse(&mut self, stream: &mut TokenStream) {

        loop {
            match stream.next() {
                Some(token) => {

                    for ch in token.chars() {
                        if ch.is_alphabetic() {
                            self.id = token.clone();
                            break
                        }
                    }

                    if self.id != "" {
                        break
                    }

                    if !(token.clone().trim().is_empty()) {
                        panic!("Unexpected Token [{:?}]!", token)
                    }
                }
                None => panic!("Unexpected EOF!")
            }
        }
        loop {
            match stream.next() {
                Some(token) => match token.as_str() {
                    ":" => break,
                    _ => { if !(token.clone().trim().is_empty()) { panic!("Unexpected Token [{:?}]!", token) } }
                }
                None => panic!("Unexpected EOF!")
            }
        }
        loop {
            match stream.next() {
                Some(token) => {
                    for ch in token.chars() {
                        if ch.is_alphabetic() {
                            self.var_type = token.clone();
                            break
                        }
                    }

                    if self.var_type != "" {
                        break
                    }

                    if !(token.clone().trim().is_empty()) {
                        panic!("Unexpected Token [{:?}]!", token)
                    }
                }
                None => panic!("Unexpected EOF!")
            }
        }
        loop {
            match stream.next() {
                Some(token) => match token.as_str() {
                    "=" => break,
                    _ => { if !(token.clone().trim().is_empty()) { panic!("Unexpected Token [{:?}]!", token) } }
                }
                None => panic!("Unexpected EOF!")
            }
        }

        loop {
            match stream.next() {
                Some(token) => match token.as_str() {
                    "\"" => {
                        let mut string_node : StringNode = StringNode::new();
                        string_node.parse(stream);
                        self.val = Some(Box::new(string_node));
                        break
                    },
                    _ => {
                        for c in token.chars() {
                            if c.is_ascii_alphabetic() {
                                let id = IdNode::new(token.clone());
                                self.val = Some(Box::new(id));
                                return;
                            } else if c.is_ascii_digit() {
                                let num = NumericNode::new(token.parse().unwrap());
                                self.val = Some(Box::new(num));
                                return;
                            } else if !(token.clone().trim().is_empty()){
                                panic!("Unexpected token [{:?}]!", token)
                            }
                        }
                    }
                }
                None => panic!("Unexpected EOF!")
            }
        }
    }

    fn run(&mut self, mut data: HashMap<String, engine::Type>) -> HashMap<String, engine::Type> {

        let this_val = match &mut self.val {
            Some(node) => {
                data = node.run(data);
                match data.get("<value>") {
                    Some(value) => match value {
                        engine::Type::String(thing) => engine::Type::String(thing.clone()),
                        engine::Type::Num(num) => engine::Type::Num(num.clone())
                    }
                    None => panic!("Missing value!")
                }
            },
            None => panic!("Missing value!")
        };

        data.remove("<value>");

        data.insert(
            self.id.clone(),
            this_val
        );
        return data
    }

}

//A node representing a string.

struct StringNode {
    thing : String
}

impl StringNode {

    fn new() -> StringNode {
        StringNode {thing : "".to_string()}
    }

}

impl Node for StringNode {

    fn parse(&mut self, stream: &mut TokenStream) {
        loop {
            match stream.next() {
                Some(next_tok) => match next_tok.as_str() {
                    "\"" => break,
                    _ => self.thing.push_str(next_tok.as_str())
                },
                None => panic!("Unexpected EOF!")
            }
        }
    }

    fn run(&mut self, mut data : HashMap<String, engine::Type>) -> HashMap<String, engine::Type> {
        data.insert("<value>".to_string(), engine::Type::String(self.thing.clone()));
        return data
    }

}

//Represents an identifier.

pub struct IdNode {
    id : String
}

impl IdNode {

    fn new(the_id : String) -> IdNode {
        IdNode {id : the_id.to_string()}
    }

}

impl Node for IdNode {

    fn parse(&mut self, stream: &mut TokenStream) {
        return;
    }

    fn run(&mut self, mut data : HashMap<String, engine::Type>) -> HashMap<String, engine::Type> {

        let value = match data.get(self.id.as_str()) {
            Some(value) => match value {
                engine::Type::String(thing) => engine::Type::String(thing.clone()),
                engine::Type::Num(num) => engine::Type::Num(num.clone())
            },
            None => panic!("[{}] is not defined in the current scope!", self.id)
        };

        data.insert("<value>".to_string(), value);

        return data
    }


}

//Represents a numeric type.

pub struct NumericNode {
    value : u64
}

impl NumericNode {

    pub fn new(val : u64) -> NumericNode {
        NumericNode {value : val}
    }
}

impl Node for NumericNode {

    fn parse(&mut self, stream: &mut TokenStream) {
        unimplemented!()
    }

    fn run(&mut self, mut data : HashMap<String, engine::Type>) -> HashMap<String, engine::Type> {
        data.insert("<value>".to_string(), engine::Type::Num(self.value.clone()));
        return data
    }
}

//Temporarily represents the print keyword, which will be replaced with an actual print function later on.

pub struct PrintNode {
    out : Option<Box<Node>>
}

impl PrintNode {

    fn new() -> PrintNode {
        PrintNode{out: None}
    }

}

impl Node for PrintNode {

    fn parse(&mut self, stream : &mut TokenStream) {

        loop {
            match stream.next() {
                Some(next_tok) => match next_tok.as_str() {
                    "\"" => {
                        let mut string_node : StringNode = StringNode::new();
                        string_node.parse(stream);
                        self.out = Some(Box::new(string_node))
                    },
                    " " => {}
                    "\n" => return,
                    _ => {
                        for c in next_tok.chars() {
                            if c.is_ascii_alphabetic() {
                                let id = IdNode::new(next_tok.clone());
                                self.out = Some(Box::new(id));
                                break
                            } else if c.is_ascii_digit() {
                                let mut num = NumericNode::new(next_tok.parse().unwrap());
                                self.out = Some(Box::new(num));
                                break
                            } else {
                                panic!("Unexpected token [{:?}]!", next_tok)
                            }
                        }
                    }
                },
                None => panic!("Unexpected EOF!")
            }
        }
    }

    fn run(&mut self, mut data: HashMap<String, engine::Type>) -> HashMap<String, engine::Type> {
        match &mut self.out {
            Some(node) => {
                data = node.run(data);

                match data.get("<value>") {
                    Some(value) => match value {
                        engine::Type::String(thing) => println!("{}", thing),
                        engine::Type::Num(num) => println!("{}", num.to_string())
                    }
                    None => panic!("Something went wrong!")
                }
            },
            None => panic!("Something broke!")
        }

        data.remove("<value>");

        return data
    }
}