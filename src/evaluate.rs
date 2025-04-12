use crate::Environment;
use crate::Node;
use crate::Value;
use crate::assignment;
use crate::control;
use crate::identity;
use crate::io;
use crate::list;
use crate::logic;
use crate::misc;
use crate::operator;
use crate::string;
use crate::log_debug;
use crate::log_message;

#[macro_export]
macro_rules! evaluate_args {
    ($args:expr, $env:expr) => {
        $args
            .iter()
            .map(|arg| evaluate_node(arg, $env))
            .collect::<Vec<_>>()
    };
}

#[macro_export]
macro_rules! expect_n_args {
    ($args:expr, $n:expr) => {
        if $args.len() != $n {
            panic!("Expected {} arguments, but got {}", $n, $args.len());
        }
    };
}

pub fn evaluate_node(node: &Node, env: &mut Environment) -> Node {
    log_debug!("Evaluating node: {node}");

    match &node.value {
        Value::List() => {
            let function = evaluate_node(&node.children[0], env);
            apply_function(&function, &node.children[1..], env)
        }
        Value::Symbol(s) => env
            .get(s)
            .map_or_else(|| node.clone(), std::clone::Clone::clone),
        Value::Module() => {
            let mut result = node.clone();
            for child in &node.children {
                result = evaluate_node(child, env);
            }
            result
        }
        _ => node.clone(),
    }
}

pub fn handle_symbol(function: &Node, args: &[Node], env: &mut Environment) -> Node {
    log_debug!("Handling symbol: {function}");

    let Value::Symbol(name) = &function.value else {
        panic!("Expected a symbol, found: {:?}", function.value);
    };

    match name.as_str() {
        // Operators
        "+" => operator::fn_add(args, env),
        "-" => operator::fn_sub(args, env),
        "*" => operator::fn_mul(args, env),
        "=" => operator::fn_equal(args, env),
        "<" => operator::fn_less_than(args, env),
        ">" => operator::fn_greater_than(args, env),
        "^" => operator::fn_pow(args, env),
        "%" => operator::fn_mod(args, env),
        "++" => operator::fn_inc(args, env),
        "--" => operator::fn_dec(args, env),
        "max" => operator::fn_max(args, env),
        "min" => operator::fn_min(args, env),

        // Types
        "text?" => identity::fn_is_text(args, env),
        "number?" => identity::fn_is_number(args, env),
        "symbol?" => identity::fn_is_symbol(args, env),
        "list?" => identity::fn_is_lparen(args, env),
        "lambda?" => identity::fn_is_lambda(args, env),
        "atom?" => identity::fn_is_atom(args, env),

        // Logical
        "&&" => logic::fn_and(args, env),
        "||" => logic::fn_or(args, env),

        // Control flow
        "true" => control::fn_true(args),
        "false" => control::fn_false(args),
        "if" => control::fn_if(args, env),
        "cond" => control::fn_cond(args, env),
        "repeat" => control::fn_repeat(args, env),
        "loop" => control::fn_loop(args, env),
        "|" => control::fn_pipeline(args, env),
        "rev|" => control::fn_reverse_pipeline(args, env),
        "{}" => control::fn_block(args, env),
        "exit" => control::fn_exit(args, env),

        // I/O
        "write!" | "!" => io::fn_write(args, env),
        "?" => io::fn_debug_write(args, env),
        "write-stderr!" => io::fn_write_stderr(args, env),
        "write-file!" => io::fn_write_file(args, env),
        "read-line!" => io::fn_read_line(args),
        "read-file!" => io::fn_read_file(args, env),

        // Strings
        "join" => string::fn_join(args, env),
        "split" => string::fn_split(args, env),
        "lines" => string::fn_lines(args, env),
        "strlen" => string::fn_strlen(args, env),
        "empty-string?" => string::fn_empty_string(args, env),

        // Assignment
        "func!" => assignment::fn_func(args, env),
        "set" => assignment::fn_set(args, env),
        "lambda" | "λ" => assignment::fn_lambda(args),

        // Lists
        "list" | "'" => list::fn_list(args, env),
        "head" => list::fn_head(args, env),
        "last" => list::fn_last(args, env),
        "tail" => list::fn_tail(args, env),
        "length" => list::fn_length(args, env),
        "reverse" => list::fn_reverse(args, env),

        // Miscellaneous
        "map" => misc::fn_map(args, env),
        "filter" => misc::fn_filter(args, env),
        "print-env!" => misc::fn_print_env(args, env),
        "get-environment-variable!" => misc::fn_get_environment_variable(args, env),
        "even?" => misc::fn_is_even(args, env),
        "odd?" => misc::fn_is_odd(args, env),
        "url-encode" => misc::fn_url_encode(args, env),
        "url-decode" => misc::fn_url_decode(args, env),
        "load!" => misc::fn_load(args, env),
        "sleep" => misc::fn_sleep(args, env),
        "sleep-ms" => misc::fn_sleep_ms(args, env),
        "time-ms" => misc::fn_time_ms(args, env),
        "system!" => misc::fn_system(args, env),
        "contains" => misc::fn_contains(args, env),
        _ => env.get(name).map_or_else(
            || panic!("Unknown function: {name}"),
            std::clone::Clone::clone,
        ),
    }
}

pub fn apply_function(function: &Node, args: &[Node], env: &mut Environment) -> Node {
    log_debug!("Applying function: {function}");

    match function.value {
        Value::Symbol(_) => handle_symbol(function, args, env),
        Value::Lambda() => {
            let params = &function.children[0];
            let body = &function.children[1..];

            assert_eq!(params.children.len(), args.len(), "Argument count mismatch");

            let mut new_env = env.clone();
            for (param, arg) in params.children.iter().zip(args) {
                match param.value {
                    Value::Symbol(ref s) => new_env.set(s.clone(), evaluate_node(arg, env)),
                    _ => panic!("Expected a symbol, found: {:?}", param.value),
                }
            }

            let mut return_value = control::fn_true(&[]);
            for child in body {
                return_value = evaluate_node(child, &mut new_env);
            }

            return_value
        }
        Value::List() => {
            apply_function(&function.children[0].clone(), &function.children[1..], env)
        }
        _ => panic!("Invalid function application: {:?}", function.value),
    }
}
