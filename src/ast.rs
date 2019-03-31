use std::str::Chars;
use std::collections::HashMap;
use crate::engine;
use std::process::id;
use core::fmt::Alignment::Right;

use crate::lex::Token;

pub fn build(tokens : Vec<Token>) -> FileNode {

    let mut token_stream : TokenStream = TokenStream {tokens, count : 0};
    let mut file_node : FileNode = FileNode::new();

    file_node.parse(&mut token_stream);

    return file_node
}

pub struct TokenStream {
    tokens : Vec<Token>,
    count : u64
}

impl TokenStream {

    pub fn next(&mut self) -> Option<Token> {

        if (self.count as usize) >= self.tokens.len() {
            return None
        }
        else {
            let next_token : Token = self.tokens[self.count as usize].clone();
            self.count += 1;
            //println!("{}", next_token.clone());
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
                Some(token) => match token {
                    Token::Main => {
                        let mut main_func : FunctionNode = FunctionNode::new_main();
                        main_func.parse(stream);
                        self.main_functions.push(main_func);
                    }
                    _ => panic!("Unexpected token!")
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
                Some(token) => match token {
                    Token::BlockIn => break,
                    _ => panic!("Unexpected token!")
                },
                None => panic!("Unexpected EOF!")
            }
        }

        loop {
            match stream.next() {
                Some(token) => match token {
                    Token::BlockOut => return,
                    Token::Print => {
                        let mut print_node : PrintNode = PrintNode::new();
                        print_node.parse(stream);
                        self.children.push(Box::new(print_node))
                    },
                    Token::Var => {
                        let mut var_node : VariableNode = VariableNode::new_var();
                        var_node.parse(stream);
                        self.children.push(Box::new(var_node))
                    }
                    Token::Newline => {}
                    _ => panic!("Unexpected token {}!", token.to_string())
                },
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

        match stream.next() {
            Some(token) => match token {
                Token::Word(the_word) => {self.id = the_word}
                _ => panic!("Unexpected token!")
            },
            None => panic!("Unexpected EOF!")
        }

        match stream.next() {
            Some(token) => match token {
                Token::Colin => {}
                _ => panic!("Unexpected token!")
            },
            None => panic!("Unexpected EOF!")
        }

        match stream.next() {
            Some(token) => match token {
                Token::Word(the_word) => {self.var_type = the_word}
                _ => panic!("Unexpected token!")
            },
            None => panic!("Unexpected EOF!")
        }

        match stream.next() {
            Some(token) => match token {
                Token::Assign => {}
                _ => panic!("Unexpected token!")
            },
            None => panic!("Unexpected EOF!")
        }

        match stream.next() {
            Some(token) => match token {
                Token::Str(a_string) => {
                    let str_node = StringNode::new(a_string.clone());
                    self.val = Some(Box::new(str_node))
                },
                Token::Word(a_word) => {
                    let id_node = IdNode::new(a_word);
                    self.val = Some(Box::new(id_node));
                }
                Token::Num(a_num) => {
                    let num_node : NumericNode = NumericNode::new(a_num.parse().unwrap());
                    self.val = Some(Box::new(num_node));
                }
                _ => panic!("Unexpected token!")
            },
            None => panic!("Unexpected EOF!")
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
    the_str: String
}

impl StringNode {

    fn new(the_str : String) -> StringNode {
        StringNode { the_str }
    }

}

impl Node for StringNode {

    fn parse(&mut self, stream: &mut TokenStream) {

    }

    fn run(&mut self, mut data : HashMap<String, engine::Type>) -> HashMap<String, engine::Type> {
        data.insert("<value>".to_string(), engine::Type::String(self.the_str.clone()));
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

    }

    fn run(&mut self, mut data : HashMap<String, engine::Type>) -> HashMap<String, engine::Type> {
        data.insert("<value>".to_string(), engine::Type::Num(self.value.clone()));
        return data
    }
}

//Nodes for expressions

pub struct ExpressionNode {
    entry_point : Option<Box<Node>>
}

impl ExpressionNode {

    fn new(tokens : Vec<String>) -> ExpressionNode {





        ExpressionNode {entry_point : None}
    }
}

//Represents operations

enum Operation {
    Add
}

impl Operation {

    fn operate(self, left : engine::Type, right : engine::Type) -> engine::Type {

        match self {
            Operation::Add => {
                match left {
                    engine::Type::Num(lnum) => {
                        match right {
                            engine::Type::Num(rnum) => {
                                return engine::Type::Num((lnum.clone()) + (rnum.clone()))
                            }
                            _ => panic!("You can only do arithmetic on numbers!")
                        }
                    }
                    _ => panic!("You can only do arithmetic on numbers!")
                }
            }
        }
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

        match stream.next() {
            Some(token) => match token {
                Token::Str(a_string) => {
                    let str_node = StringNode::new(a_string.clone());
                    self.out = Some(Box::new(str_node))
                },
                Token::Word(a_word) => {
                    let id_node = IdNode::new(a_word);
                    self.out = Some(Box::new(id_node));
                }
                Token::Num(a_num) => {
                    let num_node : NumericNode = NumericNode::new(a_num.parse().unwrap());
                    self.out = Some(Box::new(num_node));
                }
                _ => panic!("Unexpected token!")
            },
            None => panic!("Unexpected EOF!")
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