use crate::environment::Environment;
use crate::node::Node;

//- (test "number->string" (number->string 1) "1")
//- (test "number->string" (number->string 10000) "10000")
//- (test "number->string" (number->string -1) "-1")
pub fn fn_number_to_string(arguments: &[Node], _: &mut Environment) -> Result<Node, String> {
    if arguments.len() == 1 {
        if let Node::Number(n) = &arguments[0] {
            return Ok(Node::Text(n.to_string()));
        }
    }
    Err("Invalid arguments for number->string".to_string())
}

//- (test "string->number" (string->number "1") 1)
//- (test "string->number" (string->number "10000") 10000)
//- (test "string->number" (string->number "-1") -1)
pub fn fn_string_to_number(arguments: &[Node], _: &mut Environment) -> Result<Node, String> {
    if arguments.len() == 1 {
        if let Node::Text(s) = &arguments[0] {
            if let Ok(n) = s.parse::<i64>() {
                return Ok(Node::Number(n));
            }
        }
    }
    Err("Invalid arguments for string->number".to_string())
}

//- (test "string->list" (string->list "foo") (quote ("f" "o" "o")))
//- (test "string->list" (string->list "") (quote ()))
//- (test "string->list" (string->list "foo bar") (quote ("f" "o" "o" " " "b" "a" "r")))
pub fn fn_string_to_list(arguments: &[Node], _: &mut Environment) -> Result<Node, String> {
    if arguments.len() == 1 {
        if let Node::Text(s) = &arguments[0] {
            let list: Vec<Node> = s.chars().map(|c| Node::Text(c.to_string())).collect();
            return Ok(Node::List(list));
        }
    }
    Err("Invalid arguments for string->list".to_string())
}

//- (test "list->string" (list->string (quote ("f" "o" "o"))) "foo")
//- (test "list->string" (list->string (quote ())) "")
//- (test "list->string" (list->string (quote ("f" "o" "o" " " "b" "a" "r"))) "foo bar")
pub fn fn_list_to_string(arguments: &[Node], _: &mut Environment) -> Result<Node, String> {
    if arguments.len() == 1 {
        if let Node::List(l) = &arguments[0] {
            let s: String = l.iter().map(std::string::ToString::to_string).collect();
            return Ok(Node::Text(s));
        }
    }
    Err("Invalid arguments for list->string".to_string())
}

//- (test "string->symbol" (string->symbol "foo") (quote foo))
pub fn fn_string_to_symbol(arguments: &[Node], _: &mut Environment) -> Result<Node, String> {
    if arguments.len() == 1 {
        if let Node::Text(s) = &arguments[0] {
            return Ok(Node::Symbol(s.clone()));
        }
    }
    Err("Invalid arguments for string->symbol".to_string())
}

//- (test "symbol->string" (symbol->string (quote foo)) "foo")
pub fn fn_symbol_to_string(arguments: &[Node], _: &mut Environment) -> Result<Node, String> {
    if arguments.len() == 1 {
        if let Node::Symbol(s) = &arguments[0] {
            return Ok(Node::Text(s.clone()));
        }
    }
    Err("Invalid arguments for symbol->string".to_string())
}

//- (test "boolean->string" (boolean->string true) "true")
//- (test "boolean->string" (boolean->string false) "false")
//- (test "boolean->string" (boolean->string false) "false")
pub fn fn_string_to_boolean(arguments: &[Node], _: &mut Environment) -> Result<Node, String> {
    if arguments.len() == 1 {
        if let Node::Text(s) = &arguments[0] {
            return Ok(Node::Bool(s == "true"));
        }
    }
    Err("Invalid arguments for string->boolean".to_string())
}

//- (test "boolean->string" (boolean->string true) "true")
//- (test "boolean->string" (boolean->string false) "false")
//- (test "boolean->string" (boolean->string false) "false")
pub fn fn_boolean_to_string(arguments: &[Node], _: &mut Environment) -> Result<Node, String> {
    if arguments.len() == 1 {
        if let Node::Bool(b) = &arguments[0] {
            return Ok(Node::Text(b.to_string()));
        }
    }
    Err("Invalid arguments for boolean->string".to_string())
}

pub fn fn_time_to_string(arguments: &[Node], _: &mut Environment) -> Result<Node, String> {
    if arguments.len() == 1 {
        if let Node::Time(_, _) = &arguments[0] {
            return Ok(Node::Text(arguments[0].to_string()));
        }
    }
    Err("Invalid arguments for time->string".to_string())
}

pub fn fn_time_to_number(arguments: &[Node], _: &mut Environment) -> Result<Node, String> {
    if arguments.len() == 1 {
        if let Node::Time(seconds, _) = &arguments[0] {
            return Ok(Node::Number(*seconds));
        }
    }
    Err("Invalid arguments for time->number".to_string())
}
