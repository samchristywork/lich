use crate::environment::Environment;
use crate::node::Node;

pub fn eval(node: &Node, env: &mut Environment) -> Result<Node, String> {
    match node {
        Node::Symbol(_) => env
            .lookup(node)
            .ok_or_else(|| format!("Undefined variable: {node:?}")),
        Node::Number(_)
        | Node::Text(_)
        | Node::Bool(_)
        | Node::Function(_)
        | Node::Regex(_)
        | Node::Time(_, _) => Ok(node.clone()),
        Node::List(nodes) => eval_list(nodes, env),
    }
}

fn eval_if(rest: &[Node], env: &mut Environment) -> Result<Node, String> {
    let condition = eval(&rest[0], env);
    match condition {
        Ok(Node::Bool(true)) => eval(&rest[1], env),
        Ok(Node::Bool(false)) => eval(&rest[2], env),
        _ => Err("Condition must be a boolean".to_string()),
    }
}

fn eval_cond(rest: &[Node], env: &mut Environment) -> Result<Node, String> {
    for condition in rest {
        if let Node::List(conditions) = condition {
            if conditions.len() == 2 {
                let cond = eval(&conditions[0], env);
                if cond == Ok(Node::Bool(true)) {
                    return eval(&conditions[1], env);
                }
            } else {
                return Err("Invalid cond clause".to_string());
            }
        } else {
            return Err("Invalid cond clause".to_string());
        }
    }

    Err("No true condition found in cond".to_string())
}

fn eval_define(rest: &[Node], env: &mut Environment) -> Result<Node, String> {
    let variable = &rest[0];
    let value = eval(&rest[1], env)?;
    env.variables.insert(variable.clone(), value.clone());

    Ok(value)
}

fn eval_lambda(rest: &[Node]) -> Result<Node, String> {
    if rest.len() != 2 {
        return Err("Invalid arguments for lambda".to_string());
    }

    let parameters = rest[0].clone();
    let body = rest[1].clone();

    Ok(Node::List(vec![
        Node::Symbol("lambda".to_string()),
        parameters,
        body,
    ]))
}

fn eval_let(rest: &[Node], env: &mut Environment) -> Result<Node, String> {
    if rest.len() == 2 {
        let bindings = &rest[0];
        let body = &rest[1];

        if let Node::List(bindings_list) = bindings {
            let mut new_env = Environment {
                parent: Some(Box::new(env.clone())),
                variables: std::collections::HashMap::new(),
            };

            for binding in bindings_list {
                if let Node::List(binding_pair) = binding {
                    if binding_pair.len() == 2 {
                        let variable = &binding_pair[0];
                        let value = eval(&binding_pair[1], env)?;
                        new_env.variables.insert(variable.clone(), value);
                    } else {
                        return Err("Invalid binding pair".to_string());
                    }
                } else {
                    return Err("Invalid binding".to_string());
                }
            }

            return eval(body, &mut new_env);
        }
    }

    Err("Invalid arguments for let".to_string())
}

fn eval_let_restricted(rest: &[Node], env: &mut Environment) -> Result<Node, String> {
    if rest.len() == 2 {
        let bindings = &rest[0];
        let body = &rest[1];

        if let Node::List(bindings_list) = bindings {
            let mut new_env = Environment {
                parent: None,
                variables: std::collections::HashMap::new(),
            };

            for binding in bindings_list {
                if let Node::List(binding_pair) = binding {
                    if binding_pair.len() == 1 {
                        let variable = &binding_pair[0];
                        let value = eval(&binding_pair[0], env)?;
                        new_env.variables.insert(variable.clone(), value);
                    } else if binding_pair.len() == 2 {
                        let variable = &binding_pair[0];
                        let value = eval(&binding_pair[1], env)?;
                        new_env.variables.insert(variable.clone(), value);
                    } else {
                        return Err("Invalid binding pair".to_string());
                    }
                } else {
                    return Err("Invalid binding".to_string());
                }
            }

            return eval(body, &mut new_env);
        }
    }

    Err("Invalid arguments for let-restricted".to_string())
}

fn eval_list(nodes: &[Node], env: &mut Environment) -> Result<Node, String> {
    if nodes.is_empty() {
        return Ok(Node::List(vec![]));
    }

    let first = &nodes[0];
    let rest = &nodes[1..];

    Ok(match first {
        Node::Symbol(s) => {
            let operator = s.as_str();

            match operator {
                "quote" => rest[0].clone(),
                "if" => eval_if(rest, env)?,
                "cond" => eval_cond(rest, env)?,
                "define" => eval_define(rest, env)?,
                "lambda" => eval_lambda(rest)?,
                "let" => eval_let(rest, env)?,
                "let-restricted" => eval_let_restricted(rest, env)?,
                _ => {
                    let function = env
                        .lookup(first)
                        .ok_or_else(|| format!("Undefined function: {first:?}"))?;
                    let arguments = rest
                        .iter()
                        .map(|n| eval(n, env))
                        .collect::<Result<Vec<_>, _>>()?;
                    apply(&function, &arguments, env)?
                }
            }
        }
        _ => return Err(format!("Invalid expression: {first:?}")),
    })
}

pub fn apply(function: &Node, arguments: &[Node], env: &mut Environment) -> Result<Node, String> {
    Ok(match function {
        Node::Function(f) => f(arguments, env)?,
        Node::List(nodes) => {
            if let Node::Symbol(s) = &nodes[0] {
                if s == "lambda" {
                    if let Node::List(params) = &nodes[1] {
                        if params.len() != arguments.len() {
                            return Err(format!(
                                "Argument count mismatch: expected {}, got {}",
                                params.len(),
                                arguments.len()
                            ));
                        }
                        let mut new_env = Environment {
                            parent: Some(Box::new(env.clone())),
                            variables: std::collections::HashMap::new(),
                        };
                        for (param, arg) in params.iter().zip(arguments) {
                            new_env.variables.insert(param.clone(), arg.clone());
                        }
                        return eval(&nodes[2], &mut new_env);
                    }
                }
            }
            return Err(format!(
                "Function application not implemented: {function:?}"
            ));
        }
        _ => return Err(format!("Invalid function: {function:?}")),
    })
}
