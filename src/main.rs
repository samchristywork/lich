const GREY: &str = "\x1b[90m";
const NORMAL: &str = "\x1b[0m";

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
enum Node {
    Symbol(String),
    Number(i32),
    String(String),
    Bool(bool),
    List(Vec<Node>),
}

#[derive(Debug, Clone)]
struct Environment {
    parent: Option<Box<Environment>>,
    variables: std::collections::HashMap<Node, Node>,
}

#[derive(Debug)]
enum Token {
    Symbol(String),
    Number(i32),
    String(String),
    Bool(bool),
    LParen,
    RParen,
}

fn main() {
    let mut env = Environment {
        parent: None,
        variables: std::collections::HashMap::new(),
    };

    let args = std::env::args().collect::<Vec<_>>();

    if args.len() > 1 {
    } else {
    }
}
