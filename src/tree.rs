use crate::node::Node;

//- (test "leaves" (leaves (quote (1 2))) (quote (1 2)))
//- (test "leaves" (leaves (quote (1 2 3))) (quote (1 2 3)))
//- (test "leaves" (leaves (quote (1 2 (3 4)))) (quote (1 2 3 4)))
pub fn fn_leaves(arguments: &[Node]) -> Result<Node, String> {
    if arguments.len() == 1 {
        if let Node::List(l) = &arguments[0] {
            let mut result = Vec::new();
            for item in l {
                match item {
                    Node::List(inner) => {
                        let inner_leaves = fn_leaves(&[Node::List(inner.clone())])?;
                        if let Node::List(leaves) = inner_leaves {
                            result.extend(leaves);
                        }
                    }
                    _ => result.push(item.clone()),
                }
            }
            return Ok(Node::List(result));
        }
    }
    Err(format!("Invalid arguments for leaves: {:?}", &arguments[0]))
}

//- (test "depth" (depth (quote (1 2))) 1)
//- (test "depth" (depth (quote (1 2 3))) 1)
//- (test "depth" (depth (quote (1 2 (3 4)))) 2)
//- (test "depth" (depth (quote (1 2 (3 4) (5 6)))) 2)
//- (test "depth" (depth (quote (1 2 (3 4) (5 6 (7 8))))) 3)
pub fn fn_depth(arguments: &[Node]) -> Result<Node, String> {
    if arguments.len() == 1 {
        let list = &arguments[0];

        if let Node::List(l) = list {
            let mut max_depth = 0;
            for item in l {
                if let Node::List(inner) = item {
                    let inner_depth = fn_depth(&[Node::List(inner.clone())])?;
                    if let Node::Number(depth) = inner_depth {
                        if depth > max_depth {
                            max_depth = depth;
                        }
                    }
                }
            }
            return Ok(Node::Number(max_depth + 1));
        }
    }
    Err(format!("Invalid arguments for depth: {:?}", &arguments[0]))
}

fn format_tree_helper(node: &Node, depth: usize) -> String {
    let mut result = String::new();
    let indent = " ".repeat(depth * 2);
    match node {
        Node::List(l) => {
            for item in l {
                result.push_str(&format_tree_helper(item, depth + 1));
            }
        }
        _ => {
            result.push_str(&format!("{}{:?}\n", indent, node));
        }
    }
    result
}

pub fn fn_format_tree(arguments: &[Node]) -> Result<Node, String> {
    if arguments.len() == 1 {
        let list = &arguments[0];

        if let Node::List(_) = list {
            let formatted_tree = format_tree_helper(list, 0);
            return Ok(Node::Text(formatted_tree));
        }
    }
    Err(format!(
        "Invalid arguments for format-tree: {:?}",
        &arguments[0]
    ))
}
