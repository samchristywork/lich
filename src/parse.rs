use crate::Node;

enum Token {
    Symbol(String),
    Number(i64),
    Text(String),
    Bool(bool),
    LParen,
    RParen,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Symbol(s) => write!(f, "Symbol({s})"),
            Self::Number(n) => write!(f, "Number({n})"),
            Self::Text(s) => write!(f, "Text({s})"),
            Self::Bool(b) => write!(f, "Bool({b})"),
            Self::LParen => write!(f, "("),
            Self::RParen => write!(f, ")"),
        }
    }
}

macro_rules! is_symbol_char {
    ($c:expr) => {
        $c.is_alphanumeric() || "!$%&*+-./:<=>?\\^_{}|~".contains($c)
    };
}

pub fn parse(input: &str) -> Vec<Node> {
    let tokens = tokenize(input);

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
            Token::Number(n) => current_list.push(Node::Number(n)),
            Token::Text(s) => current_list.push(Node::Text(s.clone())),
            Token::Bool(b) => current_list.push(Node::Bool(b)),
        }
    }

    current_list
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
        value[1..].chars().all(|c| c.is_ascii_digit())
    } else {
        value.chars().all(|c| c.is_ascii_digit())
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
                tokens.push(Token::Text(value.replace("\\n", "\n")));
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
                tokens.push(Token::Text(value.replace("\\n", "\n")));
            }
            c if is_symbol_char!(c) => {
                let mut value = String::from(c);
                while let Some(&(_, next_c)) = chars.peek() {
                    if is_symbol_char!(next_c) {
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
            _ => panic!("Unexpected character: {c}, {c:?}"),
        }
    }

    tokens
}
