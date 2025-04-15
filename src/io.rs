use crate::node::Node;
use crate::environment::Environment;
use crate::eval;
use crate::parse::parse;

pub fn fn_print_env(_: &[Node], env: &mut Environment) -> Result<Node, String> {
    println!("{env}");
    Ok(Node::Bool(true))
}

pub fn fn_load(arguments: &[Node], env: &mut Environment) -> Result<Node, String> {
    if arguments.len() == 1 {
        let Node::Text(filename) = &arguments[0] else {
            return Err("Invalid argument for load".to_string());
        };
        let input_string = std::fs::read_to_string(filename)
            .map_err(|_| format!("Failed to read file: {filename}"))?;
        let expressions = parse(&input_string)?;
        for expression in expressions {
            eval(&expression, env)?;
        }
        return Ok(Node::Bool(true));
    }
    Err("Invalid arguments for load".to_string())
}

pub fn fn_write(arguments: &[Node], _: &mut Environment) -> Result<Node, String> {
    for arg in arguments {
        print!("{arg}");
    }

    Ok(Node::Bool(true))
}

pub fn fn_write_line(arguments: &[Node], _: &mut Environment) -> Result<Node, String> {
    for arg in arguments {
        print!("{arg}");
    }
    println!();

    Ok(Node::Bool(true))
}
