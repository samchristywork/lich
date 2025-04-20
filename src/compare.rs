use crate::invalid_arguments;
use crate::node::Node;

//- (test "=" (= 1 2) false)
//- (test "=" (= "foo" "bar") false)
//- (test "=" (= 1 1) true)
pub fn fn_eq(arguments: &[Node]) -> Result<Node, String> {
    match arguments {
        [Node::Number(a), Node::Number(b)] => Ok(Node::Bool(a == b)),
        [Node::Text(a), Node::Text(b)] => Ok(Node::Bool(a == b)),
        [Node::List(a), Node::List(b)] => Ok(Node::Bool(a == b)),
        [Node::Bool(a), Node::Bool(b)] => Ok(Node::Bool(a == b)),
        [Node::Symbol(a), Node::Symbol(b)] => Ok(Node::Bool(a == b)),
        [Node::Float(a), Node::Float(b)] => Ok(Node::Bool(a == b)),
        [Node::Time(t1, z1), Node::Time(t2, z2)] => Ok(Node::Bool(t1 == t2 && z1 == z2)),
        _ => invalid_arguments!(
            "=",
            arguments,
            [
                "[Number(a), Number(b)]",
                "[Text(a), Text(b)]",
                "[List(a), List(b)]",
                "[Bool(a), Bool(b)]",
                "[Symbol(a), Symbol(b)]",
                "[Time(t1, z1), Time(t2, z2)]"
            ]
        ),
    }
}

//- (test "<" (< 1 2) true)
//- (test "<" (< 2 1) false)
//- (test "<" (< 1 1) false)
pub fn fn_less_than(arguments: &[Node]) -> Result<Node, String> {
    match arguments {
        [Node::Number(a), Node::Number(b)] => Ok(Node::Bool(a < b)),
        _ => invalid_arguments!("<", arguments, ["[Number(a), Number(b)]"]),
    }
}

//- (test ">" (> 1 2) false)
//- (test ">" (> 2 1) true)
//- (test ">" (> 1 1) false)
pub fn fn_greater_than(arguments: &[Node]) -> Result<Node, String> {
    match arguments {
        [Node::Number(a), Node::Number(b)] => Ok(Node::Bool(a > b)),
        _ => invalid_arguments!(">", arguments, ["[Number(a), Number(b)]"]),
    }
}

//- (test "<=" (<= 1 2) true)
//- (test "<=" (<= 2 1) false)
//- (test "<=" (<= 1 1) true)
pub fn fn_less_than_or_equal(arguments: &[Node]) -> Result<Node, String> {
    match arguments {
        [Node::Number(a), Node::Number(b)] => Ok(Node::Bool(a <= b)),
        _ => invalid_arguments!("<=", arguments, ["[Number(a), Number(b)]"]),
    }
}

//- (test ">=" (>= 1 2) false)
//- (test ">=" (>= 2 1) true)
//- (test ">=" (>= 1 1) true)
pub fn fn_greater_than_or_equal(arguments: &[Node]) -> Result<Node, String> {
    match arguments {
        [Node::Number(a), Node::Number(b)] => Ok(Node::Bool(a >= b)),
        _ => invalid_arguments!(">=", arguments, ["[Number(a), Number(b)]"]),
    }
}

//- (test "not" (not true) false)
//- (test "not" (not false) true)
pub fn fn_not(arguments: &[Node]) -> Result<Node, String> {
    match arguments {
        [Node::Bool(a)] => Ok(Node::Bool(!a)),
        _ => invalid_arguments!("not", arguments, ["[Bool(a)]"]),
    }
}

//- (test "and" (and true true) true)
//- (test "and" (and true false) false)
//- (test "and" (and false true) false)
pub fn fn_and(arguments: &[Node]) -> Result<Node, String> {
    match arguments {
        [Node::Bool(a), Node::Bool(b)] => Ok(Node::Bool(*a && *b)),
        _ => invalid_arguments!("and", arguments, ["[Bool(a), Bool(b)]"]),
    }
}

//- (test "or" (or true true) true)
//- (test "or" (or true false) true)
//- (test "or" (or false false) false)
pub fn fn_or(arguments: &[Node]) -> Result<Node, String> {
    match arguments {
        [Node::Bool(a), Node::Bool(b)] => Ok(Node::Bool(*a || *b)),
        _ => invalid_arguments!("or", arguments, ["[Bool(a), Bool(b)]"]),
    }
}
