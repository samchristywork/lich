use crate::Environment;
use crate::Node;
use crate::Value;
use crate::control::fn_false;
use crate::control::fn_true;
use crate::evaluate_node;

pub fn fn_and(args: &[Node], env: &mut Environment) -> Node {
    if args.iter().all(|arg| {
        if let Value::Symbol(ref s) = evaluate_node(arg, env).value {
            s == "true"
        } else {
            panic!("Invalid argument for and function");
        }
    }) {
        fn_true(&[])
    } else {
        fn_false(&[])
    }
}

pub fn fn_or(args: &[Node], env: &mut Environment) -> Node {
    if args.iter().any(|arg| {
        if let Value::Symbol(ref s) = evaluate_node(arg, env).value {
            s == "true"
        } else {
            panic!("Invalid argument for or function");
        }
    }) {
        fn_true(&[])
    } else {
        fn_false(&[])
    }
}
