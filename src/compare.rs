use crate::environment::Environment;
use crate::node::Node;

//- (test "=" (= 1 2) false)
//- (test "=" (= "foo" "bar") false)
//- (test "=" (= 1 1) true)
pub fn fn_eq(arguments: &[Node], _: &mut Environment) -> Result<Node, String> {
    if arguments.len() == 2 {
        return Ok(Node::Bool(arguments[0] == arguments[1]));
    }
    Err(format!(
        "Invalid arguments for equality check: {:?}",
        &arguments[0]
    ))
}

//- (test "<" (< 1 2) true)
//- (test "<" (< 2 1) false)
//- (test "<" (< 1 1) false)
pub fn fn_less_than(arguments: &[Node], _: &mut Environment) -> Result<Node, String> {
    if arguments.len() == 2 {
        return Ok(Node::Bool(arguments[0] < arguments[1]));
    }
    Err(format!(
        "Invalid arguments for less than check: {:?}",
        &arguments[0]
    ))
}

//- (test ">" (> 1 2) false)
//- (test ">" (> 2 1) true)
//- (test ">" (> 1 1) false)
pub fn fn_greater_than(arguments: &[Node], _: &mut Environment) -> Result<Node, String> {
    if arguments.len() == 2 {
        return Ok(Node::Bool(arguments[0] > arguments[1]));
    }
    Err(format!(
        "Invalid arguments for greater than check: {:?}",
        &arguments[0]
    ))
}

//- (test "<=" (<= 1 2) true)
//- (test "<=" (<= 2 1) false)
//- (test "<=" (<= 1 1) true)
pub fn fn_less_than_or_equal(arguments: &[Node], _: &mut Environment) -> Result<Node, String> {
    if arguments.len() == 2 {
        return Ok(Node::Bool(arguments[0] <= arguments[1]));
    }
    Err(format!(
        "Invalid arguments for less than or equal check: {:?}",
        &arguments[0]
    ))
}

//- (test ">=" (>= 1 2) false)
//- (test ">=" (>= 2 1) true)
//- (test ">=" (>= 1 1) true)
pub fn fn_greater_than_or_equal(arguments: &[Node], _: &mut Environment) -> Result<Node, String> {
    if arguments.len() == 2 {
        return Ok(Node::Bool(arguments[0] >= arguments[1]));
    }
    Err(format!(
        "Invalid arguments for greater than or equal check: {:?}",
        &arguments[0]
    ))
}

//- (test "not" (not true) false)
//- (test "not" (not false) true)
pub fn fn_not(arguments: &[Node], _: &mut Environment) -> Result<Node, String> {
    if arguments.len() == 1 {
        if let Node::Bool(b) = &arguments[0] {
            return Ok(Node::Bool(!b));
        }
    }
    Err(format!("Invalid arguments for not: {:?}", &arguments[0]))
}

//- (test "and" (and true true) true)
//- (test "and" (and true false) false)
//- (test "and" (and false true) false)
pub fn fn_and(arguments: &[Node], _: &mut Environment) -> Result<Node, String> {
    if arguments.len() == 2 {
        if let Node::Bool(a) = &arguments[0] {
            if let Node::Bool(b) = &arguments[1] {
                return Ok(Node::Bool(*a && *b));
            }
        }
    }
    Err(format!("Invalid arguments for and: {:?}", &arguments[0]))
}

//- (test "or" (or true true) true)
//- (test "or" (or true false) true)
//- (test "or" (or false false) false)
pub fn fn_or(arguments: &[Node], _: &mut Environment) -> Result<Node, String> {
    if arguments.len() == 2 {
        if let Node::Bool(a) = &arguments[0] {
            if let Node::Bool(b) = &arguments[1] {
                return Ok(Node::Bool(*a || *b));
            }
        }
    }
    Err(format!("Invalid arguments for or: {:?}", &arguments[0]))
}
