use crate::node::Node;

#[derive(Clone)]
pub struct Environment {
    // TODO: This shouldn't be public
    pub parent: Option<Box<Environment>>,
    pub variables: std::collections::HashMap<Node, Node>,
}

impl std::fmt::Display for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(parent) = &self.parent {
            write!(f, "{parent}")?;
        }

        writeln!(f, "Environment ID: {self:p}")?;
        let mut keys: Vec<_> = self.variables.keys().collect();
        keys.sort();

        for key in keys {
            let value = self.variables.get(key).expect("Key not found");
            writeln!(f, "  {key} => {value}")?;
        }

        Ok(())
    }
}

impl Environment {
    #[must_use]
    pub fn lookup(&self, node: &Node) -> Option<Node> {
        if let Some(value) = self.variables.get(node) {
            return Some(value.clone());
        } else if let Some(parent) = &self.parent {
            return parent.lookup(node);
        }

        None
    }

    pub fn add_function(
        &mut self,
        name: &str,
        function: fn(&[Node], &mut Self) -> Node,
    ) {
        self.variables.insert(
            Node::Symbol(name.to_string()),
            Node::Function(function),
        );
    }
}
