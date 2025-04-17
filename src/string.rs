use crate::environment::Environment;
use crate::node::Node;

//- (test "concat" (concat (quote (1 2)) (quote (3 4))) (quote (1 2 3 4)))
//- (test "concat" (concat "Foo" "Bar") "FooBar")
//- (test "concat" (concat (quote ()) (quote (1))) (quote (1)))
pub fn fn_concat(arguments: &[Node], _: &mut Environment) -> Result<Node, String> {
    if arguments.len() == 2 {
        if let (Node::Text(s1), Node::Text(s2)) = (&arguments[0], &arguments[1]) {
            return Ok(Node::Text(format!("{s1}{s2}")));
        } else if let (Node::List(l1), Node::List(l2)) = (&arguments[0], &arguments[1]) {
            let mut new_list = l1.clone();
            new_list.extend_from_slice(l2);
            return Ok(Node::List(new_list));
        }
    }
    Err(format!("Invalid arguments for concat: {:?}", &arguments[0]))
}

//- (test "split" (split "," "foo,bar,baz") (quote ("foo" "bar" "baz")))
//- (test "split" (split "," "foo") (quote ("foo")))
//- (test "split" (split "," "") (quote ("")))
pub fn fn_split(arguments: &[Node], _: &mut Environment) -> Result<Node, String> {
    if arguments.len() == 2 {
        if let (Node::Text(delimiter), Node::Text(text)) = (&arguments[0], &arguments[1]) {
            let parts: Vec<_> = text
                .split(delimiter)
                .map(|s| Node::Text(s.to_string()))
                .collect();
            return Ok(Node::List(parts));
        }
    }
    Err(format!("Invalid arguments for split: {:?}", &arguments[0]))
}

//- (test "strip" (strip " foo ") "foo")
//- (test "strip" (strip "foo") "foo")
//- (test "strip" (strip "") "")
pub fn fn_strip(arguments: &[Node], _: &mut Environment) -> Result<Node, String> {
    if arguments.len() == 1 {
        if let Node::Text(s) = &arguments[0] {
            return Ok(Node::Text(s.trim().to_string()));
        }
    }
    Err(format!("Invalid arguments for strip: {:?}", &arguments[0]))
}

//- (test "join" (join "," (quote ("foo" "bar" "baz"))) "foo,bar,baz")
//- (test "join" (join "," (quote ("foo"))) "foo")
//- (test "join" (join "," (quote ())) "")
pub fn fn_join(arguments: &[Node], _: &mut Environment) -> Result<Node, String> {
    if arguments.len() == 2 {
        if let (Node::Text(delimiter), Node::List(list)) = (&arguments[0], &arguments[1]) {
            let joined = list
                .iter()
                .map(std::string::ToString::to_string)
                .collect::<Vec<_>>()
                .join(delimiter);
            return Ok(Node::Text(joined));
        }
    }
    Err(format!("Invalid arguments for join: {:?}", &arguments[0]))
}

//- (test "index-of" (index-of "foo" "foobar") 0)
//- (test "index-of" (index-of "bar" "foobar") 3)
//- (test "index-of" (index-of "baz" "foobar") -1)
pub fn fn_index_of(arguments: &[Node], _: &mut Environment) -> Result<Node, String> {
    if arguments.len() == 2 {
        if let (Node::Text(substring), Node::Text(text)) = (&arguments[0], &arguments[1]) {
            return text
                .find(substring)
                .map_or(Ok(Node::Number(-1)), |n| Ok(Node::Number(n as i64)));
        }
    }
    Err(format!(
        "Invalid arguments for index-of: {:?}",
        &arguments[0]
    ))
}

//- (test "substring" (substring "foobar" 0 3) "foo")
//- (test "substring" (substring "foobar" 3 6) "bar")
//- (test "substring" (substring "foobar" 3 3) "")
pub fn fn_substring(arguments: &[Node], _: &mut Environment) -> Result<Node, String> {
    if arguments.len() == 3 {
        if let (Node::Text(text), Node::Number(start), Node::Number(end)) =
            (&arguments[0], &arguments[1], &arguments[2])
        {
            let start = *start as usize;
            let end = *end as usize;
            return Ok(Node::Text(text[start..end].to_string()));
        }
    }
    Err(format!(
        "Invalid arguments for substring: {:?}",
        &arguments[0]
    ))
}

//- (test "replace" (replace "foo" "bar" "foobar") "barbar")
//- (test "replace" (replace "bar" "foo" "foobar") "foofoo")
//- (test "replace" (replace "baz" "foo" "foobar") "foobar")
pub fn fn_replace(arguments: &[Node], _: &mut Environment) -> Result<Node, String> {
    if arguments.len() == 3 {
        if let (Node::Text(old), Node::Text(new), Node::Text(text)) =
            (&arguments[0], &arguments[1], &arguments[2])
        {
            return Ok(Node::Text(text.replace(old, new)));
        }
    }
    Err(format!(
        "Invalid arguments for replace: {:?}",
        &arguments[0]
    ))
}

//- (test "upper" (upper "foo") "FOO")
//- (test "upper" (upper "FOO") "FOO")
//- (test "upper" (upper "") "")
pub fn fn_upper(arguments: &[Node], _: &mut Environment) -> Result<Node, String> {
    if arguments.len() == 1 {
        if let Node::Text(s) = &arguments[0] {
            return Ok(Node::Text(s.to_uppercase()));
        }
    }
    Err(format!("Invalid arguments for upper: {:?}", &arguments[0]))
}

//- (test "lower" (lower "foo") "foo")
//- (test "lower" (lower "FOO") "foo")
//- (test "lower" (lower "") "")
pub fn fn_lower(arguments: &[Node], _: &mut Environment) -> Result<Node, String> {
    if arguments.len() == 1 {
        if let Node::Text(s) = &arguments[0] {
            return Ok(Node::Text(s.to_lowercase()));
        }
    }
    Err(format!("Invalid arguments for lower: {:?}", &arguments[0]))
}
