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
