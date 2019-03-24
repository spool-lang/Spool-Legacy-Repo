use tokesies::*;

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
                false => keep()
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

    fn run(&mut self);
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

    fn run(&mut self) {
        self.main_functions[0].run()
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
                    _ => { if !(next_tok.clone().trim().is_empty()) { panic!("Unexpected Token [{:?}]!", next_tok) } }
                }
                None => panic!("Unexpected EOF!")
            }
        }
    }

    fn run(&mut self) {

        for mut child in &mut self.children {
            child.run()
        }

    }
}


//Temporarily represents the print keyword, which will be replaced with an actual print function later on.

pub struct PrintNode {
    out : String
}

impl PrintNode {

    fn new() -> PrintNode {
        PrintNode{out: "".to_string()}
    }

}

impl Node for PrintNode {

    fn parse(&mut self, stream : &mut TokenStream) {

        loop {
            match stream.next() {
                Some(next_tok) => match next_tok.as_str() {
                    "\"" => break,
                    " " => {}
                    _ => panic!("Unexpected token [{:?}]!", next_tok)
                },
                None => panic!("Unexpected EOF!")
            }
        }

        loop {
            match stream.next() {
                Some(next_tok) => match next_tok.as_str() {
                    "\"" => break,
                    _ => self.out.push_str(next_tok.as_str())
                },
                None => panic!("Unexpected EOF!")
            }
        }
    }

    fn run(&mut self) {
        println!("{}", self.out)
    }
}