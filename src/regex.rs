use crate::node::Node;
use crate::environment::Environment;

pub fn fn_regex(arguments: &[Node], _: &mut Environment) -> Result<Node, String> {
    if arguments.len() == 1 {
        if let Node::Text(s) = &arguments[0] {
            return Ok(Node::Regex(s.clone()));
        }
    }
    Err("Invalid arguments for regex".to_string())
}

//- (test "regex-match" (regex-match (regex "^foo$") "foo") true)
//- (test "regex-match" (regex-match (regex "^foo$") "bar") false)
//- (test "regex-match" (regex-match (regex "foo") "foo bar") true)
//- (test "regex-match" (regex-match (regex "foob") "foo bar") false)
pub fn fn_regex_match(arguments: &[Node], _: &mut Environment) -> Result<Node, String> {
    if arguments.len() == 2 {
        if let Node::Regex(r) = &arguments[0] {
            if let Node::Text(s) = &arguments[1] {
                let re = regex::Regex::new(r).map_err(|_| "Invalid regex".to_string())?;
                return Ok(Node::Bool(re.is_match(s)));
            }
        }
    }
    Err("Invalid arguments for regex-match".to_string())
}

//- (test "regex-replace" (regex-replace (regex "^foo$") "foo" "bar") "bar")
//- (test "regex-replace" (regex-replace (regex "^foo$") "bar" "foo") "bar")
//- (test "regex-replace" (regex-replace (regex "foo") "foo bar" "bar") "bar bar")
pub fn fn_regex_replace(arguments: &[Node], _: &mut Environment) -> Result<Node, String> {
    if arguments.len() == 3 {
        if let Node::Regex(r) = &arguments[0] {
            if let Node::Text(s) = &arguments[1] {
                if let Node::Text(replace) = &arguments[2] {
                    let re = regex::Regex::new(r).map_err(|_| "Invalid regex".to_string())?;
                    return Ok(Node::Text(re.replace_all(s, replace).to_string()));
                }
            }
        }
    }
    Err("Invalid arguments for regex-replace".to_string())
}

//- (test "regex-split" (regex-split (regex "a") "bar") (quote ("b" "r")))
//- (test "regex-split" (regex-split (regex "a") "foo bar") (quote ("foo b" "r")))
//- (test "regex-split" (regex-split (regex "a") "foo bar baz") (quote ("foo b" "r b" "z")))
pub fn fn_regex_split(arguments: &[Node], _: &mut Environment) -> Result<Node, String> {
    if arguments.len() == 2 {
        if let Node::Regex(r) = &arguments[0] {
            if let Node::Text(s) = &arguments[1] {
                let re = regex::Regex::new(r).map_err(|_| "Invalid regex".to_string())?;
                return Ok(Node::List(
                    re.split(s)
                        .map(|s| Node::Text(s.to_string()))
                        .collect(),
                ));
            }
        }
    }
    Err("Invalid arguments for regex-split".to_string())
}
