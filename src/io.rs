use crate::invalid_arguments;
use crate::node::Node;

pub fn fn_format(arguments: &[Node]) -> Result<Node, String> {
    Ok(Node::Text(
        arguments
            .iter()
            .map(|arg| arg.to_string())
            .collect::<Vec<String>>()
            .join(""),
    ))
}

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

pub fn fn_ls(arguments: &[Node]) -> Result<Node, String> {
    match arguments {
        [Node::Text(path)] => {
            let entries = std::fs::read_dir(path)
                .map_err(|_| format!("Failed to read directory: {path}"))?
                .filter_map(|entry| entry.ok())
                .map(|entry| Node::Text(entry.file_name().to_string_lossy().to_string()))
                .collect::<Vec<Node>>();
            Ok(Node::List(entries))
        }
        _ => invalid_arguments!("ls", arguments, ["[Text(path)]"]),
    }
}

pub fn fn_is_directory(arguments: &[Node]) -> Result<Node, String> {
    match arguments {
        [Node::Text(path)] => {
            let is_dir = std::fs::metadata(path)
                .map(|meta| meta.is_dir())
                .unwrap_or(false);
            Ok(Node::Bool(is_dir))
        }
        _ => invalid_arguments!("is-directory", arguments, ["[Text(path)]"]),
    }
}
