use crate::invalid_arguments;
use crate::node::Node;

pub fn fn_write(arguments: &[Node]) -> Result<Node, String> {
    for arg in arguments {
        print!("{arg}");
    }

    Ok(Node::Bool(true))
}

pub fn fn_write_line(arguments: &[Node]) -> Result<Node, String> {
    for arg in arguments {
        print!("{arg}");
    }
    println!();

    Ok(Node::Bool(true))
}

pub fn fn_write_file(arguments: &[Node]) -> Result<Node, String> {
    match arguments {
        [Node::Text(filename), Node::Text(contents)] => {
            std::fs::write(filename, contents)
                .map_err(|_| format!("Failed to write file: {filename}"))?;
            Ok(Node::Bool(true))
        }
        _ => invalid_arguments!(
            "write-file",
            arguments,
            ["[Text(filename), Text(contents)]"]
        ),
    }
}

pub fn fn_read_line(_: &[Node]) -> Result<Node, String> {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .map_err(|e| format!("Failed to read-line: {e}"))?;
    Ok(Node::Text(input.trim().to_string()))
}

pub fn fn_read_file(arguments: &[Node]) -> Result<Node, String> {
    match arguments {
        [Node::Text(filename)] => {
            let input_string = std::fs::read_to_string(filename)
                .map_err(|_| format!("Failed to read file: {filename}"))?;
            Ok(Node::Text(input_string))
        }
        _ => invalid_arguments!("read-file", arguments, ["[Text(filename)]"]),
    }
}
