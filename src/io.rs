use crate::Environment;
use crate::Node;
use crate::Value;
use crate::control::fn_true;
use crate::evaluate_args;
use crate::evaluate_node;
use crate::expect_n_args;
use crate::intrinsic::fn_print_env;

pub fn fn_write(args: &[Node], env: &mut Environment) -> Node {
    println!(
        "{}",
        evaluate_args!(args, env)
            .iter()
            .map(super::Node::string)
            .collect::<Vec<_>>()
            .join(" ")
            .as_str()
    );

    fn_true(&[])
}

pub fn fn_debug_write(args: &[Node], env: &mut Environment) -> Node {
    println!("Debug Write:");
    fn_print_env(&[], env);
    println!("Args:");
    for arg in args {
        println!("  Arg: '{}'->'{}'", arg.token.value, arg.string());
    }
    println!(
        "{}",
        evaluate_args!(args, env)
            .iter()
            .map(super::Node::string)
            .collect::<Vec<_>>()
            .join(" ")
            .as_str()
    );

    fn_true(&[])
}

pub fn fn_write_stderr(args: &[Node], env: &mut Environment) -> Node {
    eprintln!(
        "{}",
        evaluate_args!(args, env)
            .iter()
            .map(super::Node::string)
            .collect::<Vec<_>>()
            .join(" ")
            .as_str()
    );

    fn_true(&[])
}

pub fn fn_write_file(args: &[Node], env: &mut Environment) -> Node {
    expect_n_args!(args, 2);

    if let Value::Text(filename) = evaluate_node(&args[0], env).value {
        if let Value::Text(content) = evaluate_node(&args[1], env).value {
            std::fs::write(filename, content).expect("Unable to write file");
            return fn_true(&[]);
        }
    }

    panic!("Invalid arguments for write_file function");
}

// TODO: Should we be trimming?
pub fn fn_read_line(args: &[Node]) -> Node {
    expect_n_args!(args, 0);

    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    Node {
        token: args[0].token.clone(),
        value: Value::Text(input.trim().to_string()),
        children: Vec::new(),
    }
}

pub fn fn_read_file(args: &[Node], env: &mut Environment) -> Node {
    expect_n_args!(args, 1);

    if let Value::Text(filename) = evaluate_node(&args[0], env).value {
        let content = std::fs::read_to_string(filename).expect("Unable to read file");

        Node {
            token: args[0].token.clone(),
            value: Value::Text(content),
            children: Vec::new(),
        }
    } else {
        panic!("Invalid argument for read_file function");
    }
}
