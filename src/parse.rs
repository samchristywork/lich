use crate::Environment;
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

fn parens_are_balanced(tokens: &Vec<Token>) -> bool {
    let mut count = 0;
    for token in tokens {
        match token {
            Token::LParen => count += 1,
            Token::RParen => count -= 1,
            _ => {}
        }

        if count < 0 {
            return false;
        }
    }

    count == 0
}

macro_rules! is_symbol_char {
    ($c:expr) => {
        $c.is_alphanumeric() || "!$%&*+-./:<=>?\\^_{}|~".contains($c)
    };
}

pub fn parse(input: &str) -> Result<Vec<Node>, String> {
    let tokens = tokenize(input)?;

    if !parens_are_balanced(&tokens) {
        return Err("Missing parenthesis".to_string());
    }

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
                    return Err("Unmatched closing parenthesis".to_string());
                }
            }
            Token::Symbol(s) => current_list.push(Node::Symbol(s.clone())),
            Token::Number(n) => current_list.push(Node::Number(n)),
            Token::Text(s) => current_list.push(Node::Text(s.clone())),
            Token::Bool(b) => current_list.push(Node::Bool(b)),
        }
    }

    Ok(current_list)
}

fn is_valid_number(value: &str) -> Result<bool, String> {
    assert!(
        !value.is_empty(),
        "Empty string is not a valid number or symbol"
    );

    Ok(if value == "-" {
        false
    } else if value.len() == 1 {
        value
            .chars()
            .next()
            .ok_or_else(|| "Could not parse number".to_string())?
            .is_ascii_digit()
    } else if value[0..1] == *"-" {
        value[1..].chars().all(|c| c.is_ascii_digit())
    } else {
        value.chars().all(|c| c.is_ascii_digit())
    })
}

fn tokenize(source: &str) -> Result<Vec<Token>, String> {
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
                let mut found_quote = false;
                while let Some(&(_, next_c)) = chars.peek() {
                    chars.next();
                    if next_c == '`' {
                        found_quote = true;
                        break;
                    }

                    value.push(next_c);
                }
                if !found_quote {
                    return Err("Missing backquote.".to_string());
                }
                tokens.push(Token::Text(value.replace("\\n", "\n")));
            }
            '"' => {
                let mut value = String::new();
                let mut found_quote = false;
                while let Some(&(_, next_c)) = chars.peek() {
                    chars.next();
                    if next_c == '"' {
                        found_quote = true;
                        break;
                    }

                    value.push(next_c);
                }
                if !found_quote {
                    return Err("Missing quote.".to_string());
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
                if is_valid_number(&value)? {
                    tokens.push(Token::Number(match value.parse::<i64>() {
                        Ok(n) => n,
                        Err(_) => return Err("Could not parse number".to_string()),
                    }));

                // Booleans are a strict subset of symbols, so we check for booleans next
                } else if value == "true" {
                    tokens.push(Token::Bool(true));
                } else if value == "false" {
                    tokens.push(Token::Bool(false));
                } else {
                    tokens.push(Token::Symbol(value));
                }
            }
            _ => return Err(format!("Unexpected character: {c}")),
        }
    }

    Ok(tokens)
}

pub fn fn_tokenize(arguments: &[Node], _: &mut Environment) -> Result<Node, String> {
    if arguments.len() == 1 {
        if let Node::Text(text) = &arguments[0] {
            let tokens = tokenize(text)?;
            let mut result = Vec::new();
            for token in tokens {
                result.push(Node::Text(token.to_string()));
            }
            return Ok(Node::List(result));
        }
    }
    Err(format!(
        "Invalid arguments for tokenize: {:?}",
        &arguments[0]
    ))
}

pub fn fn_parse(arguments: &[Node], _: &mut Environment) -> Result<Node, String> {
    if arguments.len() == 1 {
        if let Node::Text(text) = &arguments[0] {
            let parsed = parse(text)?;
            return Ok(Node::List(parsed));
        }
    }
    Err(format!("Invalid arguments for parse: {:?}", &arguments[0]))
}
