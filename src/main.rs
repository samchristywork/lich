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

fn eval_list(nodes: &[Node], env: &mut Environment) -> Node {
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
            } else if operator == "let" {
                if rest.len() == 2 {
                    let bindings = &rest[0];
                    let body = &rest[1];

                    if let Node::List(bindings_list) = bindings {
                        let mut new_env = Environment {
                            parent: Some(Box::new(env.clone())),
                            variables: std::collections::HashMap::new(),
                        };

                        for binding in bindings_list {
                            if let Node::List(binding_pair) = binding {
                                if binding_pair.len() == 2 {
                                    let variable = &binding_pair[0];
                                    let value = eval(&binding_pair[1], env);
                                    new_env.variables.insert(variable.clone(), value);
                                } else {
                                    panic!("Invalid binding pair");
                                }
                            } else {
                                panic!("Invalid binding");
                            }
                        }

                        return eval(body, &mut new_env);
                    }
                }
                panic!("Invalid arguments for let");
            } else {
                let function = env.lookup(first);
                let arguments = rest.iter().map(|n| eval(n, env)).collect::<Vec<_>>();
                return apply(&function, &arguments, env);
            }
        }
        _ => panic!("Unknown operator {first:?}"),
    }
}

fn eval(node: &Node, env: &mut Environment) -> Node {
    match node {
        Node::Symbol(_) => env.lookup(node),
        Node::Number(_) | Node::String(_) | Node::Bool(_) => node.clone(),
        Node::List(nodes) => eval_list(nodes, env),
    }
}

fn apply(function: &Node, arguments: &[Node], env: &mut Environment) -> Node {
    match function {
        Node::List(nodes) => {
            if let Node::Symbol(s) = &nodes[0] {
                if s == "lambda" {
                    if let Node::List(params) = &nodes[1] {
                        assert!((params.len() == arguments.len()), "Argument count mismatch");
                        let mut new_env = Environment {
                            parent: Some(Box::new(env.clone())),
                            variables: std::collections::HashMap::new(),
                        };
                        for (param, arg) in params.iter().zip(arguments) {
                            new_env.variables.insert(param.clone(), arg.clone());
                        }
                        return eval(&nodes[2], &mut new_env);
                    }
                }
            }
            panic!("Function application not implemented");
        }
        Node::Symbol(s) => {
            if s == "+" {
                Node::Number(
                    arguments
                        .iter()
                        .map(|n| {
                            if let Node::Number(num) = n {
                                *num
                            } else {
                                panic!("Invalid argument for addition {n:?}");
                            }
                        })
                        .sum(),
                )
            } else if s == "-" {
                if arguments.len() == 1 {
                    if let Node::Number(num) = &arguments[0] {
                        return Node::Number(-num);
                    }
                } else if arguments.len() == 2 {
                    if let (Node::Number(a), Node::Number(b)) = (&arguments[0], &arguments[1]) {
                        return Node::Number(a - b);
                    }
                }
                panic!("Invalid arguments for subtraction");
            } else if s == "*" {
                Node::Number(
                    arguments
                        .iter()
                        .map(|n| {
                            if let Node::Number(num) = n {
                                *num
                            } else {
                                panic!("Invalid argument for multiplication {n:?}");
                            }
                        })
                        .product(),
                )
            } else if s == "=" {
                if arguments.len() == 2 {
                    return Node::Bool(arguments[0] == arguments[1]);
                }
                panic!("Invalid arguments for equality check");
            } else if s == "write" {
                for arg in arguments {
                    print!("{arg}");
                }
                return Node::Bool(true);
            } else if s == "write-line" {
                for arg in arguments {
                    print!("{arg}");
                }
                println!();
                return Node::Bool(true);
            } else if s == "car" {
                if arguments.len() == 1 {
                    if let Node::List(list) = &arguments[0] {
                        if list.is_empty() {
                            return Node::Symbol("nil".to_string());
                        }
                        return list[0].clone();
                    }
                }
                panic!("Invalid arguments for car");
            } else if s == "cdr" {
                if arguments.len() == 1 {
                    if let Node::List(list) = &arguments[0] {
                        if list.len() > 1 {
                            return Node::List(list[1..].to_vec());
                        } else if list.len() <= 1 {
                            return Node::List(vec![]);
                        }
                    }
                }
                panic!("Invalid arguments for cdr");
            } else if s == "cons" {
                if arguments.len() == 2 {
                    if let Node::List(list) = &arguments[1] {
                        let mut new_list = vec![arguments[0].clone()];
                        new_list.extend_from_slice(list);
                        return Node::List(new_list);
                    }
                }
                panic!("Invalid arguments for cons");
            } else if s == "begin" {
                let mut result = Node::Bool(false);
                for arg in arguments {
                    result = eval(arg, env);
                }

                return result;
            } else if s == "print-env" {
                println!("{env}");
                return Node::Bool(true);
            } else if s == "nil" {
                return Node::List(vec![]);
            } else if s == "length" {
                if arguments.len() == 1 {
                    if let Node::List(list) = &arguments[0] {
                        return Node::Number(
                            i64::try_from(list.len()).expect("Failed to convert length"),
                        );
                    }
                }
                panic!("Invalid arguments for length: {:?}", &arguments[0]);
            } else if s == "null?" {
                if arguments.len() == 1 {
                    if let Node::List(list) = &arguments[0] {
                        return Node::Bool(list.is_empty());
                    }

                    return Node::Bool(false);
                }
                panic!("Invalid arguments for null?");
            } else if s == "concat" {
                if arguments.len() == 2 {
                    if let (Node::String(s1), Node::String(s2)) =
                        (&arguments[0], &arguments[1])
                    {
                        return Node::String(format!("{s1}{s2}"));
                    } else if let (Node::List(l1), Node::List(l2)) =
                        (&arguments[0], &arguments[1])
                    {
                        let mut new_list = l1.clone();
                        new_list.extend_from_slice(l2);
                        return Node::List(new_list);
                    }
                }
                panic!("Invalid arguments for concat");
            } else if s == "load" {
                if arguments.len() == 1 {
                    let filename = if let Node::String(s) = &arguments[0] {
                        s
                    } else {
                        panic!("Invalid argument for load");
                    };
                    let input_string =
                        std::fs::read_to_string(filename).expect("Failed to read input file");
                    let expressions = parse(&input_string);
                    for expression in expressions {
                        eval(&expression, env);
                    }
                    return Node::Bool(true);
                }
                panic!("Invalid arguments for load");
            } else if s == "type?" {
                if arguments.len() == 1 {
                    match &arguments[0] {
                        Node::Number(_) => return Node::String("number".to_string()),
                        Node::String(_) => return Node::String("string".to_string()),
                        Node::Bool(_) => return Node::String("bool".to_string()),
                        Node::List(_) => return Node::String("list".to_string()),
                        Node::Symbol(_) => return Node::String("symbol".to_string()),
                    }
                }
                panic!("Invalid arguments for type?");
            } else if s == "list" {
                return Node::List(arguments.to_vec());
            } else if s == "number->string" {
                if arguments.len() == 1 {
                    if let Node::Number(n) = &arguments[0] {
                        return Node::String(n.to_string());
                    }
                }
                panic!("Invalid arguments for number->string");
            } else if s == "fold" {
                if arguments.len() == 3 {
                    let function = &arguments[0];
                    let initial_value = &arguments[1];
                    let list = &arguments[2];

                    if let Node::List(l) = list {
                        let mut result = initial_value.clone();
                        for item in l {
                            result = apply(function, &[result, item.clone()], env);
                        }
                        return result;
                    }
                }
                panic!("Invalid arguments for fold");
            } else if s == "zip" {
                if arguments.len() == 2 {
                    let list1 = &arguments[0];
                    let list2 = &arguments[1];

                    if let (Node::List(l1), Node::List(l2)) =
                        (list1, list2)
                    {
                        let mut zipped = Vec::new();
                        for (item1, item2) in l1.iter().zip(l2.iter()) {
                            zipped.push(Node::List(vec![item1.clone(), item2.clone()]));
                        }
                        return Node::List(zipped);
                    }
                }
                panic!("Invalid arguments for zip");
            } else if s == "time-ms" {
                if arguments.len() == 1 {
                    let start = std::time::Instant::now();
                    eval(&arguments[0], env);
                    let duration = start.elapsed();
                    return Node::Number(duration.as_millis().try_into().expect("Failed to convert duration"));
                }
                panic!("Invalid arguments for time-ms");
            } else if s == "range" {
                if arguments.len() == 2 {
                    if let (Node::Number(start), Node::Number(end)) =
                        (&arguments[0], &arguments[1])
                    {
                        let mut range = Vec::new();
                        for i in *start..*end {
                            range.push(Node::Number(i));
                        }
                        return Node::List(range);
                    }
                } else if arguments.len() == 1 {
                    if let Node::Number(end) = &arguments[0] {
                        let mut range = Vec::new();
                        for i in 0..*end {
                            range.push(Node::Number(i));
                        }
                        return Node::List(range);
                    }
                }
                panic!("Invalid arguments for range");
            } else if s == "for-each" {
                if arguments.len() == 2 {
                    let function = &arguments[0];
                    let list = &arguments[1];

                    if let Node::List(l) = list {
                        for item in l {
                            apply(function, &[item.clone()], env);
                        }
                        return Node::Bool(true);
                    }
                }
                panic!("Invalid arguments for for-each");
            } else if s == "among" {
                if arguments.len() == 2 {
                    let list = &arguments[0];
                    let value = &arguments[1];

                    if let Node::List(l) = list {
                        for item in l {
                            if item == value {
                                return Node::Bool(true);
                            }
                        }
                        return Node::Bool(false);
                    }
                }
                panic!("Invalid arguments for among");
            } else if s == "map" {
                if arguments.len() == 2 {
                    let function = &arguments[0];
                    let list = &arguments[1];

                    if let Node::List(l) = list {
                        let mut mapped = Vec::new();
                        for item in l {
                            mapped.push(apply(function, &[item.clone()], env));
                        }
                        return Node::List(mapped);
                    }
                }

                panic!("Invalid arguments for map");
            } else if s == "version" {
                return Node::String(format!("Lich version {}", env!("CARGO_PKG_VERSION")).to_string());
            } else {
                panic!("Unknown function {s}");
            }
        }
        _ => panic!("Function application not implemented"),
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
