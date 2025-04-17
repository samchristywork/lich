use crate::environment::Environment;
use crate::eval::apply;
use crate::node::Node;

//- (test "fold" (fold + 0 (quote (1 2 3))) 6)
//- (test "fold" (fold + 0 (quote ())) 0)
//- (test "fold" (fold + 0 (quote (1))) 1)
pub fn fn_fold(arguments: &[Node], env: &mut Environment) -> Result<Node, String> {
    if arguments.len() == 3 {
        let function = &arguments[0];
        let initial_value = &arguments[1];
        let list = &arguments[2];

        if let Node::List(l) = list {
            let mut result = initial_value.clone();
            for item in l {
                result = apply(function, &[result, item.clone()], env)?;
            }
            return Ok(result);
        }
    }
    Err(format!("Invalid arguments for fold: {:?}", &arguments[0]))
}

//- (test "zip" (zip (quote (1 2 3)) (quote (4 5 6))) (quote ((1 4) (2 5) (3 6))))
//- (test "zip" (zip (quote ()) (quote ())) (quote ()))
//- (test "zip" (zip (quote (1)) (quote (2))) (quote ((1 2))))
pub fn fn_zip(arguments: &[Node], _: &mut Environment) -> Result<Node, String> {
    if arguments.len() == 2 {
        let list1 = &arguments[0];
        let list2 = &arguments[1];

        if let (Node::List(l1), Node::List(l2)) = (list1, list2) {
            let mut zipped = Vec::new();
            for (item1, item2) in l1.iter().zip(l2.iter()) {
                zipped.push(Node::List(vec![item1.clone(), item2.clone()]));
            }
            return Ok(Node::List(zipped));
        }
    }
    Err(format!("Invalid arguments for zip: {:?}", &arguments[0]))
}

//- (test "range" (range 1 5) (quote (1 2 3 4)))
//- (test "range" (range 5) (quote (0 1 2 3 4)))
//- (test "range" (range 0 0) (quote ()))
pub fn fn_range(arguments: &[Node], _: &mut Environment) -> Result<Node, String> {
    if arguments.len() == 2 {
        if let (Node::Number(start), Node::Number(end)) = (&arguments[0], &arguments[1]) {
            let mut range = Vec::new();
            for i in *start..*end {
                range.push(Node::Number(i));
            }
            return Ok(Node::List(range));
        }
    } else if arguments.len() == 1 {
        if let Node::Number(end) = &arguments[0] {
            let mut range = Vec::new();
            for i in 0..*end {
                range.push(Node::Number(i));
            }
            return Ok(Node::List(range));
        }
    }
    Err(format!("Invalid arguments for range: {:?}", &arguments[0]))
}

pub fn fn_for_each(arguments: &[Node], env: &mut Environment) -> Result<Node, String> {
    if arguments.len() == 2 {
        let function = &arguments[0];
        let list = &arguments[1];

        if let Node::List(l) = list {
            for item in l {
                apply(function, &[item.clone()], env)?;
            }
            return Ok(Node::Bool(true));
        }
    }
    Err(format!(
        "Invalid arguments for for-each: {:?}",
        &arguments[0]
    ))
}

//- (define inc (lambda (x) (+ x 1)))
//- (test "map" (map inc (quote (1 2 3))) (quote (2 3 4)))
//- (test "map" (map inc (quote ())) (quote ()))
//- (test "map" (map inc (quote (1))) (quote (2)))
pub fn fn_map(arguments: &[Node], env: &mut Environment) -> Result<Node, String> {
    if arguments.len() == 2 {
        let function = &arguments[0];
        let list = &arguments[1];

        if let Node::List(l) = list {
            let mut mapped = Vec::new();
            for item in l {
                mapped.push(apply(function, &[item.clone()], env)?);
            }
            return Ok(Node::List(mapped));
        }
    }
    Err(format!("Invalid arguments for map: {:?}", &arguments[0]))
}

//- (test "filter" (filter even? (quote (1 2 3 4))) (quote (2 4)))
//- (test "filter" (filter even? (quote ())) (quote ()))
//- (test "filter" (filter even? (quote (1))) (quote ()))
pub fn fn_filter(arguments: &[Node], env: &mut Environment) -> Result<Node, String> {
    if arguments.len() == 2 {
        let function = &arguments[0];
        let list = &arguments[1];

        if let Node::List(l) = list {
            let mut filtered = Vec::new();
            for item in l {
                if let Node::Bool(b) = apply(function, &[item.clone()], env)? {
                    if b {
                        filtered.push(item.clone());
                    }
                } else {
                    return Err(format!("Function did not return a boolean value: {item:?}",));
                }
            }
            return Ok(Node::List(filtered));
        }
    }
    Err(format!("Invalid arguments for filter: {:?}", &arguments[0]))
}
