use crate::node::Node;
use crate::environment::Environment;
use crate::eval;
use crate::parse::parse;

pub fn fn_print_env(_: &[Node], env: &mut Environment) -> Node {
    println!("{env}");
    Node::Bool(true)
}

pub fn fn_load(arguments: &[Node], env: &mut Environment) -> Node {
    if arguments.len() == 1 {
        let Node::Text(filename) = &arguments[0] else {
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
}

pub fn fn_write(arguments: &[Node], _: &mut Environment) -> Node {
    for arg in arguments {
        print!("{arg}");
    }

    Node::Bool(true)
}

pub fn fn_write_line(arguments: &[Node], _: &mut Environment) -> Node {
    for arg in arguments {
        print!("{arg}");
    }
    println!();

    Node::Bool(true)
}
