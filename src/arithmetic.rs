use crate::node::Node;
use crate::environment::Environment;

//- (test "+" (+ 1 2 3) 6)
//- (test "+" (+ -10 2 3) -5)
//- (test "+" (+ 1 2 3 4) 10)
//- (test "+" (+ 1 (+ 2 3)) 6)
pub fn fn_add(arguments: &[Node], _: &mut Environment) -> Result<Node, String> {
    let mut result = 0;
    for n in arguments {
        if let Node::Number(num) = n {
            result += num;
        } else {
            return Err(format!("Invalid argument for addition {n:?}"));
        }
    }

    Ok(Node::Number(result))
}

//- (test "-" (- 1 2) -1)
//- (test "-" (- 1) -1)
//- (test "-" (- 1 2 3) -4)
pub fn fn_sub(arguments: &[Node], _: &mut Environment) -> Result<Node, String> {
    match arguments.len().cmp(&1) {
        std::cmp::Ordering::Equal => {
            if let Node::Number(num) = &arguments[0] {
                Ok(Node::Number(-num))
            } else {
                Err(format!("Invalid argument for subtraction {:?}", arguments[0]))
            }
        }
        std::cmp::Ordering::Greater => {
            if let Node::Number(first) = &arguments[0] {
                let rest = &arguments[1..];
                let mut result = *first;
                for n in rest {
                    if let Node::Number(num) = n {
                        result -= num;
                    } else {
                        return Err(format!("Invalid argument for subtraction {n:?}"));
                    }
                }
                Ok(Node::Number(result))
            } else {
                Err(format!("Invalid argument for subtraction {:?}", arguments[0]))
            }
        }
        std::cmp::Ordering::Less => {
            Err(format!("Invalid arguments for subtraction: {arguments:?}"))
        }
    }
}

//- (test "*" (* 1 2) 2)
//- (test "*" (* 1) 1)
//- (test "*" (* 1 2 -3) -6)
pub fn fn_mult(arguments: &[Node], _: &mut Environment) -> Result<Node, String> {
    let mut result = 1;
    for n in arguments {
        if let Node::Number(num) = n {
            result *= num;
        } else {
            return Err(format!("Invalid argument for multiplication {n:?}"));
        }
    }

    Ok(Node::Number(result))
}

//- (test "even?" (even? 2) true)
//- (test "even?" (even? 3) false)
//- (test "even?" (even? 0) true)
pub fn fn_is_even(arguments: &[Node], _: &mut Environment) -> Result<Node, String> {
    if arguments.len() == 1 {
        if let Node::Number(num) = &arguments[0] {
            return Ok(Node::Bool(num % 2 == 0));
        }
    }

    Err(format!("Invalid arguments for even?: {arguments:?}"))
}
