use crate::node::Node;
use crate::environment::Environment;

pub fn fn_system(arguments: &[Node], _: &mut Environment) -> Node {
    if arguments.len() == 1 {
        let Node::Text(command) = &arguments[0] else {
            panic!("Invalid argument for system");
        };
        let output = std::process::Command::new("sh")
            .arg("-c")
            .arg(command)
            .output()
            .expect("Failed to execute command");
        return Node::Text(String::from_utf8_lossy(&output.stdout).to_string());
    }
    panic!("Invalid arguments for system");
}

pub fn fn_version(_: &[Node], _: &mut Environment) -> Node {
    Node::Text(format!("Lich version {}", env!("CARGO_PKG_VERSION")))
}

pub fn fn_exit(_: &[Node], _: &mut Environment) -> Node {
    std::process::exit(0);
}
