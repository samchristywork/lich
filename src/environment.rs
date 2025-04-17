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

        // Sort and print the variables
        let mut items = self.variables.keys().map(|key| {
            let value = self.variables.get(key);
            match value {
                Some(value) => format!("  {key} => {value}"),
                None => format!("  {key} => <not found>"),
            }
        }).collect::<Vec<_>>();

        items.sort();
        for item in items {
            writeln!(f, "{item}")?;
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
        function: fn(&[Node], &mut Self) -> Result<Node, String>,
    ) {
        self.variables
            .insert(Node::Symbol(name.to_string()), Node::Function(function));
    }
}
