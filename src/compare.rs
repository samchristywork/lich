use crate::node::Node;
use crate::environment::Environment;

//- (test "=" (= 1 2) false)
//- (test "=" (= "foo" "bar") false)
//- (test "=" (= 1 1) true)
pub fn fn_eq(arguments: &[Node], _: &mut Environment) -> Node {
    if arguments.len() == 2 {
        return Node::Bool(arguments[0] == arguments[1]);
    }
    panic!("Invalid arguments for equality check");
}

//- (test "<" (< 1 2) true)
//- (test "<" (< 2 1) false)
//- (test "<" (< 1 1) false)
pub fn fn_less_than(arguments: &[Node], _: &mut Environment) -> Node {
    if arguments.len() == 2 {
        return Node::Bool(arguments[0] < arguments[1]);
    }
    panic!("Invalid arguments for less than check");
}

//- (test ">" (> 1 2) false)
//- (test ">" (> 2 1) true)
//- (test ">" (> 1 1) false)
pub fn fn_greater_than(arguments: &[Node], _: &mut Environment) -> Node {
    if arguments.len() == 2 {
        return Node::Bool(arguments[0] > arguments[1]);
    }
    panic!("Invalid arguments for greater than check");
}
