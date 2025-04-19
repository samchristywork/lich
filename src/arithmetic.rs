use crate::environment::Environment;
use crate::invalid_arguments;
use crate::node::Node;

//- (test "+" (+ 1 2) 3)
//- (test "+" (+ 0 0) 0)
//- (test "+" (+ -1 1) 0)
pub fn fn_add(arguments: &[Node], _: &mut Environment) -> Result<Node, String> {
    match arguments {
        [Node::Number(a), Node::Number(b)] => Ok(Node::Number(a + b)),
        [Node::Float(a), Node::Float(b)] => Ok(Node::Float(a + b)),
        _ => invalid_arguments!(
            "+",
            arguments,
            ["[Number(a), Number(b)]", "[Float(a), Float(b)]"]
        ),
    }
}

//- (test "-" (- 1 2) -1)
//- (test "-" (- 0 0) 0)
//- (test "-" (- -1 1) -2)
pub fn fn_sub(arguments: &[Node], _: &mut Environment) -> Result<Node, String> {
    match arguments {
        [Node::Number(a), Node::Number(b)] => Ok(Node::Number(a - b)),
        [Node::Float(a), Node::Float(b)] => Ok(Node::Float(a - b)),
        _ => invalid_arguments!(
            "-",
            arguments,
            ["[Number(a), Number(b)]", "[Float(a), Float(b)]"]
        ),
    }
}

//- (test "*" (* 1 2) 2)
//- (test "*" (* 0 0) 0)
//- (test "*" (* -1 1) -1)
pub fn fn_mult(arguments: &[Node], _: &mut Environment) -> Result<Node, String> {
    match arguments {
        [Node::Number(a), Node::Number(b)] => Ok(Node::Number(a * b)),
        [Node::Float(a), Node::Float(b)] => Ok(Node::Float(a * b)),
        _ => invalid_arguments!(
            "*",
            arguments,
            ["[Number(a), Number(b)]", "[Float(a), Float(b)]"]
        ),
    }
}

pub fn fn_div(arguments: &[Node], _: &mut Environment) -> Result<Node, String> {
    match arguments {
        [Node::Float(a), Node::Float(b)] => {
            if *b == 0.0 {
                return Err("Division by zero".to_string());
            }
            Ok(Node::Float(a / b))
        }
        _ => invalid_arguments!("/", arguments, ["[Float(a), Float(b)]"]),
    }
}

//- (test "even?" (even? 2) true)
//- (test "even?" (even? 3) false)
//- (test "even?" (even? 0) true)
pub fn fn_is_even(arguments: &[Node], _: &mut Environment) -> Result<Node, String> {
    match arguments {
        [Node::Number(num)] => Ok(Node::Bool(num % 2 == 0)),
        _ => invalid_arguments!("even?", arguments, ["[Number(num)]"]),
    }
}

//- (test "odd?" (odd? 2) false)
//- (test "odd?" (odd? 3) true)
//- (test "odd?" (odd? 0) false)
pub fn fn_is_odd(arguments: &[Node], _: &mut Environment) -> Result<Node, String> {
    match arguments {
        [Node::Number(num)] => Ok(Node::Bool(num % 2 != 0)),
        _ => invalid_arguments!("odd?", arguments, ["[Number(num)]"]),
    }
}

//- (test "inc" (inc 1) 2)
//- (test "inc" (inc 0) 1)
//- (test "inc" (inc -1) 0)
pub fn fn_inc(arguments: &[Node], _: &mut Environment) -> Result<Node, String> {
    match arguments {
        [Node::Number(num)] => Ok(Node::Number(num + 1)),
        _ => invalid_arguments!("inc", arguments, ["[Number(num)]"]),
    }
}

//- (test "dec" (dec 1) 0)
//- (test "dec" (dec 0) -1)
//- (test "dec" (dec -1) -2)
pub fn fn_dec(arguments: &[Node], _: &mut Environment) -> Result<Node, String> {
    match arguments {
        [Node::Number(num)] => Ok(Node::Number(num - 1)),
        _ => invalid_arguments!("dec", arguments, ["[Number(num)]"]),
    }
}

//- (test "abs" (abs 1) 1)
//- (test "abs" (abs -1) 1)
//- (test "abs" (abs 0) 0)
pub fn fn_abs(arguments: &[Node], _: &mut Environment) -> Result<Node, String> {
    match arguments {
        [Node::Number(num)] => Ok(Node::Number(num.abs())),
        [Node::Float(num)] => Ok(Node::Float(num.abs())),
        _ => invalid_arguments!("abs", arguments, ["[Number(num)]", "[Float(num)]"]),
    }
}

//- (test "pow" (pow 2 3) 8)
//- (test "pow" (pow 2 0) 1)
//- (test "pow" (pow 0 2) 0)
pub fn fn_pow(arguments: &[Node], _: &mut Environment) -> Result<Node, String> {
    match arguments {
        [Node::Number(base), Node::Number(exp)] => Ok(Node::Number(base.pow(*exp as u32))),
        [Node::Float(base), Node::Float(exp)] => Ok(Node::Float(base.powf(*exp))),
        _ => invalid_arguments!(
            "pow",
            arguments,
            ["[Number(base), Number(exp)]", "[Float(base), Float(exp)]"]
        ),
    }
}

//- (test "negate" (negate 1) -1)
//- (test "negate" (negate -1) 1)
//- (test "negate" (negate 0) 0)
pub fn fn_negate(arguments: &[Node], _: &mut Environment) -> Result<Node, String> {
    match arguments {
        [Node::Number(num)] => Ok(Node::Number(-num)),
        _ => invalid_arguments!("negate", arguments, ["[Number(num)]"]),
    }
}
