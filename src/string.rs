use crate::node::Node;
use crate::environment::Environment;

//- (test "concat" (concat (quote (1 2)) (quote (3 4))) (quote (1 2 3 4)))
//- (test "concat" (concat "Foo" "Bar") "FooBar")
//- (test "concat" (concat (quote ()) (quote (1))) (quote (1)))
pub fn fn_concat(arguments: &[Node], _: &mut Environment) -> Node {
    if arguments.len() == 2 {
        if let (Node::Text(s1), Node::Text(s2)) =
            (&arguments[0], &arguments[1])
        {
            return Node::Text(format!("{s1}{s2}"));
        } else if let (Node::List(l1), Node::List(l2)) =
            (&arguments[0], &arguments[1])
        {
            let mut new_list = l1.clone();
            new_list.extend_from_slice(l2);
            return Node::List(new_list);
        }
    }
    panic!("Invalid arguments for concat");
}

//- (test "split" (split "," "foo,bar,baz") (quote ("foo" "bar" "baz")))
//- (test "split" (split "," "foo") (quote ("foo")))
//- (test "split" (split "," "") (quote ("")))
pub fn fn_split(arguments: &[Node], _: &mut Environment) -> Node {
    if arguments.len() == 2 {
        if let (Node::Text(delimiter), Node::Text(text)) =
            (&arguments[0], &arguments[1])
        {
            let parts: Vec<_> = text.split(delimiter).map(|s| Node::Text(s.to_string())).collect();
            return Node::List(parts);
        }
    }
    panic!("Invalid arguments for split");
}

//- (test "strip" (strip " foo ") "foo")
//- (test "strip" (strip "foo") "foo")
//- (test "strip" (strip "") "")
pub fn fn_strip(arguments: &[Node], _: &mut Environment) -> Node {
    if arguments.len() == 1 {
        if let Node::Text(s) = &arguments[0] {
            return Node::Text(s.trim().to_string());
        }
    }
    panic!("Invalid arguments for strip");
}
