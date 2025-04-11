use crate::Environment;
use crate::Node;
use crate::Range;
use crate::Token;
use crate::TokenKind;
use crate::Value;
use crate::evaluate_node;
use crate::expect_n_args;

pub fn fn_true(_: &[Node]) -> Node {
    Node {
        token: Token {
            value: "true".to_string(),
            kind: TokenKind::Symbol,
            range: Range { start: 0, end: 0 },
        },
        value: Value::Symbol("true".to_string()),
        children: Vec::new(),
    }
}

pub fn fn_false(_: &[Node]) -> Node {
    Node {
        token: Token {
            value: "false".to_string(),
            kind: TokenKind::Symbol,
            range: Range { start: 0, end: 0 },
        },
        value: Value::Symbol("false".to_string()),
        children: Vec::new(),
    }
}

pub fn fn_if(args: &[Node], env: &mut Environment) -> Node {
    expect_n_args!(args, 3);

    if let Value::Symbol(ref s) = evaluate_node(&args[0], env).value {
        if s == "true" {
            return evaluate_node(&args[1], env);
        }
        return evaluate_node(&args[2], env);
    }

    panic!("Invalid argument for if function");
}

pub fn fn_cond(args: &[Node], env: &mut Environment) -> Node {
    args.iter()
        .find_map(|arg| {
            if let Value::Symbol(ref s) = evaluate_node(&arg.children[0], env).value {
                if s == "true" {
                    Some(evaluate_node(&arg.children[1], env))
                } else {
                    None
                }
            } else {
                panic!("Invalid argument for cond function");
            }
        })
        .unwrap_or_else(|| fn_false(&[]))
}

pub fn fn_repeat(args: &[Node], env: &mut Environment) -> Node {
    expect_n_args!(args, 2);

    if let Value::Number(n) = evaluate_node(&args[0], env).value {
        (0..n).fold(fn_false(&[]), |_, _| evaluate_node(&args[1], env))
    } else {
        panic!("Invalid argument for repeat function");
    }
}

pub fn fn_loop(args: &[Node], env: &mut Environment) -> Node {
    expect_n_args!(args, 1);

    loop {
        evaluate_node(&args[0], env);
    }
}

pub fn fn_pipeline(args: &[Node], env: &mut Environment) -> Node {
    let mut root = args[args.len() - 1].clone();

    args.iter()
        .rev()
        .skip(1)
        .fold(&mut root, |current, next_node| {
            current.children.push(next_node.clone());
            current.children.last_mut().expect("No last element")
        });

    evaluate_node(&root, env)
}

pub fn fn_reverse_pipeline(args: &[Node], env: &mut Environment) -> Node {
    let mut root = args[0].clone();

    args.iter().skip(1).fold(&mut root, |current, next_node| {
        current.children.push(next_node.clone());
        current.children.last_mut().expect("No last element")
    });

    evaluate_node(&root, env)
}

pub fn fn_block(args: &[Node], env: &mut Environment) -> Node {
    for arg in args {
        evaluate_node(arg, env);
    }

    fn_true(&[])
}

pub fn fn_exit(args: &[Node], env: &mut Environment) -> Node {
    expect_n_args!(args, 1);

    if let Value::Number(exit_code) = evaluate_node(&args[0], env).value {
        std::process::exit(exit_code.try_into().expect("Invalid exit code"));
    }

    panic!("Invalid argument for exit function");
}
