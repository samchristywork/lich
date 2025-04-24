use crate::invalid_arguments;
use crate::node::Node;

//- (test "number->string" (number->string 1) "1")
//- (test "number->string" (number->string 10000) "10000")
//- (test "number->string" (number->string -1) "-1")
pub fn fn_number_to_string(arguments: &[Node]) -> Result<Node, String> {
    match arguments {
        [Node::Number(n)] => Ok(Node::Text(n.to_string())),
        _ => invalid_arguments!("number->string", arguments, ["[Number(n)]",]),
    }
}

//- (test "string->number" (string->number "1") 1)
//- (test "string->number" (string->number "10000") 10000)
//- (test "string->number" (string->number "-1") -1)
pub fn fn_string_to_number(arguments: &[Node]) -> Result<Node, String> {
    match arguments {
        [Node::Text(s)] => {
            let n = s.parse::<i64>().map_err(|_| "Invalid number".to_string())?;
            return Ok(Node::Number(n));
        }
        _ => invalid_arguments!("string->number", arguments, ["[Text(s)]"]),
    }
}

//- (test "string->list" (string->list "foo") (quote ("f" "o" "o")))
//- (test "string->list" (string->list "") (quote ()))
//- (test "string->list" (string->list "foo bar") (quote ("f" "o" "o" " " "b" "a" "r")))
pub fn fn_string_to_list(arguments: &[Node]) -> Result<Node, String> {
    match arguments {
        [Node::Text(s)] => {
            let list: Vec<Node> = s.chars().map(|c| Node::Text(c.to_string())).collect();
            return Ok(Node::List(list));
        }
        _ => invalid_arguments!("string->list", arguments, ["[Text(s)]"]),
    }
}

//- (test "list->string" (list->string (quote ("f" "o" "o"))) "foo")
//- (test "list->string" (list->string (quote ())) "")
//- (test "list->string" (list->string (quote ("f" "o" "o" " " "b" "a" "r"))) "foo bar")
pub fn fn_list_to_string(arguments: &[Node]) -> Result<Node, String> {
    match arguments {
        [Node::List(l)] => {
            let s: String = l.iter().map(std::string::ToString::to_string).collect();
            return Ok(Node::Text(s));
        }
        _ => invalid_arguments!("list->string", arguments, ["[List(l)]"]),
    }
}

//- (test "string->symbol" (string->symbol "foo") (quote foo))
pub fn fn_string_to_symbol(arguments: &[Node]) -> Result<Node, String> {
    match arguments {
        [Node::Text(s)] => {
            let symbol = s.chars().collect::<String>();
            return Ok(Node::Symbol(symbol));
        }
        _ => invalid_arguments!("string->symbol", arguments, ["[Text(s)]"]),
    }
}

//- (test "symbol->string" (symbol->string (quote foo)) "foo")
pub fn fn_symbol_to_string(arguments: &[Node]) -> Result<Node, String> {
    match arguments {
        [Node::Symbol(s)] => {
            let symbol = s.chars().collect::<String>();
            return Ok(Node::Text(symbol));
        }
        _ => invalid_arguments!("symbol->string", arguments, ["[Symbol(s)]"]),
    }
}

//- (test "boolean->string" (boolean->string true) "true")
//- (test "boolean->string" (boolean->string false) "false")
//- (test "boolean->string" (boolean->string false) "false")
pub fn fn_string_to_boolean(arguments: &[Node]) -> Result<Node, String> {
    match arguments {
        [Node::Text(s)] => {
            let b = s == "true";
            return Ok(Node::Bool(b));
        }
        _ => invalid_arguments!("string->boolean", arguments, ["[Text(s)]"]),
    }
}

//- (test "boolean->string" (boolean->string true) "true")
//- (test "boolean->string" (boolean->string false) "false")
//- (test "boolean->string" (boolean->string false) "false")
pub fn fn_boolean_to_string(arguments: &[Node]) -> Result<Node, String> {
    match arguments {
        [Node::Bool(b)] => {
            let s = b.to_string();
            return Ok(Node::Text(s));
        }
        _ => invalid_arguments!("boolean->string", arguments, ["[Bool(b)]"]),
    }
}

pub fn fn_time_to_string(arguments: &[Node]) -> Result<Node, String> {
    match arguments {
        [Node::Time(_, _)] => {
            return Ok(Node::Text(arguments[0].to_string()));
        }
        _ => invalid_arguments!("time->string", arguments, ["[Time(seconds, zone)]"]),
    }
}

pub fn fn_time_to_number(arguments: &[Node]) -> Result<Node, String> {
    match arguments {
        [Node::Time(seconds, _)] => {
            return Ok(Node::Number(*seconds));
        }
        _ => invalid_arguments!("time->number", arguments, ["[Time(seconds, zone)]"]),
    }
}

pub fn fn_number_to_float(arguments: &[Node]) -> Result<Node, String> {
    match arguments {
        [Node::Number(n)] => {
            let n = i32::try_from(*n)
                .map_err(|_| "Invalid number for conversion to float".to_string())?;
            let f = f64::from(n);
            return Ok(Node::Float(f));
        }
        _ => invalid_arguments!("number->float", arguments, ["[Number(n)]"]),
    }
}
