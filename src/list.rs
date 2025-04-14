use crate::node::Node;
use crate::environment::Environment;

//- (test "car" (car (quote (1 2 3))) 1)
//- (test "car" (car (quote ())) ())
//- (test "car" (car (quote (1))) 1)
pub fn fn_car(arguments: &[Node], _: &mut Environment) -> Node {
    if arguments.len() == 1 {
        if let Node::List(list) = &arguments[0] {
            if list.is_empty() {
                return Node::List(vec![]);
            }
            return list[0].clone();
        }
    }
    panic!("Invalid arguments for car");
}

//- (test "cdr" (cdr (quote (1 2 3))) (quote (2 3)))
//- (test "cdr" (cdr (quote ())) ())
//- (test "cdr" (cdr (quote (1))) ())
pub fn fn_cdr(arguments: &[Node], _: &mut Environment) -> Node {
    if arguments.len() == 1 {
        if let Node::List(list) = &arguments[0] {
            if list.len() > 1 {
                return Node::List(list[1..].to_vec());
            } else if list.len() <= 1 {
                return Node::List(vec![]);
            }
        }
    }
    panic!("Invalid arguments for cdr");
}

//- (test "cons" (cons 1 (quote (2 3))) (quote (1 2 3)))
//- (test "cons" (cons 1 (quote ())) (quote (1)))
//- (test "cons" (cons (quote (1)) (quote (2))) (quote ((1) 2)))
pub fn fn_cons(arguments: &[Node], _: &mut Environment) -> Node {
    if arguments.len() == 2 {
        if let Node::List(list) = &arguments[1] {
            let mut new_list = vec![arguments[0].clone()];
            new_list.extend_from_slice(list);
            return Node::List(new_list);
        }
    }
    panic!("Invalid arguments for cons");
}

//- (test "length" (length (quote (1 2 3))) 3)
//- (test "length" (length (quote ())) 0)
//- (test "length" (length (quote (1))) 1)
pub fn fn_length(arguments: &[Node], _: &mut Environment) -> Node {
    if arguments.len() == 1 {
        if let Node::List(list) = &arguments[0] {
            return Node::Number(
                i64::try_from(list.len()).expect("Failed to convert length"),
            );
        }
    }
    panic!("Invalid arguments for length: {:?}", &arguments[0]);
}

//- (test "null?" (null? (quote ())) true)
//- (test "null?" (null? (quote (1))) false)
//- (test "null?" (null? (quote "foo")) false)
pub fn fn_is_null(arguments: &[Node], _: &mut Environment) -> Node {
    if arguments.len() == 1 {
        if let Node::List(list) = &arguments[0] {
            return Node::Bool(list.is_empty());
        }

        return Node::Bool(false);
    }
    panic!("Invalid arguments for null?");
}

//- (test "list" (list 1 2 3) (quote (1 2 3)))
//- (test "list" (list) (quote ()))
//- (test "list" (list 1) (quote (1)))
pub fn fn_list(arguments: &[Node], _: &mut Environment) -> Node {
    Node::List(arguments.to_vec())
}

//- (test "last" (last (quote (1 2 3))) 3)
//- (test "last" (last (quote ())) ())
//- (test "last" (last (quote (1))) 1)
pub fn fn_last(arguments: &[Node], _: &mut Environment) -> Node {
    if arguments.len() == 1 {
        if let Node::List(list) = &arguments[0] {
            if list.is_empty() {
                return Node::List(vec![]);
            }
            return list.last().expect("Failed to get last element").clone();
        }
    }
    panic!("Invalid arguments for last");
}
