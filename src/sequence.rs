use crate::node::Node;
use crate::environment::Environment;
use crate::apply;

//- (test "fold" (fold + 0 (quote (1 2 3))) 6)
//- (test "fold" (fold + 0 (quote ())) 0)
//- (test "fold" (fold + 0 (quote (1))) 1)
pub fn fn_fold(arguments: &[Node], env: &mut Environment) -> Node {
    if arguments.len() == 3 {
        let function = &arguments[0];
        let initial_value = &arguments[1];
        let list = &arguments[2];

        if let Node::List(l) = list {
            let mut result = initial_value.clone();
            for item in l {
                result = apply(function, &[result, item.clone()], env);
            }
            return result;
        }
    }
    panic!("Invalid arguments for fold");
}

//- (test "zip" (zip (quote (1 2 3)) (quote (4 5 6))) (quote ((1 4) (2 5) (3 6))))
//- (test "zip" (zip (quote ()) (quote ())) (quote ()))
//- (test "zip" (zip (quote (1)) (quote (2))) (quote ((1 2))))
pub fn fn_zip(arguments: &[Node], _: &mut Environment) -> Node {
    if arguments.len() == 2 {
        let list1 = &arguments[0];
        let list2 = &arguments[1];

        if let (Node::List(l1), Node::List(l2)) =
            (list1, list2)
        {
            let mut zipped = Vec::new();
            for (item1, item2) in l1.iter().zip(l2.iter()) {
                zipped.push(Node::List(vec![item1.clone(), item2.clone()]));
            }
            return Node::List(zipped);
        }
    }
    panic!("Invalid arguments for zip");
}

//- (test "range" (range 1 5) (quote (1 2 3 4)))
//- (test "range" (range 5) (quote (0 1 2 3 4)))
//- (test "range" (range 0 0) (quote ()))
pub fn fn_range(arguments: &[Node], _: &mut Environment) -> Node {
    if arguments.len() == 2 {
        if let (Node::Number(start), Node::Number(end)) =
            (&arguments[0], &arguments[1])
        {
            let mut range = Vec::new();
            for i in *start..*end {
                range.push(Node::Number(i));
            }
            return Node::List(range);
        }
    } else if arguments.len() == 1 {
        if let Node::Number(end) = &arguments[0] {
            let mut range = Vec::new();
            for i in 0..*end {
                range.push(Node::Number(i));
            }
            return Node::List(range);
        }
    }
    panic!("Invalid arguments for range");
}

pub fn fn_for_each(arguments: &[Node], env: &mut Environment) -> Node {
    if arguments.len() == 2 {
        let function = &arguments[0];
        let list = &arguments[1];

        if let Node::List(l) = list {
            for item in l {
                apply(function, &[item.clone()], env);
            }
            return Node::Bool(true);
        }
    }
    panic!("Invalid arguments for for-each");
}

//- (define inc (lambda (x) (+ x 1)))
//- (test "map" (map inc (quote (1 2 3))) (quote (2 3 4)))
//- (test "map" (map inc (quote ())) (quote ()))
//- (test "map" (map inc (quote (1))) (quote (2)))
pub fn fn_map(arguments: &[Node], env: &mut Environment) -> Node {
    if arguments.len() == 2 {
        let function = &arguments[0];
        let list = &arguments[1];

        if let Node::List(l) = list {
            let mut mapped = Vec::new();
            for item in l {
                mapped.push(apply(function, &[item.clone()], env));
            }
            return Node::List(mapped);
        }
    }

    panic!("Invalid arguments for map");
}

//- (test "filter" (filter even? (quote (1 2 3 4))) (quote (2 4)))
//- (test "filter" (filter even? (quote ())) (quote ()))
//- (test "filter" (filter even? (quote (1))) (quote ()))
pub fn fn_filter(arguments: &[Node], env: &mut Environment) -> Node {
    if arguments.len() == 2 {
        let function = &arguments[0];
        let list = &arguments[1];

        if let Node::List(l) = list {
            let mut filtered = Vec::new();
            for item in l {
                if let Node::Bool(b) = apply(function, &[item.clone()], env) {
                    if b {
                        filtered.push(item.clone());
                    }
                } else {
                    panic!("Function did not return a boolean value");
                }
            }
            return Node::List(filtered);
        }
    }

    panic!("Invalid arguments for filter");
}
