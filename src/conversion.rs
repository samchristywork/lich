use crate::node::Node;
use crate::environment::Environment;

//- (test "number->string" (number->string 1) "1")
//- (test "number->string" (number->string 10000) "10000")
//- (test "number->string" (number->string -1) "-1")
pub fn fn_number_to_string(arguments: &[Node], _: &mut Environment) -> Node {
    if arguments.len() == 1 {
        if let Node::Number(n) = &arguments[0] {
            return Node::Text(n.to_string());
        }
    }
    panic!("Invalid arguments for number->string");
}

//- (test "string->number" (string->number "1") 1)
//- (test "string->number" (string->number "10000") 10000)
//- (test "string->number" (string->number "-1") -1)
pub fn fn_string_to_number(arguments: &[Node], _: &mut Environment) -> Node {
    if arguments.len() == 1 {
        if let Node::Text(s) = &arguments[0] {
            if let Ok(n) = s.parse::<i64>() {
                return Node::Number(n);
            }
        }
    }
    panic!("Invalid arguments for string->number");
}

//- (test "string->list" (string->list "foo") (quote ("f" "o" "o")))
//- (test "string->list" (string->list "") (quote ()))
//- (test "string->list" (string->list "foo bar") (quote ("f" "o" "o" " " "b" "a" "r")))
pub fn fn_string_to_list(arguments: &[Node], _: &mut Environment) -> Node {
    if arguments.len() == 1 {
        if let Node::Text(s) = &arguments[0] {
            let list: Vec<Node> = s.chars().map(|c| Node::Text(c.to_string())).collect();
            return Node::List(list);
        }
    }
    panic!("Invalid arguments for string->list");
}

//- (test "list->string" (list->string (quote ("f" "o" "o"))) "foo")
//- (test "list->string" (list->string (quote ())) "")
//- (test "list->string" (list->string (quote ("f" "o" "o" " " "b" "a" "r"))) "foo bar")
pub fn fn_list_to_string(arguments: &[Node], _: &mut Environment) -> Node {
    if arguments.len() == 1 {
        if let Node::List(l) = &arguments[0] {
            let s: String = l.iter().map(|n| n.to_string()).collect();
            return Node::Text(s);
        }
    }
    panic!("Invalid arguments for list->string");
}

//- (test "string->symbol" (string->symbol "foo") (quote foo))
pub fn fn_string_to_symbol(arguments: &[Node], _: &mut Environment) -> Node {
    if arguments.len() == 1 {
        if let Node::Text(s) = &arguments[0] {
            return Node::Symbol(s.clone());
        }
    }
    panic!("Invalid arguments for string->symbol");
}

//- (test "symbol->string" (symbol->string (quote foo)) "foo")
pub fn fn_symbol_to_string(arguments: &[Node], _: &mut Environment) -> Node {
    if arguments.len() == 1 {
        if let Node::Symbol(s) = &arguments[0] {
            return Node::Text(s.clone());
        }
    }
    panic!("Invalid arguments for symbol->string");
}

//- (test "boolean->string" (boolean->string true) "true")
//- (test "boolean->string" (boolean->string false) "false")
//- (test "boolean->string" (boolean->string false) "false")
pub fn fn_string_to_boolean(arguments: &[Node], _: &mut Environment) -> Node {
    if arguments.len() == 1 {
        if let Node::Text(s) = &arguments[0] {
            return Node::Bool(s == "true");
        }
    }
    panic!("Invalid arguments for string->boolean");
}

//- (test "boolean->string" (boolean->string true) "true")
//- (test "boolean->string" (boolean->string false) "false")
//- (test "boolean->string" (boolean->string false) "false")
pub fn fn_boolean_to_string(arguments: &[Node], _: &mut Environment) -> Node {
    if arguments.len() == 1 {
        if let Node::Bool(b) = &arguments[0] {
            return Node::Text(b.to_string());
        }
    }
    panic!("Invalid arguments for bool->string");
}
