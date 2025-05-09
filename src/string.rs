use crate::invalid_arguments;
use crate::node::Node;

macro_rules! all_list {
    ($args:expr) => {
        $args.iter().all(|arg| matches!(arg, Node::List(_)))
    };
}

macro_rules! all_text {
    ($args:expr) => {
        $args.iter().all(|arg| matches!(arg, Node::Text(_)))
    };
}

//- (test "concat" (concat (quote (1 2)) (quote (3 4))) (quote (1 2 3 4)))
//- (test "concat" (concat "Foo" "Bar") "FooBar")
//- (test "concat" (concat (quote ()) (quote (1))) (quote (1)))
//- (test "concat" (concat (quote (1 2)) (quote (4 1)) (quote (1))) (quote (1 2 4 1 1)))
//- (test "concat" (concat "Foo" "Bar" "Baz") "FooBarBaz")
pub fn fn_concat(arguments: &[Node]) -> Result<Node, String> {
    match arguments {
        [Node::List(_), ..] => {
            if !all_list!(arguments) {
                return Err(format!(
                    "Arguments to `concat` must all be of the same type.\nGot: {arguments:?}"
                ));
            }
            let mut result = vec![];
            for arg in arguments {
                if let Node::List(l) = arg {
                    result.extend_from_slice(l);
                } else {
                    return Err(format!(
                        "Arguments to `concat` must all be of the same type.\nGot: {arguments:?}"
                    ));
                }
            }

            Ok(Node::List(result))
        }
        [Node::Text(_), ..] => {
            if !all_text!(arguments) {
                return Err(format!(
                    "Arguments to `concat` must all be of the same type.\nGot: {arguments:?}"
                ));
            }
            let mut result = String::new();
            for arg in arguments {
                if let Node::Text(s) = arg {
                    result.push_str(s);
                } else {
                    return Err(format!(
                        "Arguments to `concat` must all be of the same type.\nGot: {arguments:?}"
                    ));
                }
            }

            Ok(Node::Text(result))
        }
        _ => {
            invalid_arguments!("concat", arguments, ["[Text(s1), ...]", "[List(l1), ...]"])
        }
    }
}

//- (test "split" (split "," "foo,bar,baz") (quote ("foo" "bar" "baz")))
//- (test "split" (split "," "foo") (quote ("foo")))
//- (test "split" (split "," "") (quote ("")))
pub fn fn_split(arguments: &[Node]) -> Result<Node, String> {
    match arguments {
        [Node::Text(delimiter), Node::Text(text)] => {
            let split = text
                .split(delimiter)
                .map(|s| Node::Text(s.to_string()))
                .collect();
            Ok(Node::List(split))
        }
        _ => {
            invalid_arguments!("split", arguments, ["[Text(delimiter), Text(text)]"])
        }
    }
}

//- (test "strip" (strip " foo ") "foo")
//- (test "strip" (strip "foo") "foo")
//- (test "strip" (strip "") "")
pub fn fn_strip(arguments: &[Node]) -> Result<Node, String> {
    match arguments {
        [Node::Text(s)] => Ok(Node::Text(s.trim().to_string())),
        _ => {
            invalid_arguments!("strip", arguments, ["[Text(s)]"])
        }
    }
}

//- (test "join" (join "," (quote ("foo" "bar" "baz"))) "foo,bar,baz")
//- (test "join" (join "," (quote ("foo"))) "foo")
//- (test "join" (join "," (quote ())) "")
pub fn fn_join(arguments: &[Node]) -> Result<Node, String> {
    match arguments {
        [Node::Text(delimiter), Node::List(list)] => {
            let joined = list
                .iter()
                .filter_map(|node| match node {
                    Node::Text(s) => Some(s.clone()),
                    _ => None,
                })
                .collect::<Vec<_>>()
                .join(delimiter);
            Ok(Node::Text(joined))
        }
        _ => {
            invalid_arguments!("join", arguments, ["[Text(delimiter), List(list)]"])
        }
    }
}

//- (test "index-of" (index-of "foo" "foobar") 0)
//- (test "index-of" (index-of "bar" "foobar") 3)
pub fn fn_index_of(arguments: &[Node]) -> Result<Node, String> {
    match arguments {
        [Node::Text(substring), Node::Text(text)] => match text.find(substring) {
            Some(index) => match index.try_into() {
                Ok(index) => Ok(Node::Number(index)),
                Err(_) => Err(format!("Index {index} is too large for a Number",)),
            },
            None => Err(format!("Substring '{substring}' not found in '{text}'",)),
        },
        _ => {
            invalid_arguments!("index-of", arguments, ["[Text(substring), Text(text)]"])
        }
    }
}

//- (test "substring" (substring "foobar" 0 3) "foo")
//- (test "substring" (substring "foobar" 3 6) "bar")
//- (test "substring" (substring "foobar" 3 3) "")
pub fn fn_substring(arguments: &[Node]) -> Result<Node, String> {
    match arguments {
        [Node::Text(text), Node::Number(start), Node::Number(end)] => {
            let start = usize::try_from(*start)
                .map_err(|_| format!("Start index {start} is too large for a Number"))?;
            let end = usize::try_from(*end)
                .map_err(|_| format!("End index {end} is too large for a Number"))?;
            if start <= end && end <= text.len() {
                Ok(Node::Text(text[start..end].to_string()))
            } else {
                Err(format!(
                    "Invalid substring range: {}..{} for text of length {}",
                    start,
                    end,
                    text.len()
                ))
            }
        }
        _ => {
            invalid_arguments!(
                "substring",
                arguments,
                ["[Text(text), Number(start), Number(end)]"]
            )
        }
    }
}

//- (test "replace" (replace "foo" "bar" "foobar") "barbar")
//- (test "replace" (replace "bar" "foo" "foobar") "foofoo")
//- (test "replace" (replace "baz" "foo" "foobar") "foobar")
pub fn fn_replace(arguments: &[Node]) -> Result<Node, String> {
    match arguments {
        [Node::Text(old), Node::Text(new), Node::Text(text)] => {
            let replaced = text.replace(old, new);
            Ok(Node::Text(replaced))
        }
        _ => {
            invalid_arguments!("replace", arguments, ["[Text(old), Text(new), Text(text)]"])
        }
    }
}

//- (test "upper" (upper "foo") "FOO")
//- (test "upper" (upper "FOO") "FOO")
//- (test "upper" (upper "") "")
pub fn fn_upper(arguments: &[Node]) -> Result<Node, String> {
    match arguments {
        [Node::Text(s)] => Ok(Node::Text(s.to_uppercase())),
        _ => {
            invalid_arguments!("upper", arguments, ["[Text(s)]"])
        }
    }
}

//- (test "lower" (lower "foo") "foo")
//- (test "lower" (lower "FOO") "foo")
//- (test "lower" (lower "") "")
pub fn fn_lower(arguments: &[Node]) -> Result<Node, String> {
    match arguments {
        [Node::Text(s)] => Ok(Node::Text(s.to_lowercase())),
        _ => {
            invalid_arguments!("lower", arguments, ["[Text(s)]"])
        }
    }
}

//- (test "starts-with?" (starts-with? "foo" "foobar") true)
//- (test "starts-with?" (starts-with? "bar" "foobar") false)
//- (test "starts-with?" (starts-with? "" "foobar") true)
pub fn fn_starts_with(arguments: &[Node]) -> Result<Node, String> {
    match arguments {
        [Node::Text(prefix), Node::Text(text)] => Ok(Node::Bool(text.starts_with(prefix))),
        _ => {
            invalid_arguments!("starts-with", arguments, ["[Text(prefix), Text(text)]"])
        }
    }
}

//- (test "ends-with?" (ends-with? "bar" "foobar") true)
//- (test "ends-with?" (ends-with? "foo" "foobar") false)
//- (test "ends-with?" (ends-with? "" "foobar") true)
pub fn fn_ends_with(arguments: &[Node]) -> Result<Node, String> {
    match arguments {
        [Node::Text(suffix), Node::Text(text)] => Ok(Node::Bool(text.ends_with(suffix))),
        _ => {
            invalid_arguments!("ends-with", arguments, ["[Text(suffix), Text(text)]"])
        }
    }
}
