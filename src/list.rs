use crate::Environment;
use crate::Node;
use crate::Value;
use crate::evaluate_args;
use crate::evaluate_node;
use crate::expect_n_args;

pub fn fn_list(args: &[Node], env: &mut Environment) -> Node {
    let list = evaluate_args!(args, env);

    Node {
        token: args[0].token.clone(),
        value: Value::LParen(),
        children: list,
    }
}

pub fn fn_head(args: &[Node], env: &mut Environment) -> Node {
    expect_n_args!(args, 1);

    let n = evaluate_node(&args[0], env);
    if n.value == Value::LParen() {
        if args[0].children.is_empty() {
            panic!("Empty list");
        } else {
            return n.children[0].clone();
        }
    }

    panic!("Invalid argument for head function");
}

pub fn fn_last(args: &[Node], env: &mut Environment) -> Node {
    expect_n_args!(args, 1);

    let n = evaluate_node(&args[0], env);
    if n.value == Value::LParen() {
        if args[0].children.is_empty() {
            panic!("Empty list");
        } else {
            return n.children.last().expect("No last element").clone();
        }
    }

    panic!("Invalid argument for last function");
}

pub fn fn_tail(args: &[Node], env: &mut Environment) -> Node {
    expect_n_args!(args, 1);

    let n = evaluate_node(&args[0], env);
    if n.value == Value::LParen() {
        if args[0].children.is_empty() {
            panic!("Empty list");
        } else {
            return Node {
                token: args[0].token.clone(),
                value: Value::LParen(),
                children: n.children[1..].to_vec(),
            };
        }
    }

    panic!("Invalid argument for tail function");
}

pub fn fn_length(args: &[Node], env: &mut Environment) -> Node {
    expect_n_args!(args, 1);

    let n = evaluate_node(&args[0], env);
    if n.value == Value::LParen() {
        return Node {
            token: args[0].token.clone(),
            value: Value::Number(n.children.len().try_into().expect("Invalid length")),
            children: Vec::new(),
        };
    }

    panic!("Invalid argument for length function");
}

pub fn fn_reverse(args: &[Node], env: &mut Environment) -> Node {
    expect_n_args!(args, 1);

    let n = evaluate_node(&args[0], env);
    if n.value == Value::LParen() {
        return Node {
            token: args[0].token.clone(),
            value: Value::LParen(),
            children: n.children.iter().rev().cloned().collect(),
        };
    }

    panic!("Invalid argument for reverse function");
}
