use crate::Environment;
use crate::Node;
use crate::Value;
use crate::control::fn_false;
use crate::control::fn_true;
use crate::evaluate_node;
use crate::expect_n_args;

pub fn fn_join(args: &[Node], env: &mut Environment) -> Node {
    expect_n_args!(args, 2);

    if let Value::Text(separator) = evaluate_node(&args[0], env).value {
        if matches!(&args[1].value, Value::List()) {
            let mut elements = Vec::new();
            for child in &args[1].children {
                if let Value::Text(text) = evaluate_node(child, env).value {
                    elements.push(text);
                } else {
                    panic!("Invalid argument for join function");
                }
            }
            let joined = elements.join(&separator);
            return Node {
                token: args[0].token.clone(),
                value: Value::Text(joined),
                children: Vec::new(),
            };
        }
    }

    panic!("Invalid arguments for join function");
}

pub fn fn_split(args: &[Node], env: &mut Environment) -> Node {
    expect_n_args!(args, 2);

    if let Value::Text(separator) = evaluate_node(&args[0], env).value {
        if let Value::Text(string) = evaluate_node(&args[1], env).value {
            let parts = string.split(&separator).map(|s| Node {
                token: args[0].token.clone(),
                value: Value::Text(s.to_string()),
                children: Vec::new(),
            });

            return Node {
                token: args[0].token.clone(),
                value: Value::List(),
                children: parts.collect(),
            };
        }
    }

    panic!("Invalid arguments for split function");
}

pub fn fn_lines(args: &[Node], env: &mut Environment) -> Node {
    expect_n_args!(args, 1);

    if let Value::Text(string) = evaluate_node(&args[0], env).value {
        let lines = string.lines().map(|s| Node {
            token: args[0].token.clone(),
            value: Value::Text(s.to_string()),
            children: Vec::new(),
        });

        return Node {
            token: args[0].token.clone(),
            value: Value::List(),
            children: lines.collect(),
        };
    }

    panic!("Invalid argument for lines function");
}

pub fn fn_strlen(args: &[Node], env: &mut Environment) -> Node {
    expect_n_args!(args, 1);

    if let Value::Text(string) = evaluate_node(&args[0], env).value {
        return Node {
            token: args[0].token.clone(),
            value: Value::Number(string.len().try_into().expect("Invalid length")),
            children: Vec::new(),
        };
    }

    panic!("Invalid argument for strlen function");
}

pub fn fn_empty_string(args: &[Node], env: &mut Environment) -> Node {
    expect_n_args!(args, 1);

    if let Value::Text(string) = evaluate_node(&args[0], env).value {
        if string.is_empty() {
            return fn_true(&[]);
        }
        return fn_false(&[]);
    }

    panic!("Invalid argument for empty_string function");
}
