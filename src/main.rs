use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;
use std::process;

mod trig;

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

    let result = run_script(lines);

    match result {
        None => {
            eprintln!("failed to run script");
            process::exit(1);
        }
        Some(_) => {
            eprintln!("execution finished successfully");
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
    Sin,
    DropVar,
    Unknown,
}

fn run_script(script: String) -> Option<bool> {
    let mut vars = HashMap::<String, f64>::new();

    for (line_idx, line) in script.lines().enumerate() {
        let _op: Operator = Operator::Unknown;
        let mut current_line_state: LineState = LineState::Unknown;
        let mut var_being_assigned_to = String::new();

        for (idx, token) in line.split_whitespace().enumerate() {
            match idx {
                0 => {
                    match token {
                        "let" => {
                            current_line_state = LineState::NewAssign;
                        }
                        "vars" => {
                            println!("currently assigned vars:");
                            for (key, val) in vars.iter() {
                                println!("{} = {}", key, val);
                            }
                            break;
                        }
                        "sin" => {
                            current_line_state = LineState::Sin;
                        }
                        v => {
                            if !vars.contains_key(v) {
                                eprintln!("unknown variable on {}: {}", line_idx + 1, v);
                                return None;
                            } else {
                                current_line_state = LineState::ApplyOperator;
                                var_being_assigned_to = v.to_string();
                            }
                        }
                    };
                }
                1 => {
                    match current_line_state {
                        LineState::NewAssign => {
                            // This is where we read the variable name.
                            var_being_assigned_to = token.to_string();
                        }
                        LineState::Unknown => {
                            eprintln!("syntax error at line {}: {}", line_idx + 1, token);
                            return None;
                        }
                        LineState::Sin => match token.parse::<f64>() {
                            Ok(f) => {
                                println!("{}", trig::sine(f));
                            }
                            Err(_) => {
                                eprintln!("invalid f64 number: {}", token);
                                return None;
                            }
                        },
                        _ => todo!(),
                    };
                }
                2 => {
                    match current_line_state {
                        LineState::NewAssign => {
                            if token != "=" {
                                eprintln!("equals sign required at position 2 for assignment");
                                return None;
                            }
                        }
                        _ => todo!(),
                    };
                }
                3 => {
                    match current_line_state {
                        LineState::NewAssign => match token.parse::<f64>() {
                            Ok(f) => {
                                vars.insert(var_being_assigned_to.clone(), f);
                            }
                            Err(_) => {
                                eprintln!("invalid f64 number: {}", token);
                                return None;
                            }
                        },
                        _ => todo!(),
                    };
                }
                _ => {}
            };
        }
    }

    Some(true)
}
