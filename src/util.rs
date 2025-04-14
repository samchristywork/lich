use crate::node::Node;
use crate::environment::Environment;
use crate::eval;

//- (test "begin" (begin 1 2 3) 3)
//- (test "begin" (begin) ())
//- (test "begin" (begin (define x 1) (define x (+ x 1)) x) 2)
pub fn fn_begin(arguments: &[Node], env: &mut Environment) -> Node {
    let mut result = Node::List(vec![]);
    for arg in arguments {
        result = eval(arg, env);
    }

    result
}

//- (test "type?" (type? (quote ())) "list")
//- (test "type?" (type? (quote 1)) "number")
//- (test "type?" (type? (quote "foo")) "string")
pub fn fn_type(arguments: &[Node], _: &mut Environment) -> Node {
    if arguments.len() == 1 {
        match &arguments[0] {
            Node::Number(_) => return Node::Text("number".to_string()),
            Node::Text(_) => return Node::Text("string".to_string()),
            Node::Bool(_) => return Node::Text("bool".to_string()),
            Node::List(_) => return Node::Text("list".to_string()),
            Node::Symbol(_) => return Node::Text("symbol".to_string()),
            Node::Function(_) => return Node::Text("function".to_string()),
        }
    }
    panic!("Invalid arguments for type?");
}

pub fn fn_time_ms(arguments: &[Node], env: &mut Environment) -> Node {
    if arguments.len() == 1 {
        let start = std::time::Instant::now();
        eval(&arguments[0], env);
        let duration = start.elapsed();
        return Node::Number(duration.as_millis().try_into().expect("Failed to convert duration"));
    }
    panic!("Invalid arguments for time-ms");
}

//- (test "among" (among (quote (1 2 3)) 2) true)
//- (test "among" (among (quote (1 2 3)) 4) false)
//- (test "among" (among (quote (1 "foo" 3)) "foo") true)
//- (test "among" (among (quote ()) 1) false)
pub fn fn_among(arguments: &[Node], _: &mut Environment) -> Node {
    if arguments.len() == 2 {
        let list = &arguments[0];
        let value = &arguments[1];

        if let Node::List(l) = list {
            for item in l {
                if item == value {
                    return Node::Bool(true);
                }
            }
            return Node::Bool(false);
        }
    }
    panic!("Invalid arguments for among");
}
