use crate::invalid_arguments;
use crate::node::Node;


//- (test "zip" (zip (quote (1 2 3)) (quote (4 5 6))) (quote ((1 4) (2 5) (3 6))))
//- (test "zip" (zip (quote ()) (quote ())) (quote ()))
//- (test "zip" (zip (quote (1)) (quote (2))) (quote ((1 2))))
pub fn fn_zip(arguments: &[Node]) -> Result<Node, String> {
    match arguments {
        [Node::List(list1), Node::List(list2)] => {
            let mut zipped = Vec::new();
            let len = list1.len().min(list2.len());
            for i in 0..len {
                zipped.push(Node::List(vec![list1[i].clone(), list2[i].clone()]));
            }
            Ok(Node::List(zipped))
        }
        _ => invalid_arguments!("zip", arguments, ["[List(list1), List(list2)]"]),
    }
}

//- (test "range" (range 1 5) (quote (1 2 3 4)))
//- (test "range" (range 5) (quote (0 1 2 3 4)))
//- (test "range" (range 0 0) (quote ()))
pub fn fn_range(arguments: &[Node]) -> Result<Node, String> {
    match arguments {
        [Node::Number(end)] => {
            let mut range = Vec::new();
            for i in 0..*end {
                range.push(Node::Number(i));
            }
            Ok(Node::List(range))
        }
        [Node::Number(start), Node::Number(end)] => {
            let mut range = Vec::new();
            for i in *start..*end {
                range.push(Node::Number(i));
            }
            Ok(Node::List(range))
        }
        [Node::Number(start), Node::Number(end), Node::Number(step)] => {
            let mut range = Vec::new();
            if *step == 0 {
                return Err("Step cannot be zero".to_string());
            }
            let mut current = *start;
            while (step > &0 && current < *end) || (step < &0 && current > *end) {
                range.push(Node::Number(current));
                current += step;
            }
            Ok(Node::List(range))
        }
        _ => invalid_arguments!(
            "range",
            arguments,
            [
                "[Number(end)",
                "Number(start), Number(end)",
                "Number(start), Number(end), Number(step)]"
            ]
        ),
    }
}



