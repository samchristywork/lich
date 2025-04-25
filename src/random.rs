use crate::invalid_arguments;
use crate::node::Node;
use rand::Rng;
use rand::prelude::IteratorRandom;

pub fn fn_random_number(arguments: &[Node]) -> Result<Node, String> {
    match arguments {
        [] => Ok(Node::Number(rand::random::<i64>())),
        [Node::Number(max)] => {
            if *max < 0 {
                return Err("Maximum value cannot be negative".to_string());
            }
            let max = { *max };
            let random_number = rand::rng().random_range(0..=max);
            Ok(Node::Number(random_number))
        }
        [Node::Number(min), Node::Number(max)] => {
            if min > max {
                return Err("Minimum value cannot be greater than maximum value".to_string());
            }
            let min = { *min };
            let max = { *max };
            let random_number = rand::rng().random_range(min..=max);
            Ok(Node::Number(random_number))
        }
        [Node::Float(max)] => {
            if *max < 0.0 {
                return Err("Maximum value cannot be negative".to_string());
            }
            let max = *max;
            let random_number = rand::rng().random_range(0.0..=max);
            Ok(Node::Float(random_number))
        }
        [Node::Float(min), Node::Float(max)] => {
            if min > max {
                return Err("Minimum value cannot be greater than maximum value".to_string());
            }
            let min = *min;
            let max = *max;
            let random_number = rand::rng().random_range(min..=max);
            Ok(Node::Float(random_number))
        }
        _ => invalid_arguments!(
            "random-number",
            arguments,
            [
                "[]",
                "[Number(max)]",
                "[Number(min), Number(max)]",
                "[]",
                "[Float(max)]",
                "[Float(min), Float(max)]"
            ]
        ),
    }
}

pub fn fn_random_letter(arguments: &[Node]) -> Result<Node, String> {
    let letters = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    match arguments {
        [] => Ok(Node::Text(
            letters
                .chars()
                .choose(&mut rand::thread_rng())
                .unwrap()
                .to_string(),
        )),
        _ => invalid_arguments!("random-letter", arguments, ["[]", "[Number(length)]"]),
    }
}

pub fn fn_random_string(arguments: &[Node]) -> Result<Node, String> {
    let letters = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    match arguments {
        [Node::Number(length)] => {
            if *length < 0 {
                return Err("Length cannot be negative".to_string());
            }
            let length = *length;
            let random_string: String = (0..length)
                .map(|_| letters.chars().choose(&mut rand::thread_rng()).unwrap())
                .collect();
            Ok(Node::Text(random_string))
        }
        _ => invalid_arguments!("random-string", arguments, ["[Number(length)]"]),
    }
}

pub fn fn_random_choice(arguments: &[Node]) -> Result<Node, String> {
    match arguments {
        [Node::List(list)] => {
            if list.is_empty() {
                return Err("Cannot choose from an empty list".to_string());
            }
            let random_index = rand::rng().random_range(0..list.len());
            Ok(list[random_index].clone())
        }
        _ => invalid_arguments!("random-choice", arguments, ["[List(list)]"]),
    }
}

pub fn fn_random_boolean(arguments: &[Node]) -> Result<Node, String> {
    match arguments {
        [] => Ok(Node::Bool(rand::random())),
        _ => invalid_arguments!("random-boolean", arguments, ["[]"]),
    }
}

pub fn fn_random_float(arguments: &[Node]) -> Result<Node, String> {
    match arguments {
        [] => Ok(Node::Float(rand::random())),
        _ => invalid_arguments!("random-float", arguments, ["[]"]),
    }
}
