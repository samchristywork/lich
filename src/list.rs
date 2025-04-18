use crate::environment::Environment;
use crate::node::Node;

//- (test "car" (car (quote (1 2 3))) 1)
//- (test "car" (car (quote ())) ())
//- (test "car" (car (quote (1))) 1)
pub fn fn_car(arguments: &[Node], _: &mut Environment) -> Result<Node, String> {
    if arguments.len() == 1 {
        if let Node::List(list) = &arguments[0] {
            if list.is_empty() {
                return Ok(Node::List(vec![]));
            }
            return Ok(list[0].clone());
        }
    }
    Err(format!("Invalid arguments for car: {:?}", &arguments[0]))
}

//- (test "cdr" (cdr (quote (1 2 3))) (quote (2 3)))
//- (test "cdr" (cdr (quote ())) ())
//- (test "cdr" (cdr (quote (1))) ())
pub fn fn_cdr(arguments: &[Node], _: &mut Environment) -> Result<Node, String> {
    if arguments.len() == 1 {
        if let Node::List(list) = &arguments[0] {
            if list.len() > 1 {
                return Ok(Node::List(list[1..].to_vec()));
            } else if list.len() <= 1 {
                return Ok(Node::List(vec![]));
            }
        }
    }
    Err("Invalid arguments for cdr".to_string())
}

//- (test "cons" (cons 1 (quote (2 3))) (quote (1 2 3)))
//- (test "cons" (cons 1 (quote ())) (quote (1)))
//- (test "cons" (cons (quote (1)) (quote (2))) (quote ((1) 2)))
pub fn fn_cons(arguments: &[Node], _: &mut Environment) -> Result<Node, String> {
    if arguments.len() == 2 {
        if let Node::List(list) = &arguments[1] {
            let mut new_list = vec![arguments[0].clone()];
            new_list.extend_from_slice(list);
            return Ok(Node::List(new_list));
        }
    }
    Err("Invalid arguments for cons".to_string())
}

//- (test "length" (length (quote (1 2 3))) 3)
//- (test "length" (length (quote ())) 0)
//- (test "length" (length (quote (1))) 1)
pub fn fn_length(arguments: &[Node], _: &mut Environment) -> Result<Node, String> {
    if arguments.len() == 1 {
        if let Node::List(list) = &arguments[0] {
            let n = i64::try_from(list.len());
            if let Ok(n) = n {
                return Ok(Node::Number(n));
            }
            return Err("Failed to convert length".to_string());
        }

        if let Node::Text(text) = &arguments[0] {
            let n = i64::try_from(text.len());
            if let Ok(n) = n {
                return Ok(Node::Number(n));
            }
            return Err("Failed to convert length".to_string());
        }
    }
    Err(format!("Invalid arguments for length: {:?}", &arguments[0]))
}

//- (test "null?" (null? (quote ())) true)
//- (test "null?" (null? (quote (1))) false)
//- (test "null?" (null? (quote "foo")) false)
pub fn fn_is_null(arguments: &[Node], _: &mut Environment) -> Result<Node, String> {
    if arguments.len() == 1 {
        if let Node::List(list) = &arguments[0] {
            return Ok(Node::Bool(list.is_empty()));
        }

        return Ok(Node::Bool(false));
    }
    Err("Invalid arguments for null?".to_string())
}

//- (test "list" (list 1 2 3) (quote (1 2 3)))
//- (test "list" (list) (quote ()))
//- (test "list" (list 1) (quote (1)))
pub fn fn_list(arguments: &[Node], _: &mut Environment) -> Result<Node, String> {
    Ok(Node::List(arguments.to_vec()))
}

//- (test "last" (last (quote (1 2 3))) 3)
//- (test "last" (last (quote ())) ())
//- (test "last" (last (quote (1))) 1)
pub fn fn_last(arguments: &[Node], _: &mut Environment) -> Result<Node, String> {
    if arguments.len() == 1 {
        if let Node::List(list) = &arguments[0] {
            if list.is_empty() {
                return Ok(Node::List(vec![]));
            }
            let last = list.last();
            if let Some(last) = last {
                return Ok(last.clone());
            }

            return Err("Failed to get last element".to_string());
        }
    }
    Err("Invalid arguments for last".to_string())
}

//- (test "nth" (nth 1 (quote (1 2 3))) 2)
//- (test "nth" (nth 0 (quote ())) ())
//- (test "nth" (nth 0 (quote (1))) 1)
pub fn fn_nth(arguments: &[Node], _: &mut Environment) -> Result<Node, String> {
    if arguments.len() == 2 {
        if let Node::List(list) = &arguments[1] {
            if let Node::Number(index) = &arguments[0] {
                let Ok(l) = i64::try_from(list.len()) else {
                    return Err("Failed to convert length".to_string());
                };
                if *index < 0 || *index >= l {
                    return Ok(Node::List(vec![]));
                }
                let Ok(index) = usize::try_from(*index) else {
                    return Err("Failed to convert index".to_string());
                };
                return Ok(list[index].clone());
            }
        }
    }
    Err("Invalid arguments for nth".to_string())
}
