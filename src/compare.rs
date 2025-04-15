use crate::node::Node;
use crate::environment::Environment;

//- (test "=" (= 1 2) false)
//- (test "=" (= "foo" "bar") false)
//- (test "=" (= 1 1) true)
pub fn fn_eq(arguments: &[Node], _: &mut Environment) -> Result<Node, String> {
    if arguments.len() == 2 {
        return Ok(Node::Bool(arguments[0] == arguments[1]));
    }
    Err(format!("Invalid arguments for equality check: {:?}", &arguments[0]))
}

//- (test "<" (< 1 2) true)
//- (test "<" (< 2 1) false)
//- (test "<" (< 1 1) false)
pub fn fn_less_than(arguments: &[Node], _: &mut Environment) -> Result<Node, String> {
    if arguments.len() == 2 {
        return Ok(Node::Bool(arguments[0] < arguments[1]));
    }
    Err(format!("Invalid arguments for less than check: {:?}", &arguments[0]))
}

//- (test ">" (> 1 2) false)
//- (test ">" (> 2 1) true)
//- (test ">" (> 1 1) false)
pub fn fn_greater_than(arguments: &[Node], _: &mut Environment) -> Result<Node, String> {
    if arguments.len() == 2 {
        return Ok(Node::Bool(arguments[0] > arguments[1]));
    }
    Err(format!("Invalid arguments for greater than check: {:?}", &arguments[0]))
}
