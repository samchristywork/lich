use crate::Environment;
use crate::Node;
use crate::Value;
use crate::control::fn_false;
use crate::control::fn_true;
use crate::evaluate_args;
use crate::evaluate_node;
use crate::expect_n_args;

pub fn fn_add(args: &[Node], env: &mut Environment) -> Node {
    let mut n = 0;

    for arg in evaluate_args!(args, env) {
        if let Value::Number(value) = arg.value {
            n += value;
        } else {
            panic!("Invalid argument for addition function");
        }
    }

    Node {
        token: args[0].token.clone(),
        value: Value::Number(n),
        children: Vec::new(),
    }
}

pub fn fn_sub(args: &[Node], env: &mut Environment) -> Node {
    if let Value::Number(first_arg) = evaluate_node(&args[0], env).value {
        let mut n = first_arg;

        for arg in args.iter().skip(1) {
            if let Value::Number(value) = evaluate_node(arg, env).value {
                n -= value;
            } else {
                panic!("Invalid argument for subtraction function");
            }
        }

        return Node {
            token: args[0].token.clone(),
            value: Value::Number(n),
            children: Vec::new(),
        };
    }

    panic!("Invalid arguments for subtraction function");
}

pub fn fn_mul(args: &[Node], env: &mut Environment) -> Node {
    let mut n = 1;

    for arg in evaluate_args!(args, env) {
        if let Value::Number(value) = arg.value {
            n *= value;
        } else {
            panic!("Invalid argument for multiplication function");
        }
    }

    Node {
        token: args[0].token.clone(),
        value: Value::Number(n),
        children: Vec::new(),
    }
}

pub fn test_equal(a: &Node, b: &Node) -> bool {
    if a.value != b.value {
        return false;
    }

    if a.value == Value::LParen() {
        if a.children.len() != b.children.len() {
            return false;
        }
        for (a_child, b_child) in a.children.iter().zip(&b.children) {
            if !test_equal(a_child, b_child) {
                return false;
            }
        }
    }

    true
}

pub fn fn_equal(args: &[Node], env: &mut Environment) -> Node {
    expect_n_args!(args, 2);

    let a = evaluate_node(&args[0], env);
    let b = evaluate_node(&args[1], env);

    if test_equal(&a, &b) {
        fn_true(&[])
    } else {
        fn_false(&[])
    }
}

pub fn fn_less_than(args: &[Node], env: &mut Environment) -> Node {
    expect_n_args!(args, 2);

    if let Value::Number(a) = evaluate_node(&args[0], env).value {
        if let Value::Number(b) = evaluate_node(&args[1], env).value {
            if a < b {
                return fn_true(&[]);
            }
            return fn_false(&[]);
        }
    }

    panic!("Invalid arguments for less_than function");
}

pub fn fn_greater_than(args: &[Node], env: &mut Environment) -> Node {
    expect_n_args!(args, 2);

    if let Value::Number(a) = evaluate_node(&args[0], env).value {
        if let Value::Number(b) = evaluate_node(&args[1], env).value {
            if a > b {
                return fn_true(&[]);
            }
            return fn_false(&[]);
        }
    }

    panic!("Invalid arguments for greater_than function");
}

pub fn fn_pow(args: &[Node], env: &mut Environment) -> Node {
    expect_n_args!(args, 2);

    if let Value::Number(base) = evaluate_node(&args[0], env).value {
        if let Value::Number(exponent) = evaluate_node(&args[1], env).value {
            return Node {
                token: args[0].token.clone(),
                value: Value::Number(base.pow(exponent.try_into().expect("Invalid exponent"))),
                children: Vec::new(),
            };
        }
    }

    panic!("Invalid arguments for power function");
}

pub fn fn_mod(args: &[Node], env: &mut Environment) -> Node {
    expect_n_args!(args, 2);

    if let Value::Number(dividend) = evaluate_node(&args[0], env).value {
        if let Value::Number(divisor) = evaluate_node(&args[1], env).value {
            return Node {
                token: args[0].token.clone(),
                value: Value::Number(dividend % divisor),
                children: Vec::new(),
            };
        }
    }

    panic!("Invalid arguments for modulo function");
}

pub fn fn_inc(args: &[Node], env: &mut Environment) -> Node {
    expect_n_args!(args, 1);

    if let Value::Number(n) = evaluate_node(&args[0], env).value {
        return Node {
            token: args[0].token.clone(),
            value: Value::Number(n + 1),
            children: Vec::new(),
        };
    }

    panic!("Invalid arguments for increment function");
}

pub fn fn_dec(args: &[Node], env: &mut Environment) -> Node {
    expect_n_args!(args, 1);

    if let Value::Number(n) = evaluate_node(&args[0], env).value {
        return Node {
            token: args[0].token.clone(),
            value: Value::Number(n - 1),
            children: Vec::new(),
        };
    }

    panic!("Invalid arguments for decrement function");
}

pub fn fn_max(args: &[Node], env: &mut Environment) -> Node {
    assert!(!args.is_empty(), "No arguments provided for max function");

    let max_value = evaluate_args!(args, env)
        .iter()
        .map(|arg| {
            if let Value::Number(value) = arg.value {
                value
            } else {
                panic!("Invalid argument for max function");
            }
        })
        .max()
        .unwrap_or(0);

    Node {
        token: args[0].token.clone(),
        value: Value::Number(max_value),
        children: Vec::new(),
    }
}

pub fn fn_min(args: &[Node], env: &mut Environment) -> Node {
    assert!(!args.is_empty(), "No arguments provided for min function");

    let min_value = evaluate_args!(args, env)
        .iter()
        .map(|arg| {
            if let Value::Number(value) = arg.value {
                value
            } else {
                panic!("Invalid argument for min function");
            }
        })
        .min()
        .unwrap_or(0);

    Node {
        token: args[0].token.clone(),
        value: Value::Number(min_value),
        children: Vec::new(),
    }
}
