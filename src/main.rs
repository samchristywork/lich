use std::fmt;

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

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let res = match self {
            Node::Number(n) => n.to_string(),
            Node::Bool(b) => b.to_string(),
            Node::String(s) | Node::Symbol(s) => s.clone(),
            Node::List(nodes) => {
                let mut result = String::new();
                result.push('(');
                result.push_str(
                    &nodes
                    .iter()
                    .map(Node::to_string)
                    .collect::<Vec<_>>()
                    .join(" "),
                );
                result.push(')');
                result
            }
        };

        if res.is_empty() {
            write!(f, "nil")
        } else {
            write!(f, "{res}")
        }
    }
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
