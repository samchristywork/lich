use crate::node::Node;
use crate::environment::Environment;

//- (test "+" (+ 1 2 3) 6)
//- (test "+" (+ -10 2 3) -5)
//- (test "+" (+ 1 2 3 4) 10)
//- (test "+" (+ 1 (+ 2 3)) 6)
pub fn fn_add(arguments: &[Node], _: &mut Environment) -> Node {
    Node::Number(
        arguments
        .iter()
        .map(|n| {
            if let Node::Number(num) = n {
                *num
            } else {
                panic!("Invalid argument for addition {n:?}");
            }
        })
        .sum())
}

//- (test "-" (- 1 2) -1)
//- (test "-" (- 1) -1)
//- (test "-" (- 1 2 3) -4)
pub fn fn_sub(arguments: &[Node], _: &mut Environment) -> Node {
    if arguments.len() == 1 {
        if let Node::Number(num) = &arguments[0] {
            return Node::Number(-num);
        }
    } else if arguments.len() > 1 {
        if let Node::Number(first) = &arguments[0] {
            let rest = &arguments[1..];
            let result = rest.into_iter().fold(*first, |acc, n| {
                if let Node::Number(num) = n {
                    acc - num
                } else {
                    panic!("Invalid argument for subtraction {n:?}");
                }
            });
            return Node::Number(result);
        }
    }
    panic!("Invalid arguments for subtraction");
}

//- (test "*" (* 1 2) 2)
//- (test "*" (* 1) 1)
//- (test "*" (* 1 2 -3) -6)
pub fn fn_mult(arguments: &[Node], _: &mut Environment) -> Node {
    Node::Number(
        arguments
        .iter()
        .map(|n| {
            if let Node::Number(num) = n {
                *num
            } else {
                panic!("Invalid argument for multiplication {n:?}");
            }
        })
        .product())
}

//- (test "even?" (even? 2) true)
//- (test "even?" (even? 3) false)
//- (test "even?" (even? 0) true)
pub fn fn_is_even(arguments: &[Node], _: &mut Environment) -> Node {
    if arguments.len() == 1 {
        if let Node::Number(num) = &arguments[0] {
            return Node::Bool(num % 2 == 0);
        }
    }
    panic!("Invalid arguments for is_even");
}
