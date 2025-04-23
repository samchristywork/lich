use crate::node::Node;
use std::io::Write;

fn flush() -> Result<(), String> {
    if let Err(e) = std::io::stdout().flush() {
        return Err(format!("Failed to flush stdout: {e}"));
    }
    Ok(())
}

pub fn fn_clear(_: &[Node]) -> Result<Node, String> {
    print!(r"[2J[1;1H");
    flush()?;

    Ok(Node::Bool(true))
}

pub fn fn_alternate_screen(_: &[Node]) -> Result<Node, String> {
    print!(r"[?1049h");
    flush()?;

    std::thread::sleep(std::time::Duration::from_secs(1));
    println!("Displaying on Alternate Screen");
    std::thread::sleep(std::time::Duration::from_secs(1));

    Ok(Node::Bool(true))
}

pub fn fn_normal_screen(_: &[Node]) -> Result<Node, String> {
    print!(r"[?1049l");
    flush()?;

    Ok(Node::Bool(true))
}

pub fn fn_fg(arguments: &[Node]) -> Result<Node, String> {
    if arguments.len() == 1 {
        if let Node::Text(color) = &arguments[0] {
            print!(
                "{}",
                match color.as_str() {
                    "black" => "[30m",
                    "red" => "[31m",
                    "green" => "[32m",
                    "yellow" => "[33m",
                    "blue" => "[34m",
                    "magenta" => "[35m",
                    "cyan" => "[36m",
                    "white" => "[37m",
                    _ => return Err(format!("Invalid color: {color}")),
                }
            );
        }
    } else if arguments.is_empty() {
        print!("[0m");
    } else {
        return Err("Invalid arguments for fg".to_string());
    }

    Ok(Node::Bool(true))
}

pub fn fn_bg(arguments: &[Node]) -> Result<Node, String> {
    if arguments.len() == 1 {
        if let Node::Text(color) = &arguments[0] {
            print!(
                "{}",
                match color.as_str() {
                    "black" => "[40m",
                    "red" => "[41m",
                    "green" => "[42m",
                    "yellow" => "[43m",
                    "blue" => "[44m",
                    "magenta" => "[45m",
                    "cyan" => "[46m",
                    "white" => "[47m",
                    _ => return Err(format!("Invalid color: {color}")),
                }
            );
        }
    } else if arguments.is_empty() {
        print!("[0m");
    } else {
        return Err("Invalid arguments for bg".to_string());
    }

    Ok(Node::Bool(true))
}

pub fn fn_set_cursor_pos(arguments: &[Node]) -> Result<Node, String> {
    if arguments.len() == 2 {
        if let (Node::Number(x), Node::Number(y)) = (&arguments[0], &arguments[1]) {
            print!("[{y};{x}H");
        } else {
            return Err("Invalid arguments for set_cursor_pos".to_string());
        }
    } else {
        return Err("Invalid arguments for set_cursor_pos".to_string());
    }

    Ok(Node::Bool(true))
}
