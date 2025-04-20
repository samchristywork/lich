use crate::invalid_arguments;
use crate::node::Node;
use rand::Rng;

pub fn fn_random_number(arguments: &[Node]) -> Result<Node, String> {
    match arguments {
        [] => Ok(Node::Number(rand::random::<i64>())),
        [Node::Number(max)] => {
            if *max < 0 {
                return Err("Maximum value cannot be negative".to_string());
            }
            let max = *max as i64;
            let random_number = rand::rng().random_range(0..=max);
            Ok(Node::Number(random_number))
        }
        [Node::Number(min), Node::Number(max)] => {
            if min > max {
                return Err("Minimum value cannot be greater than maximum value".to_string());
            }
            let min = *min as i64;
            let max = *max as i64;
            let random_number = rand::rng().random_range(min..=max);
            Ok(Node::Number(random_number))
        }
        _ => invalid_arguments!(
            "random-number",
            arguments,
            ["[]", "[Number(min), Number(max)]"]
        ),
    }
}
