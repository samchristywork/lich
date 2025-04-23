use crate::node::Node;

#[derive(Clone)]
pub struct Environment {
    parent: Option<Box<Environment>>,
    variables: std::collections::HashMap<String, Node>,
}

impl std::fmt::Display for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(parent) = &self.parent {
            write!(f, "{parent}")?;
        }

        writeln!(f, "Environment ID: {self:p}")?;

        let mut items = self
            .variables
            .keys()
            .map(|key| {
                let value = self.variables.get(key);
                match value {
                    Some(value) => format!("  {key} => {value}"),
                    None => format!("  {key} => <not found>"),
                }
            })
            .collect::<Vec<_>>();

        items.sort();
        for item in items {
            writeln!(f, "{item}")?;
        }

        Ok(())
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}

impl Environment {
    #[must_use]
    pub fn new() -> Self {
        Self {
            parent: None,
            variables: std::collections::HashMap::new(),
        }
    }

    #[must_use]
    pub fn from_parent(parent: Self) -> Self {
        Self {
            parent: Some(Box::new(parent)),
            variables: std::collections::HashMap::new(),
        }
    }

    pub fn insert(&mut self, name: &str, value: Node) {
        self.variables.insert(name.to_string(), value);
    }

    pub fn remove(&mut self, name: &str) {
        self.variables.remove(name);
    }

    #[must_use]
    pub fn lookup(&self, node: &Node) -> Option<Node> {
        match node {
            Node::Symbol(name) => {
                if let Some(value) = self.variables.get(name) {
                    Some(value.clone())
                } else if let Some(parent) = &self.parent {
                    parent.lookup(node)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    pub fn add_function(&mut self, name: &str, function: fn(&[Node]) -> Result<Node, String>) {
        self.variables
            .insert(name.to_string(), Node::Function(function));
    }
}
