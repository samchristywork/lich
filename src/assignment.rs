use crate::Environment;
use crate::Node;
use crate::Value;
use crate::control::fn_true;
use crate::evaluate_node;
use crate::expect_n_args;

pub fn fn_def(args: &[Node], env: &mut Environment) -> Node {
    expect_n_args!(args, 2);

    if let Value::Symbol(name) = evaluate_node(&args[0], env).value {
        let value = evaluate_node(&args[1], env);
        env.set(name, value);
        fn_true(&[])
    } else {
        panic!("Invalid argument for def function");
    }
}

fn look_for_bang(children: &[Node]) -> bool {
    children.iter().any(|child| {
        if let Value::Symbol(ref s) = child.value {
            s.ends_with('!')
        } else {
            look_for_bang(&child.children)
        }
    })
}

//fn list_dependencies(name: &str, children: &[Node], env: &mut Environment) {
//    for child in children {
//        if child.token.kind == TokenKind::Symbol {
//            // TODO: Find a way to differentiate between a function and variable
//            println!("{} -> {}", name, child.token.value);
//        } else {
//            list_dependencies(name, &child.children, env);
//        }
//    }
//}

pub fn fn_func(args: &[Node], env: &mut Environment) -> Node {
    ////println!("Creating new function: {}", name);
    ////println!("Dependencies:");
    ////list_dependencies(name, &children, env);

    if let Value::Symbol(name) = evaluate_node(&args[0], env).value {
        let params = args[1].clone();
        let body = args[2..].to_vec();
        let mut children = vec![params];
        children.extend(body);

        assert_eq!(
            name.ends_with('!'),
            look_for_bang(&children),
            "Function name and child identifiers must match the bang convention",
        );

        let lambda = Node {
            token: args[0].token.clone(),
            value: Value::Lambda(),
            children,
        };

        env.set(name, lambda);

        return fn_true(&[]);
    }

    panic!("Invalid argument for func function");
}

pub fn fn_set(args: &[Node], env: &mut Environment) -> Node {
    expect_n_args!(args, 3);

    if let Value::Symbol(name) = evaluate_node(&args[0], env).value {
        let value = &args[1];
        let body = &args[2];

        let mut new_env = env.clone();
        new_env.set(name, value.clone());
        for child in &body.children {
            evaluate_node(child, &mut new_env);
        }
        return fn_true(&[]);
    }

    panic!("Invalid argument for set function");
}

pub fn fn_lambda(args: &[Node]) -> Node {
    expect_n_args!(args, 2);

    let params = args[0].clone();
    let body = args[1].clone();

    Node {
        token: args[0].token.clone(),
        value: Value::Lambda(),
        children: vec![params, body],
    }
}
