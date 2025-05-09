use crate::environment::Environment;
use crate::invalid_arguments;
use crate::node::Node;

pub fn eval(node: &Node, env: &mut Environment) -> Result<Node, String> {
    match node {
        Node::Symbol(_) => env
            .lookup(node)
            .ok_or_else(|| format!("Undefined variable: {node:?}")),
        Node::Number(_)
        | Node::Text(_)
        | Node::Bool(_)
        | Node::Float(_)
        | Node::Function(_)
        | Node::Regex(_)
        | Node::Time(_, _) => Ok(node.clone()),
        Node::List(nodes) => eval_list(nodes, env),
    }
}

fn eval_if(rest: &[Node], env: &mut Environment) -> Result<Node, String> {
    if rest.len() == 3 {
        let condition = eval(&rest[0], env);
        match condition {
            Ok(Node::Bool(true)) => eval(&rest[1], env),
            Ok(Node::Bool(false)) => eval(&rest[2], env),
            Err(e) => Err(e),
            _ => Err(format!("Condition must be a boolean: {condition:?}")),
        }
    } else if rest.len() == 2 {
        let condition = eval(&rest[0], env);
        match condition {
            Ok(Node::Bool(true)) => eval(&rest[1], env),
            Ok(Node::Bool(false)) => Ok(Node::List(vec![])),
            Err(e) => Err(e),
            _ => Err(format!("Condition must be a boolean: {condition:?}")),
        }
    } else {
        Err("Invalid arguments for if".to_string())
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

fn eval_begin(rest: &[Node], env: &mut Environment) -> Result<Node, String> {
    if rest.is_empty() {
        return Ok(Node::List(vec![]));
    }

    let mut result = Node::List(vec![]);
    for expr in rest {
        result = eval(expr, env)?;
    }

    Ok(result)
}

fn eval_define(rest: &[Node], env: &mut Environment) -> Result<Node, String> {
    if rest.len() == 2 {
        let variable = &rest[0];
        let value = eval(&rest[1], env)?;
        match variable {
            Node::Symbol(s) => env.insert(s, value.clone()),
            _ => return Err(format!("Invalid variable in define: {variable:?}")),
        }

        Ok(value)
    } else if rest.len() == 1 {
        let variable = &rest[0];
        let value = Node::Bool(true);
        match variable {
            Node::Symbol(s) => env.insert(s, value.clone()),
            _ => return Err(format!("Invalid variable in define: {variable:?}")),
        }

        Ok(value)
    } else {
        Err("Invalid arguments for define".to_string())
    }
}

fn eval_undefine(rest: &[Node], env: &mut Environment) -> Result<Node, String> {
    if rest.len() == 1 {
        let variable = &rest[0];
        match variable {
            Node::Symbol(s) => env.remove(s),
            _ => return Err(format!("Invalid variable in undefine: {variable:?}")),
        }

        Ok(Node::Bool(true))
    } else {
        Err("Invalid arguments for undefine".to_string())
    }
}

fn eval_is_defined(rest: &[Node], env: &Environment) -> Result<Node, String> {
    if rest.len() == 1 {
        let variable = &rest[0];
        let is_defined = env.lookup(variable).is_some();

        Ok(Node::Bool(is_defined))
    } else {
        Err("Invalid arguments for defined?".to_string())
    }
}

fn eval_get_type(rest: &[Node], env: &mut Environment) -> Result<Node, String> {
    if rest.len() == 1 {
        let value = eval(&rest[0], env)?;
        let type_name = match value {
            Node::Number(_) => "number",
            Node::Text(_) => "text",
            Node::Float(_) => "float",
            Node::Bool(_) => "bool",
            Node::Function(_) => "function",
            Node::Regex(_) => "regex",
            Node::Time(_, _) => "time",
            Node::Symbol(_) => "symbol",
            Node::List(_) => "list",
        };

        Ok(Node::Text(type_name.to_string()))
    } else {
        Err("Invalid arguments for type?".to_string())
    }
}

pub fn eval_print_env(_: &[Node], env: &mut Environment) -> Result<Node, String> {
    println!("{env}");
    Ok(Node::Bool(true))
}

fn eval_lambda(rest: &[Node]) -> Result<Node, String> {
    if rest.len() != 2 {
        return Err(format!(
            "Invalid arguments for lambda: expected 2, got {}",
            rest.len()
        ));
    }

    let parameters = rest[0].clone();
    let body = rest[1].clone();

    Ok(Node::List(vec![
        Node::Symbol("lambda".to_string()),
        parameters,
        body,
    ]))
}

//- (test "map" (map inc (quote (1 2 3))) (quote (2 3 4)))
//- (test "map" (map inc (quote ())) (quote ()))
//- (test "map" (map inc (quote (1))) (quote (2)))
fn eval_map(rest: &[Node], env: &mut Environment) -> Result<Node, String> {
    let rest = rest
        .iter()
        .map(|n| eval(n, env))
        .collect::<Result<Vec<_>, _>>()?;
    match &rest[..] {
        [function, Node::List(list)] => {
            let mut mapped = Vec::new();
            for item in list {
                mapped.push(apply(function, &[item.clone()], env)?);
            }
            Ok(Node::List(mapped))
        }
        [function, Node::List(list), Node::List(args)] => {
            let mut mapped = Vec::new();
            for item in list {
                let mut new_args = vec![item.clone()];
                new_args.extend(args.iter().cloned());
                mapped.push(apply(function, &new_args, env)?);
            }
            Ok(Node::List(mapped))
        }
        _ => invalid_arguments!(
            "map",
            rest,
            [
                "[Any(function), List(list)]",
                "[Any(function), List(list), List(args)]"
            ]
        ),
    }
}

//- (test "filter" (filter even? (quote (1 2 3 4))) (quote (2 4)))
//- (test "filter" (filter even? (quote ())) (quote ()))
//- (test "filter" (filter even? (quote (1))) (quote ()))
fn eval_filter(rest: &[Node], env: &mut Environment) -> Result<Node, String> {
    let rest = rest
        .iter()
        .map(|n| eval(n, env))
        .collect::<Result<Vec<_>, _>>()?;
    match &rest[..] {
        [function, Node::List(list)] => {
            let mut filtered = Vec::new();
            for item in list {
                if apply(function, &[item.clone()], env)? == Node::Bool(true) {
                    filtered.push(item.clone());
                }
            }
            Ok(Node::List(filtered))
        }
        _ => invalid_arguments!("filter", rest, ["[Any(function), List(list)]"]),
    }
}

//- (test "fold" (fold + 0 (quote (1 2 3))) 6)
//- (test "fold" (fold + 0 (quote ())) 0)
//- (test "fold" (fold + 0 (quote (1))) 1)
fn eval_fold(rest: &[Node], env: &mut Environment) -> Result<Node, String> {
    let rest = rest
        .iter()
        .map(|n| eval(n, env))
        .collect::<Result<Vec<_>, _>>()?;
    match &rest[..] {
        [function, initial_value, Node::List(list)] => {
            let mut result = initial_value.clone();
            for item in list {
                result = apply(function, &[result, item.clone()], env)?;
            }
            Ok(result)
        }
        _ => invalid_arguments!(
            "fold",
            rest,
            ["[Any(function), Any(initial_value), List(list)]"]
        ),
    }
}

fn eval_eval(rest: &[Node], env: &mut Environment) -> Result<Node, String> {
    let rest = rest
        .iter()
        .map(|n| eval(n, env))
        .collect::<Result<Vec<_>, _>>()?;
    match &rest[..] {
        [Node::List(nodes)] => {
            for node in nodes {
                eval(node, env)?;
            }
            Ok(Node::List(nodes.clone()))
        }
        _ => invalid_arguments!("eval", rest, ["[List(nodes)]"]),
    }
}

fn eval_pipe(rest: &[Node], env: &mut Environment) -> Result<Node, String> {
    let first = eval(&rest[0], env);
    rest.iter().skip(1).fold(first, |current, next| {
        if let Node::List(nodes) = next {
            let function = eval(&nodes[0], env)?;
            let mut arguments = vec![];
            for node in nodes.iter().skip(1) {
                arguments.push(eval(node, env)?);
            }
            arguments.push(current?);

            Ok(apply(&function, &arguments, env)?)
        } else {
            //panic!("Invalid pipe: {next:?}");
            invalid_arguments!("pipe", rest, ["[TODO]"])
        }
    })
}

fn eval_let(rest: &[Node], env: &mut Environment) -> Result<Node, String> {
    if rest.len() == 2 {
        let bindings = &rest[0];
        let body = &rest[1];

        if let Node::List(bindings_list) = bindings {
            let mut new_env = Environment::from_parent(env.clone());

            for binding in bindings_list {
                if let Node::List(binding_pair) = binding {
                    if binding_pair.len() == 2 {
                        let variable = &binding_pair[0];
                        let value = eval(&binding_pair[1], env)?;
                        match variable {
                            Node::Symbol(s) => new_env.insert(s, value),
                            _ => return Err(format!("Invalid variable in let: {variable:?}")),
                        }
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
            let mut new_env = Environment::new();

            for binding in bindings_list {
                if let Node::List(binding_pair) = binding {
                    if binding_pair.len() == 1 {
                        let variable = &binding_pair[0];
                        let value = eval(&binding_pair[0], env)?;
                        match variable {
                            Node::Symbol(s) => new_env.insert(s, value),
                            _ => {
                                return Err(format!(
                                    "Invalid variable in let-restricted: {variable:?}"
                                ));
                            }
                        }
                    } else if binding_pair.len() == 2 {
                        let variable = &binding_pair[0];
                        let value = eval(&binding_pair[1], env)?;
                        match variable {
                            Node::Symbol(s) => new_env.insert(s, value),
                            _ => {
                                return Err(format!(
                                    "Invalid variable in let-restricted: {variable:?}"
                                ));
                            }
                        }
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

fn eval_time_ms(rest: &[Node], env: &mut Environment) -> Result<Node, String> {
    if rest.len() == 1 {
        let start = std::time::Instant::now();
        eval(&rest[0], env)?;
        let duration = start.elapsed();
        let result = duration.as_millis();
        let Ok(n) = result.try_into() else {
            return Err("Time duration too large".to_string());
        };
        return Ok(Node::Number(n));
    }
    Err("Invalid arguments for time-ms".to_string())
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
                "begin" => eval_begin(rest, env)?,
                "define" => eval_define(rest, env)?,
                "undefine" => eval_undefine(rest, env)?,
                "defined?" => eval_is_defined(rest, env)?,
                "type?" => eval_get_type(rest, env)?,
                "print-env" => eval_print_env(rest, env)?,
                "lambda" => eval_lambda(rest)?,
                "map" => eval_map(rest, env)?,
                "filter" => eval_filter(rest, env)?,
                "fold" => eval_fold(rest, env)?,
                "eval" => eval_eval(rest, env)?,
                "|" | "pipe" => eval_pipe(rest, env)?,
                "let" => eval_let(rest, env)?,
                "let-restricted" => eval_let_restricted(rest, env)?,
                "time-ms" => eval_time_ms(rest, env)?,
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
        _ => {
            return Err(format!(
                "Invalid expression: {first:?}\nDid you mean to quote it?"
            ));
        }
    })
}

pub fn apply(function: &Node, arguments: &[Node], env: &mut Environment) -> Result<Node, String> {
    Ok(match function {
        Node::Function(f) => f(arguments)?,
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
                        let mut new_env = Environment::from_parent(env.clone());
                        for (param, arg) in params.iter().zip(arguments) {
                            match param {
                                Node::Symbol(s) => new_env.insert(s, arg.clone()),
                                _ => return Err(format!("Invalid parameter in lambda: {param:?}")),
                            }
                        }
                        return match eval(&nodes[2], &mut new_env) {
                            Ok(result) => Ok(result),
                            Err(e) => {
                                println!("Error evaluating lambda body: {function:?}");
                                Err(e)
                            }
                        };
                    }
                }
            }
            return Err(format!(
                "Function application not implemented: {function:?}"
            ));
        }
        _ => {
            println!("Invalid function: {function:?}");
            return Err(format!("Invalid function: {function:?}"));
        }
    })
}
