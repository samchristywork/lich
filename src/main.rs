use std::fmt;

const GREY: &str = "\x1b[90m";
const NORMAL: &str = "\x1b[0m";

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
enum Node {
    Symbol(String),
    Number(i64),
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

#[derive(Clone)]
struct Environment {
    parent: Option<Box<Environment>>,
    variables: std::collections::HashMap<Node, Node>,
}

impl fmt::Display for Environment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::new();
        for (key, value) in &self.variables {
            result.push_str(&format!("{key} => {value}\n"));
        }
        if let Some(parent) = &self.parent {
            result.push_str(&parent.to_string());
        }
        write!(f, "{}", result)
    }
}

impl Environment {
    fn lookup(&self, node: &Node) -> Node {
        if let Some(value) = self.variables.get(node) {
            return value.clone();
        } else if let Some(parent) = &self.parent {
            return parent.lookup(node);
        }

        panic!("Undefined variable: {node:?}");
    }
}

enum Token {
    Symbol(String),
    Number(i64),
    String(String),
    Bool(bool),
    LParen,
    RParen,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Symbol(s) => write!(f, "Symbol({s})"),
            Token::Number(n) => write!(f, "Number({n})"),
            Token::String(s) => write!(f, "String({s})"),
            Token::Bool(b) => write!(f, "Bool({b})"),
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
        }
    }
}

fn is_symbol_char(c: char) -> bool {
    c.is_alphanumeric() || "!$%&*+-./:<=>?\\^_{}|~".contains(c)
}

fn is_valid_number(value: &str) -> bool {
    assert!(
        !value.is_empty(),
        "Empty string is not a valid number or symbol"
    );

    if value == "-" {
        false
    } else if value.len() == 1 {
        value
            .chars()
            .next()
            .expect("Single character")
            .is_ascii_digit()
    } else if value[0..1] == *"-" {
        if value[1..].chars().all(|c| c.is_ascii_digit()) {
            true
        } else {
            false
        }
    } else if value.chars().all(|c| c.is_ascii_digit()) {
        true
    } else {
        false
    }
}

fn tokenize(source: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = source.char_indices().peekable();

    while let Some((_, c)) = chars.next() {
        match c {
            ';' => {
                while let Some(&(_, next_c)) = chars.peek() {
                    if next_c == '\n' {
                        break;
                    }
                    chars.next();
                }
            }
            '(' => tokens.push(Token::LParen),
            ')' => tokens.push(Token::RParen),
            c if c.is_whitespace() => {} // Skip whitespace
            '`' => {
                let mut value = String::new();
                while let Some(&(_, next_c)) = chars.peek() {
                    chars.next();
                    if next_c == '`' {
                        break;
                    }

                    value.push(next_c);
                }
                tokens.push(Token::String(value.replace("\\n", "\n")));
            }
            '"' => {
                let mut value = String::new();
                while let Some(&(_, next_c)) = chars.peek() {
                    chars.next();
                    if next_c == '"' {
                        break;
                    }

                    value.push(next_c);
                }
                tokens.push(Token::String(value.replace("\\n", "\n")));
            }
            c if is_symbol_char(c) => {
                let mut value = String::from(c);
                while let Some(&(_, next_c)) = chars.peek() {
                    if is_symbol_char(next_c) {
                        value.push(next_c);
                        chars.next();
                    } else {
                        break;
                    }
                }

                // Numbers are a strict subset of symbols, so we check for numbers first
                if is_valid_number(&value) {
                    tokens.push(Token::Number(value.parse::<i64>().expect("Failed to parse number")));

                // Booleans are a strict subset of symbols, so we check for booleans next
                } else if value == "true" {
                    tokens.push(Token::Bool(true));
                } else if value == "false" {
                    tokens.push(Token::Bool(false));
                } else {
                    tokens.push(Token::Symbol(value));
                }
            }
            _ => panic!("Unexpected character: {c}, {:?}", c),
        }
    }

    tokens
}

fn parse_tokens(tokens: &[Token]) -> Vec<Node> {
    let mut stack = Vec::new();
    let mut current_list = Vec::new();

    for token in tokens {
        match token {
            Token::LParen => {
                stack.push(current_list);
                current_list = Vec::new();
            }
            Token::RParen => {
                if let Some(last_list) = stack.pop() {
                    let list_node = Node::List(current_list);
                    current_list = last_list;
                    current_list.push(list_node);
                } else {
                    panic!("Unmatched closing parenthesis");
                }
            }
            Token::Symbol(s) => current_list.push(Node::Symbol(s.clone())),
            Token::Number(n) => current_list.push(Node::Number(*n)),
            Token::String(s) => current_list.push(Node::String(s.clone())),
            Token::Bool(b) => current_list.push(Node::Bool(*b)),
        }
    }

    current_list
}

fn parse(input: &str) -> Vec<Node> {
    let tokens = tokenize(input);
    parse_tokens(&tokens)
}

fn eval(node: &Node, env: &mut Environment) -> Node {
    match node {
        Node::Symbol(_) => env.lookup(node),
        Node::Number(_) | Node::String(_) | Node::Bool(_) => node.clone(),
        Node::List(nodes) => {
            if nodes.is_empty() {
                return Node::List(vec![]);
            }

            let first = &nodes[0];
            let rest = &nodes[1..];

            match first {
                Node::Symbol(s) => {
                    let operator = s.as_str();
                    if operator == "quote" {
                        rest[0].clone()
                    } else if operator == "if" {
                        let condition = eval(&rest[0], env);
                        match condition {
                            Node::Bool(true) => eval(&rest[1], env),
                            Node::Bool(false) => eval(&rest[2], env),
                            _ => panic!("Condition must be a boolean"),
                        }
                    } else if operator == "cond" {
                        for condition in rest {
                            if let Node::List(conditions) = condition {
                                if conditions.len() == 2 {
                                    let cond = eval(&conditions[0], env);
                                    if cond == Node::Bool(true) {
                                        return eval(&conditions[1], env);
                                    }
                                } else {
                                    panic!("Invalid cond clause");
                                }
                            } else {
                                panic!("Invalid cond clause");
                            }
                        }

                        panic!("No true condition found in cond");
                    } else if operator == "define" {
                        let variable = &rest[0];
                        let value = eval(&rest[1], env);
                        env.variables.insert(variable.clone(), value.clone());

                        return value;
                    } else if operator == "lambda" {
                        if rest.len() != 2 {
                            panic!("Invalid arguments for lambda");
                        }
                        let parameters = rest[0].clone();
                        let body = rest[1].clone();

                        let lambda =
                            Node::List(vec![Node::Symbol("lambda".to_string()), parameters, body]);
                        return lambda;
                    } else {
                        let function = env.lookup(first);
                        let arguments = rest.iter().map(|n| eval(n, env)).collect::<Vec<_>>();
                        return apply(&function, &arguments, env);
                    }
                }
                _ => panic!("Unknown operator {first:?}"),
            }
        }
    }
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
