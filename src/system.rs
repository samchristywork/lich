use crate::node::Node;

pub fn fn_system(arguments: &[Node]) -> Result<Node, String> {
    if arguments.len() == 1 {
        let Node::Text(command) = &arguments[0] else {
            return Err("Invalid argument for system".to_string());
        };
        let output = std::process::Command::new("sh")
            .arg("-c")
            .arg(command)
            .output();
        if let Ok(output) = output {
            return Ok(Node::Text(
                String::from_utf8_lossy(&output.stdout).to_string(),
            ));
        }
    }

    Err("Invalid arguments for system".to_string())
}

pub fn fn_version(_: &[Node]) -> Result<Node, String> {
    Ok(Node::Text(format!(
        "Lich version {}",
        env!("CARGO_PKG_VERSION")
    )))
}

pub fn fn_exit(_: &[Node]) -> Result<Node, String> {
    std::process::exit(0);
}
