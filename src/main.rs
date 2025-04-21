pub mod arithmetic;
pub mod compare;
pub mod conversion;
pub mod environment;
pub mod error;
pub mod eval;
pub mod io;
pub mod list;
pub mod node;
pub mod parse;
pub mod random;
pub mod regex;
pub mod sequence;
pub mod string;
pub mod system;
pub mod terminal;
pub mod time;
pub mod tree;

use crate::environment::Environment;
use crate::eval::eval;
use crate::node::Node;
use crate::parse::parse;
use std::io::BufRead;
use std::io::Write;

const RED: &str = "\x1b[31m";
const GREY: &str = "\x1b[90m";
const NORMAL: &str = "\x1b[0m";

fn evaluate_version(env: &mut Environment) -> Result<Node, String> {
    let input = "(version)";
    let expressions = parse(input).map_err(|e| format!("Failed to parse input: {e}"))?;

    eval(&expressions[0], env)
}

fn print_version(env: &mut Environment) {
    match evaluate_version(env) {
        Ok(node) => {
            println!("{node}");
        }
        Err(e) => {
            eprintln!("Failed to evaluate version: {e}");
        }
    }
}

fn repl(env: &mut Environment, server: bool) -> Result<(), String> {
    if server {
        let socket_string = "localhost:8080";

        let listener = std::net::TcpListener::bind(socket_string)
            .map_err(|e| format!("Failed to bind to socket: {e}"))?;

        println!("Server started on {socket_string}");
        loop {
            let (mut stream, addr) = listener
                .accept()
                .map_err(|e| format!("Failed to accept connection: {e}"))?;
            println!("Client connected: {addr}");
            let mut reader = std::io::BufReader::new(
                stream
                    .try_clone()
                    .map_err(|e| format!("Failed to clone stream: {e}"))?,
            );

            let mut input = String::new();
            while reader
                .read_line(&mut input)
                .map_err(|e| format!("Failed to read line: {e}"))?
                > 0
            {
                let input_string = input.trim().to_string();
                match parse(&input_string) {
                    Ok(expressions) => {
                        for expression in expressions {
                            let result = eval(&expression, env);
                            eprintln!("{result:?}");

                            // Send the result back to the client
                            let response = format!("{result:?}\n");
                            stream
                                .write_all(response.as_bytes())
                                .map_err(|e| format!("Failed to write to stream: {e}"))?;
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to parse input: {input_string}\n{e}");
                    }
                }
                input.clear();
            }
        }
    } else {
        match evaluate_version(env) {
            Ok(node) => {
                println!("{node}");
            }
            Err(e) => {
                eprintln!("Failed to evaluate version: {e}");
            }
        }

        loop {
            print!("lich> ");
            std::io::stdout()
                .flush()
                .map_err(|e| format!("Failed to flush stdout: {e}"))?;

            let mut input = String::new();
            std::io::stdin()
                .read_line(&mut input)
                .map_err(|e| format!("Failed to read line: {e}"))?;
            if input.is_empty() {
                println!();
                break;
            }
            let input_string = input.trim().to_string();
            if input_string == "exit" {
                break;
            }

            if let Ok(expressions) = parse(&input_string) {
                for expression in expressions {
                    let result = eval(&expression, env);
                    match result {
                        Ok(node) => {
                            println!("{GREY}{node}{NORMAL}");
                        }
                        Err(e) => {
                            eprintln!("{RED}{e}{NORMAL}");
                        }
                    }
                }
            } else {
                eprintln!("Failed to parse input: {input_string}");
            }
        }
    }

    Ok(())
}

fn process_files(positional_args: &Vec<&String>, env: &mut Environment, verbose: bool) {
    for arg in positional_args {
        let input_string = std::fs::read_to_string(arg).expect("Failed to read input file");

        match parse(&input_string) {
            Ok(expressions) => {
                for expression in expressions {
                    if verbose {
                        eprintln!("{GREY}{expression}{NORMAL}");
                        eprintln!("Result: {:?}", eval(&expression, env));
                    } else {
                        let result = eval(&expression, env);
                        if let Err(e) = result {
                            eprintln!("{RED}{e}{NORMAL}");
                            return;
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to parse input file: {arg}\n{e}");
                return;
            }
        }
    }
}

fn create_environment(env: &mut Environment) {
    env.insert(
        "args",
        Node::List(std::env::args().skip(1).map(Node::Text).collect()),
    );

    // Arithmetic
    env.add_function("+", arithmetic::fn_add);
    env.add_function("-", arithmetic::fn_sub);
    env.add_function("*", arithmetic::fn_mult);
    env.add_function("/", arithmetic::fn_div);
    env.add_function("even?", arithmetic::fn_is_even);
    env.add_function("odd?", arithmetic::fn_is_odd);
    env.add_function("inc", arithmetic::fn_inc);
    env.add_function("dec", arithmetic::fn_dec);
    env.add_function("abs", arithmetic::fn_abs);
    env.add_function("pow", arithmetic::fn_pow);
    env.add_function("negate", arithmetic::fn_negate);

    // Comparison
    env.add_function("=", compare::fn_eq);
    env.add_function("<", compare::fn_less_than);
    env.add_function(">", compare::fn_greater_than);
    env.add_function("<=", compare::fn_less_than_or_equal);
    env.add_function(">=", compare::fn_greater_than_or_equal);
    env.add_function("not", compare::fn_not);
    env.add_function("and", compare::fn_and);
    env.add_function("or", compare::fn_or);

    // Conversion
    env.add_function("number->string", conversion::fn_number_to_string);
    env.add_function("string->number", conversion::fn_string_to_number);
    env.add_function("string->list", conversion::fn_string_to_list);
    env.add_function("list->string", conversion::fn_list_to_string);
    env.add_function("string->symbol", conversion::fn_string_to_symbol);
    env.add_function("symbol->string", conversion::fn_symbol_to_string);
    env.add_function("string->boolean", conversion::fn_string_to_boolean);
    env.add_function("boolean->string", conversion::fn_boolean_to_string);
    env.add_function("time->string", conversion::fn_time_to_string);
    env.add_function("time->number", conversion::fn_time_to_number);
    env.add_function("number->float", conversion::fn_number_to_float);

    // I/O
    env.add_function("format", io::fn_format);
    env.add_function("write", io::fn_write);
    env.add_function("write-line", io::fn_write_line);
    env.add_function("write-file", io::fn_write_file);
    env.add_function("read-line", io::fn_read_line);
    env.add_function("read-file", io::fn_read_file);
    env.add_function("ls", io::fn_ls);
    env.add_function("directory?", io::fn_is_directory);

    // List Manipulation
    env.add_function("car", list::fn_car);
    env.add_function("cdr", list::fn_cdr);
    env.add_function("cons", list::fn_cons);
    env.add_function("length", list::fn_length);
    env.add_function("null?", list::fn_is_null);
    env.add_function("list", list::fn_list);
    env.add_function("last", list::fn_last);
    env.add_function("nth", list::fn_nth);

    // Parsing
    env.add_function("tokenize", parse::fn_tokenize);
    env.add_function("parse", parse::fn_parse);

    // Random
    env.add_function("random-number", random::fn_random_number);

    // Regex
    env.add_function("regex", regex::fn_regex);
    env.add_function("regex-match", regex::fn_regex_match);
    env.add_function("regex-replace", regex::fn_regex_replace);
    env.add_function("regex-split", regex::fn_regex_split);

    // Sequence Manipulation
    env.add_function("zip", sequence::fn_zip);
    env.add_function("range", sequence::fn_range);

    // String Manipulation
    env.add_function("concat", string::fn_concat);
    env.add_function("split", string::fn_split);
    env.add_function("strip", string::fn_strip);
    env.add_function("join", string::fn_join);
    env.add_function("index-of", string::fn_index_of);
    env.add_function("substring", string::fn_substring);
    env.add_function("replace", string::fn_replace);
    env.add_function("upper", string::fn_upper);
    env.add_function("lower", string::fn_lower);
    env.add_function("starts-with?", string::fn_starts_with);
    env.add_function("ends-with?", string::fn_ends_with);

    // System
    env.add_function("system", system::fn_system);
    env.add_function("version", system::fn_version);
    env.add_function("exit", system::fn_exit);

    // Terminal
    env.add_function("clear", terminal::fn_clear);
    env.add_function("alternate-screen", terminal::fn_alternate_screen);
    env.add_function("normal-screen", terminal::fn_alternate_screen);
    env.add_function("fg", terminal::fn_fg);
    env.add_function("bg", terminal::fn_bg);
    env.add_function("set-cursor-pos", terminal::fn_set_cursor_pos);

    // Time
    env.add_function("time", time::fn_time);
    env.add_function("now", time::fn_now);
    env.add_function("add-days", time::fn_add_days);
    env.add_function("add-hours", time::fn_add_hours);
    env.add_function("add-minutes", time::fn_add_minutes);
    env.add_function("add-seconds", time::fn_add_seconds);

    // Tree Manipulation
    env.add_function("leaves", tree::fn_leaves);
    env.add_function("depth", tree::fn_depth);
    env.add_function("format-tree", tree::fn_format_tree);
}

macro_rules! get_flag {
    ($args:expr, $flag:expr, $long_flag:expr) => {
        $args.iter().any(|arg| *arg == $flag || *arg == $long_flag)
    };
}

fn usage() {
    println!("Usage: lich [options] [file1 file2 ...]");
    println!("Options:");
    println!("  -h, --help       Show this help message");
    println!("  -v, --verbose    Enable verbose mode");
    println!("  -V, --version    Show version information");
    println!("  -s, --server     Start in server mode");
}

fn main() {
    let mut env = Environment::new();

    create_environment(&mut env);

    let args = std::env::args().collect::<Vec<_>>();
    let flag_args = args
        .iter()
        .filter(|arg| arg.starts_with('-'))
        .collect::<Vec<_>>();
    let positional_args = args
        .iter()
        .filter(|arg| !arg.starts_with('-'))
        .collect::<Vec<_>>();

    let server_flag = get_flag!(flag_args, "-s", "--server");
    let help_flag = get_flag!(flag_args, "-h", "--help");
    let verbose_flag = get_flag!(flag_args, "-v", "--verbose");
    let version_flag = get_flag!(flag_args, "-V", "--version");

    if version_flag {
        print_version(&mut env);
        return;
    }

    if help_flag {
        print_version(&mut env);
        println!();
        usage();
        return;
    }

    if positional_args.len() > 1 {
        process_files(
            &positional_args.into_iter().skip(1).collect(),
            &mut env,
            verbose_flag,
        );
    } else {
        repl(&mut env, server_flag).expect("Failed to start REPL");
    }
}
