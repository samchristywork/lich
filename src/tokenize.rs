use crate::Range;
use crate::Token;
use crate::TokenKind;

fn is_symbol_char(c: char) -> bool {
    c.is_alphanumeric()
        || c == '!'
        || c == '$'
        || c == '%'
        || c == '&'
        || c == '*'
        || c == '+'
        || c == '-'
        || c == '.'
        || c == '/'
        || c == ':'
        || c == '<'
        || c == '='
        || c == '>'
        || c == '?'
        || c == '\''
        || c == '^'
        || c == '_'
        || c == '{'
        || c == '|'
        || c == '}'
        || c == '~'
}

macro_rules! push_token {
    ($tokens:expr, $value:expr, $kind:expr, $start:expr, $end:expr) => {
        $tokens.push(Token {
            value: $value.to_string(),
            kind: $kind,
            range: Range {
                start: $start,
                end: $end,
            },
        })
    };
}

fn is_valid_number(value: &str) -> bool {
    assert!(!value.is_empty(), "Empty string is not a valid number or symbol");

    if value == "-" {
        return false;
    } else if value.len() == 1 {
        return value.chars().next().expect("Single character")
            .is_ascii_digit();
    }

    if value[0..1] == *"-" {
        if value[1..].chars().all(|c| c.is_ascii_digit()) {
            return true;
        }
    } else if value.chars().all(|c| c.is_ascii_digit()) {
        return true;
    }

    false
}

pub fn tokenize(source: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = source.char_indices().peekable();

    while let Some((start, c)) = chars.next() {
        let mut end = start;
        match c {
            '(' => push_token!(tokens, "(", TokenKind::LParen, start, end),
            ')' => push_token!(tokens, ")", TokenKind::RParen, start, end),
            c if c.is_whitespace() => {} // Skip whitespace
            ';' => {
                while let Some(&(_, next_c)) = chars.peek() {
                    if next_c == '\n' {
                        break;
                    }
                    chars.next();
                }
            }
            '`' => {
                let mut value = String::new();
                while let Some(&(next_start, next_c)) = chars.peek() {
                    chars.next();
                    end = next_start;
                    if next_c == '`' {
                        break;
                    }

                    value.push(next_c);
                }
                push_token!(tokens, value, TokenKind::Text, start, end);
            }
            '"' => {
                let mut value = String::new();
                while let Some(&(next_start, next_c)) = chars.peek() {
                    chars.next();
                    end = next_start;
                    if next_c == '"' {
                        break;
                    }

                    value.push(next_c);
                }
                push_token!(tokens, value, TokenKind::Text, start, end);
            }
            ':' => {
                let mut value = String::from(c);
                while let Some(&(next_start, next_c)) = chars.peek() {
                    if is_symbol_char(next_c) {
                        value.push(next_c);
                        chars.next();
                        end = next_start;
                    } else {
                        break;
                    }
                }
                push_token!(tokens, value, TokenKind::Atom, start, end);
            }
            c if is_symbol_char(c) => {
                let mut value = String::from(c);
                while let Some(&(next_start, next_c)) = chars.peek() {
                    if is_symbol_char(next_c) {
                        value.push(next_c);
                        chars.next();
                        end = next_start;
                    } else {
                        break;
                    }
                }

                // Numbers are a strict subset of symbols, so we check for numbers first
                if is_valid_number(&value) {
                    push_token!(tokens, value, TokenKind::Number, start, end);
                } else {
                    push_token!(tokens, value, TokenKind::Symbol, start, end);
                }
            }
            _ => panic!("Unexpected character: {c}"),
        }
    }

    tokens
}
