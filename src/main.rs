use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("you need to provide only a single argument to the program");
        process::exit(1);
    }

    let script_path = &args[1];

    let lines = read_to_string(script_path);
    let lines = match lines {
        Ok(v) => v,
        Err(_) => {
            eprintln!("failed to open file");
            process::exit(1);
        }
    };
}

enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Unknown,
}

enum LineState {
    NewAssign,
    ApplyOperator,
    DropVar,
    Unknown,
}

fn run_script(script: String) -> Option<bool> {
    let mut vars = HashMap::<String, i32>::new();

    for (line_idx, line) in script.lines().enumerate() {
        let mut op: Operator = Operator::Unknown;
        let mut current_line_state: LineState = LineState::Unknown;
        let mut var_being_assigned_to = String::new();

        for (idx, token) in line.split_whitespace().enumerate() {
            match idx {
                0 => {
                    match token {
                        "let" => {
                            current_line_state = LineState::NewAssign;
                        }
                        v => {
                            if !vars.contains_key(v) {
                                eprintln!("unknown variable on {}: {}", line_idx + 1, v);
                                process::exit(1);
                            } else {
                                current_line_state = LineState::ApplyOperator;
                                var_being_assigned_to = v.to_string();
                            }
                        }
                    };
                }
                1 => {
                    match current_line_state {
                        LineState::NewAssign => {}
                        LineState::ApplyOperator => {}
                        LineState::Unknown => {
                            eprintln!("syntax error at line {}: {}", line_idx + 1, token);
                            process::exit(1);
                        }
                        _ => todo!(),
                    };
                }
                _ => {}
            };
        }
    }

    Some(true)
}
