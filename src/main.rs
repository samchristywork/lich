pub mod list;
pub mod conversion;
pub mod io;
pub mod arithmetic;
pub mod compare;
pub mod environment;
pub mod node;
pub mod util;
pub mod string;
pub mod sequence;
pub mod system;
pub mod parse;

use std::io::Write;
use crate::node::Node;
use crate::environment::Environment;
use crate::parse::parse;

const GREY: &str = "\x1b[90m";
const NORMAL: &str = "\x1b[0m";

fn eval(node: &Node, env: &mut Environment) -> Node {
    match node {
        Node::Symbol(_) => env.lookup(node).expect(format!("Undefined variable: {node:?}").as_str()),
        Node::Number(_) | Node::Text(_) | Node::Bool(_) | Node::Function(_) => node.clone(),
        Node::List(nodes) => eval_list(nodes, env),
    }
}

fn eval_if(rest: &[Node], env: &mut Environment) -> Node {
    let condition = eval(&rest[0], env);
    match condition {
        Node::Bool(true) => eval(&rest[1], env),
        Node::Bool(false) => eval(&rest[2], env),
        _ => panic!("Condition must be a boolean"),
    }
}

fn eval_cond(rest: &[Node], env: &mut Environment) -> Node {
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
}

fn eval_define(rest: &[Node], env: &mut Environment) -> Node {
    let variable = &rest[0];
    let value = eval(&rest[1], env);
    env.variables.insert(variable.clone(), value.clone());

    value
}

fn eval_lambda(rest: &[Node]) -> Node {
    assert!(rest.len() == 2, "Invalid arguments for lambda");
    let parameters = rest[0].clone();
    let body = rest[1].clone();

    Node::List(vec![Node::Symbol("lambda".to_string()), parameters, body])
}

fn eval_let(rest: &[Node], env: &mut Environment) -> Node {
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
}

fn eval_let_restricted(rest: &[Node], env: &mut Environment) -> Node {
    if rest.len() == 2 {
        let bindings = &rest[0];
        let body = &rest[1];

        if let Node::List(bindings_list) = bindings {
            let mut new_env = Environment {
                parent: None,
                variables: std::collections::HashMap::new(),
            };

            for binding in bindings_list {
                if let Node::List(binding_pair) = binding {
                    if binding_pair.len() == 1 {
                        let variable = &binding_pair[0];
                        let value = eval(&binding_pair[0], env);
                        new_env.variables.insert(variable.clone(), value);
                    }else if binding_pair.len() == 2 {
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

            match operator {
                "quote" => rest[0].clone(),
                "if" => eval_if(rest, env),
                "cond" => eval_cond(rest, env),
                "define" => eval_define(rest, env),
                "lambda" => eval_lambda(rest),
                "let" => eval_let(rest, env),
                "let-restricted" => eval_let_restricted(rest, env),
                _ => {
                    let function = env.lookup(first).expect(format!("Undefined function: {first:?}").as_str());
                    let arguments = rest.iter().map(|n| eval(n, env)).collect::<Vec<_>>();
                    apply(&function, &arguments, env)
                }
            }
        }
        _ => panic!("Unknown operator {first:?}"),
    }
}


fn apply(function: &Node, arguments: &[Node], env: &mut Environment) -> Node {
    match function {
        Node::Function(f) => f(arguments, env),
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
        _ => panic!("Function application not implemented"),
    }
}

fn main() {
    let mut env = Environment {
        parent: None,
        variables: std::collections::HashMap::new(),
    };


    // Arithmetic
    env.add_function("+", arithmetic::fn_add);
    env.add_function("-", arithmetic::fn_sub);
    env.add_function("*", arithmetic::fn_mult);
    env.add_function("even?", arithmetic::fn_is_even);

    // Comparison
    env.add_function("=", compare::fn_eq);
    env.add_function("<", compare::fn_less_than);
    env.add_function(">", compare::fn_greater_than);

    // List Manipulation
    env.add_function("car", list::fn_car);
    env.add_function("cdr", list::fn_cdr);
    env.add_function("cons", list::fn_cons);
    env.add_function("length", list::fn_length);
    env.add_function("null?", list::fn_is_null);
    env.add_function("list", list::fn_list);
    env.add_function("last", list::fn_last);

    // Sequence Manipulation
    env.add_function("fold", sequence::fn_fold);
    env.add_function("zip", sequence::fn_zip);
    env.add_function("range", sequence::fn_range);
    env.add_function("for-each", sequence::fn_for_each);
    env.add_function("map", sequence::fn_map);
    env.add_function("filter", sequence::fn_filter);

    // String Manipulation
    env.add_function("concat", string::fn_concat);
    env.add_function("split", string::fn_split);
    env.add_function("strip", string::fn_strip);

    // Conversion
    env.add_function("number->string", conversion::fn_number_to_string);
    env.add_function("string->number", conversion::fn_string_to_number);
    env.add_function("string->list", conversion::fn_string_to_list);
    env.add_function("list->string", conversion::fn_list_to_string);
    env.add_function("string->symbol", conversion::fn_string_to_symbol);
    env.add_function("symbol->string", conversion::fn_symbol_to_string);
    env.add_function("string->boolean", conversion::fn_string_to_boolean);
    env.add_function("boolean->string", conversion::fn_boolean_to_string);

    // I/O
    env.add_function("print-env", io::fn_print_env);
    env.add_function("load", io::fn_load);
    env.add_function("write", io::fn_write);
    env.add_function("write-line", io::fn_write_line);

    // Utility
    env.add_function("begin", util::fn_begin);
    env.add_function("type?", util::fn_type);
    env.add_function("time-ms", util::fn_time_ms);
    env.add_function("among", util::fn_among);

    // System
    env.add_function("system", system::fn_system);
    env.add_function("version", system::fn_version);
    env.add_function("exit", system::fn_exit);

    let args = std::env::args().collect::<Vec<_>>();
    let flag_args = args
        .iter()
        .filter(|arg| arg.starts_with('-'))
        .collect::<Vec<_>>();
    let positional_args = args
        .iter()
        .filter(|arg| !arg.starts_with('-'))
        .collect::<Vec<_>>();

    let verbose = flag_args
        .iter()
        .any(|arg| arg == &"-v" || arg == &"--verbose");

    if positional_args.len() > 1 {
        positional_args
            .iter()
            .skip(1)
            .for_each(|arg| {
                let input_string = std::fs::read_to_string(arg)
                    .expect("Failed to read input file");
                let expressions = parse(&input_string);

                for expression in expressions {
                    if verbose {
                        eprintln!("{GREY}{expression}{NORMAL}");
                        eprintln!("Result: {:?}", eval(&expression, &mut env));
                    } else {
                        eval(&expression, &mut env);
                    }
                }
            });
    } else {
        loop {
            print!("lich> ");
            std::io::stdout().flush().expect("Failed to flush stdout");

            let mut input = String::new();
            std::io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            let input_string = input.trim().to_string();
            if input_string == "exit" {
                break;
            }

            let expressions = parse(&input_string);
            for expression in expressions {
                let result = eval(&expression, &mut env);
                eprintln!("{GREY}{result:?}{NORMAL}");
            }
        }
    }
}
