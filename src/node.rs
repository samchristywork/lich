use crate::environment::Environment;

#[derive(Debug, Clone, PartialEq, Hash, Eq, Ord, PartialOrd)]
pub enum Node {
    Symbol(String),
    Number(i64),
    Text(String),
    Bool(bool),
    List(Vec<Node>),
    Function(fn(&[Node], &mut Environment) -> Node),
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = match self {
            Self::Number(n) => n.to_string(),
            Self::Bool(b) => b.to_string(),
            Self::Text(s) | Self::Symbol(s) => s.clone(),
            Self::Function(_) => "function".to_string(),
            Self::List(nodes) => {
                let mut result = String::new();
                result.push('(');
                result.push_str(
                    &nodes
                    .iter()
                    .map(Self::to_string)
                    .collect::<Vec<_>>()
                    .join(" "),
                );
                result.push(')');
                result
            }
        };

        if res.is_empty() {
            write!(f, "nil")
        } else {
            write!(f, "{res}")
        }
    }
}
