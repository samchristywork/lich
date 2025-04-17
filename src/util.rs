use crate::environment::Environment;
use crate::eval;
use crate::node::Node;

//- (test "begin" (begin 1 2 3) 3)
//- (test "begin" (begin) ())
//- (test "begin" (begin (define x 1) (define x (+ x 1)) x) 2)
pub fn fn_begin(arguments: &[Node], env: &mut Environment) -> Result<Node, String> {
    let mut result = Node::List(vec![]);
    for arg in arguments {
        result = eval(arg, env)?;
    }

    Ok(result)
}

//- (test "type?" (type? (quote ())) "list")
//- (test "type?" (type? (quote 1)) "number")
//- (test "type?" (type? (quote "foo")) "string")
pub fn fn_type(arguments: &[Node], _: &mut Environment) -> Result<Node, String> {
    if arguments.len() == 1 {
        match &arguments[0] {
            Node::Number(_) => return Ok(Node::Text("number".to_string())),
            Node::Text(_) => return Ok(Node::Text("string".to_string())),
            Node::Bool(_) => return Ok(Node::Text("bool".to_string())),
            Node::List(_) => return Ok(Node::Text("list".to_string())),
            Node::Symbol(_) => return Ok(Node::Text("symbol".to_string())),
            Node::Function(_) => return Ok(Node::Text("function".to_string())),
            Node::Regex(_) => return Ok(Node::Text("regex".to_string())),
            Node::Time(_, _) => return Ok(Node::Text("time".to_string())),
        }
    }
    Err("Invalid arguments for type".to_string())
}

pub fn fn_time_ms(arguments: &[Node], env: &mut Environment) -> Result<Node, String> {
    if arguments.len() == 1 {
        let start = std::time::Instant::now();
        eval(&arguments[0], env)?;
        let duration = start.elapsed();
        let result = duration.as_millis().try_into();
        if let Ok(d) = result {
            return Ok(Node::Number(d));
        }
    }
    Err("Invalid arguments for time-ms".to_string())
}
