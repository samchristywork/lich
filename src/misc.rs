use std::io::Write;

use crate::Environment;
use crate::Node;
use crate::Value;
use crate::control::fn_false;
use crate::control::fn_true;
use crate::evaluate_node;
use crate::expect_n_args;
use crate::handle_symbol;
use crate::process_file;

pub fn fn_map(args: &[Node], env: &mut Environment) -> Node {
    expect_n_args!(args, 2);

    let function = &args[0];
    let list = &args[1];

    let children = list
        .children
        .iter()
        .map(|item| Node {
            token: args[0].token.clone(),
            value: Value::List(),
            children: vec![function.clone(), item.clone()],
        })
        .collect::<Vec<_>>()
        .iter()
        .map(|child| evaluate_node(child, env))
        .collect::<Vec<_>>();

    Node {
        token: args[0].token.clone(),
        value: Value::List(),
        children,
    }
}

pub fn fn_filter(args: &[Node], env: &mut Environment) -> Node {
    expect_n_args!(args, 2);

    let function = &args[0];
    let list = &args[1];

    let mut children = Vec::new();
    if env.get(&function.token.value.clone()).is_some() {
        // Not intrinsic
        for item in evaluate_node(list, env).children {
            let lambda = handle_symbol(function, &item.children, env);

            let lambda_params = &lambda.children[0];
            let lambda_body = &lambda.children[1..];

            let mut new_env = env.clone();
            for param in &lambda_params.children {
                new_env.set(param.token.value.clone(), item.clone());
            }

            for child in lambda_body {
                let ret = evaluate_node(child, &mut new_env);
                if ret.token.value == "true" {
                    children.push(item.clone());
                }
            }
        }
    } else {
        // Intrinsic
        for item in evaluate_node(list, env).children {
            let ret = evaluate_node(
                &Node {
                    token: function.token.clone(),
                    value: Value::List(),
                    children: vec![function.clone(), item.clone()],
                },
                env,
            );
            if ret.token.value == "true" {
                children.push(item.clone());
            }
        }
    }

    Node {
        token: args[0].token.clone(),
        value: Value::List(),
        children,
    }
}

// TODO: This function should take an optional argument
pub fn fn_print_env(_: &[Node], env: &Environment) -> Node {
    println!("Environment:");
    let red = "\x1b[31m";
    let normal = "\x1b[0m";
    print!("{red}");
    env.variables.iter().for_each(|(key, value)| {
        println!("  {}: {}", key, value.string());
    });
    println!("{normal}");

    fn_true(&[])
}

pub fn fn_get_environment_variable(args: &[Node], env: &mut Environment) -> Node {
    expect_n_args!(args, 1);

    if let Value::Text(var_name) = evaluate_node(&args[0], env).value {
        let value = std::env::var(var_name).unwrap_or_else(|_| String::new());
        return Node {
            token: args[0].token.clone(),
            value: Value::Text(value),
            children: Vec::new(),
        };
    }

    panic!("Invalid argument for get_environment_variable function");
}

pub fn fn_is_even(args: &[Node], env: &mut Environment) -> Node {
    expect_n_args!(args, 1);

    if let Value::Number(n) = evaluate_node(&args[0], env).value {
        if n % 2 == 0 {
            return fn_true(&[]);
        }
        return fn_false(&[]);
    }

    panic!("Invalid argument for is_even function");
}

pub fn fn_is_odd(args: &[Node], env: &mut Environment) -> Node {
    expect_n_args!(args, 1);

    if let Value::Number(n) = evaluate_node(&args[0], env).value {
        if n % 2 != 0 {
            return fn_true(&[]);
        }
        return fn_false(&[]);
    }

    panic!("Invalid argument for is_odd function");
}

pub fn fn_url_encode(args: &[Node], env: &mut Environment) -> Node {
    expect_n_args!(args, 1);

    if let Value::Text(input) = evaluate_node(&args[0], env).value {
        let encoded = url::form_urlencoded::byte_serialize(input.as_bytes()).collect::<String>();

        return Node {
            token: args[0].token.clone(),
            value: Value::Text(encoded),
            children: Vec::new(),
        };
    }

    panic!("Invalid argument for url_encode function");
}

pub fn fn_url_decode(args: &[Node], env: &mut Environment) -> Node {
    expect_n_args!(args, 1);

    if let Value::Text(input) = evaluate_node(&args[0], env).value {
        let decoded = url::form_urlencoded::parse(input.as_bytes())
            .map(|(key, value)| format!("{key}={value}"))
            .collect::<Vec<_>>()
            .join("&");

        return Node {
            token: args[0].token.clone(),
            value: Value::Text(decoded),
            children: Vec::new(),
        };
    }

    panic!("Invalid argument for url_decode function");
}

pub fn fn_load(args: &[Node], env: &mut Environment) -> Node {
    for arg in args {
        if let Value::Text(filename) = evaluate_node(arg, env).value {
            process_file(&filename, env, false, false);
        } else {
            panic!("Invalid argument for load function");
        }
    }

    fn_true(&[])
}

pub fn fn_sleep(args: &[Node], env: &mut Environment) -> Node {
    expect_n_args!(args, 1);

    if let Value::Number(duration) = evaluate_node(&args[0], env).value {
        std::thread::sleep(std::time::Duration::from_secs(
            duration.try_into().expect("Invalid duration"),
        ));
        return fn_true(&[]);
    }

    panic!("Invalid argument for sleep function");
}

pub fn fn_sleep_ms(args: &[Node], env: &mut Environment) -> Node {
    expect_n_args!(args, 1);

    if let Value::Number(duration) = evaluate_node(&args[0], env).value {
        std::thread::sleep(std::time::Duration::from_millis(
            duration.try_into().expect("Invalid duration"),
        ));
        return fn_true(&[]);
    }

    panic!("Invalid argument for sleep_ms function");
}

pub fn fn_time_ms(args: &[Node], env: &mut Environment) -> Node {
    let start = std::time::Instant::now();
    for arg in args {
        evaluate_node(arg, env);
    }

    Node {
        token: args[0].token.clone(),
        value: Value::Number(
            start
                .elapsed()
                .as_millis()
                .try_into()
                .expect("Invalid duration"),
        ),
        children: Vec::new(),
    }
}

pub fn fn_system(args: &[Node], env: &mut Environment) -> Node {
    assert!(!args.is_empty(), "No command provided");

    if let Value::Text(command) = evaluate_node(&args[0], env).value {
        let arguments = if args.len() > 1 {
            args[1].clone()
        } else {
            Node {
                token: args[0].token.clone(),
                value: Value::List(),
                children: Vec::new(),
            }
        };

        let stdin_string = if args.len() > 2 {
            if let Value::Text(string) = evaluate_node(&args[2], env).value {
                string
            } else {
                panic!("Invalid argument for system function");
            }
        } else {
            String::new()
        };

        let mut child = std::process::Command::new(command)
            .args(arguments.children.iter().map(|arg| {
                if let Value::Text(ref s) = arg.value {
                    s.clone()
                } else {
                    panic!("Invalid argument for system function");
                }
            }))
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .expect("Failed to start process");

        {
            let mut stdin = child.stdin.take().expect("Failed to open stdin");
            stdin
                .write_all(stdin_string.as_bytes())
                .expect("Failed to write to stdin");
        }

        let output = child
            .wait_with_output()
            .expect("Failed to read stdout/stderr");

        return Node {
            token: args[0].token.clone(),
            value: Value::List(),
            children: vec![
                Node {
                    token: args[0].token.clone(),
                    value: Value::Number(i64::from(output.status.code().unwrap_or(-1))),
                    children: Vec::new(),
                },
                Node {
                    token: args[0].token.clone(),
                    value: Value::Text(String::from_utf8_lossy(&output.stdout).to_string()),
                    children: Vec::new(),
                },
                Node {
                    token: args[0].token.clone(),
                    value: Value::Text(String::from_utf8_lossy(&output.stderr).to_string()),
                    children: Vec::new(),
                },
            ],
        };
    }

    panic!("Invalid argument for system function");
}

use crate::operator::test_equal;
pub fn fn_contains(args: &[Node], env: &mut Environment) -> Node {
    expect_n_args!(args, 2);

    if evaluate_node(&args[1], env).value == Value::List() {
        for item in evaluate_node(&args[1], env).children {
            if test_equal(&item, &args[0]) {
                return fn_true(&[]);
            }
        }
    }

    panic!("Invalid arguments for contains function");
}
