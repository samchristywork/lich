use crate::Environment;
use crate::Node;
use crate::Value;
use crate::control::fn_false;
use crate::control::fn_true;
use crate::evaluate_node;
use crate::expect_n_args;

pub fn fn_is_text(args: &[Node], env: &mut Environment) -> Node {
    expect_n_args!(args, 1);
    if let Value::Text(_) = evaluate_node(&args[0], env).value {
        fn_true(&[])
    } else {
        fn_false(&[])
    }
}

pub fn fn_is_number(args: &[Node], env: &mut Environment) -> Node {
    expect_n_args!(args, 1);
    if let Value::Number(_) = evaluate_node(&args[0], env).value {
        fn_true(&[])
    } else {
        fn_false(&[])
    }
}

pub fn fn_is_symbol(args: &[Node], env: &mut Environment) -> Node {
    expect_n_args!(args, 1);
    if let Value::Symbol(_) = evaluate_node(&args[0], env).value {
        fn_true(&[])
    } else {
        fn_false(&[])
    }
}

pub fn fn_is_lparen(args: &[Node], env: &mut Environment) -> Node {
    expect_n_args!(args, 1);
    if evaluate_node(&args[0], env).value == Value::LParen() {
        fn_true(&[])
    } else {
        fn_false(&[])
    }
}

pub fn fn_is_lambda(args: &[Node], env: &mut Environment) -> Node {
    expect_n_args!(args, 1);
    if evaluate_node(&args[0], env).value == Value::Lambda() {
        fn_true(&[])
    } else {
        fn_false(&[])
    }
}

pub fn fn_is_atom(args: &[Node], env: &mut Environment) -> Node {
    expect_n_args!(args, 1);
    if let Value::Atom(_) = evaluate_node(&args[0], env).value {
        fn_true(&[])
    } else {
        fn_false(&[])
    }
}
