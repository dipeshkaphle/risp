use std::io::{self, Write};

use std::process::exit;

use super::default_env::*;
use super::eval::parse_and_eval;
use rustyline::error::ReadlineError;
use rustyline::Editor;

const RISP_HISTORY_FILE: &'static str = ".risp_history";

fn add_to_history_rl(x: &str, rl: &mut Editor<()>) {
    rl.add_history_entry(x);
    rl.save_history(RISP_HISTORY_FILE).unwrap();
}

enum ParensStatus {
    MATCHING,
    LeftParensMore,
    RightParensMore,
}

enum LineStatus {
    EXIT,
    GoNextLine(String),
    InvalidSExpr(String),
    ValidSExpr(String),
}

fn line_status(s: &str, stack: &mut Vec<char>) -> ParensStatus {
    for c in s.chars() {
        match c {
            '(' => {
                stack.push('(');
            }
            ')' => {
                if stack.is_empty() {
                    return ParensStatus::RightParensMore;
                } else {
                    stack.pop();
                }
            }
            _ => continue,
        }
    }
    if stack.is_empty() {
        ParensStatus::MATCHING
    } else {
        ParensStatus::LeftParensMore
    }
}

fn handle_first_line(
    line: Result<String, ReadlineError>,
    stack: &mut Vec<char>,
) -> Option<LineStatus> {
    let mut s = String::new();
    match line {
        Ok(ln) => {
            s += &ln;
            if s.is_empty() {
                return Some(LineStatus::ValidSExpr(s));
            }
        }
        Err(ReadlineError::Eof) => {
            println!("Exiting!!!");
            exit(0);
        }
        Err(_) => {
            return None;
        }
    }
    if s.as_str() == "exit" {
        return Some(LineStatus::EXIT);
    } else {
        let status = line_status(s.as_str(), stack);
        match status {
            ParensStatus::MATCHING => {
                return Some(LineStatus::ValidSExpr(s));
            }
            ParensStatus::LeftParensMore => {
                return Some(LineStatus::GoNextLine(s));
            }
            ParensStatus::RightParensMore => {
                return Some(LineStatus::InvalidSExpr(s));
            }
        }
    }
}

fn handle_non_first_lines(
    line: Result<String, ReadlineError>,
    stack: &mut Vec<char>,
) -> Option<LineStatus> {
    let mut s = String::new();
    match line {
        Err(ReadlineError::Interrupted) => return None,
        Err(ReadlineError::Eof) => {
            println!("Exiting");
            exit(0);
        }
        _ => {}
    }
    s += &line.unwrap();
    let status = line_status(s.as_str(), stack);
    match status {
        ParensStatus::MATCHING => {
            return Some(LineStatus::ValidSExpr(s));
        }
        ParensStatus::LeftParensMore => {
            return Some(LineStatus::GoNextLine(s));
        }
        ParensStatus::RightParensMore => {
            return Some(LineStatus::InvalidSExpr(s));
        }
    }
}

pub fn repl() -> Option<String> {
    let mut rl = Editor::<()>::new();
    if rl.load_history(RISP_HISTORY_FILE).is_err() {
        println!("No history found");
    }

    io::stdout().flush().unwrap();

    let mut stack: Vec<char> = Vec::new();

    let mut s = String::new();
    let mut ret_none = false;
    let line = rl.readline(">>>");
    let first_line = handle_first_line(line, &mut stack);
    if first_line.is_none() {
        return None;
    }
    match first_line.unwrap() {
        LineStatus::EXIT => {
            add_to_history_rl("exit", &mut rl);
            println!("Exiting!!!");
            exit(0);
        }
        LineStatus::GoNextLine(x) => {
            add_to_history_rl(x.as_str(), &mut rl);
            s += &x;
            s.push('\n');
        }
        LineStatus::ValidSExpr(x) | LineStatus::InvalidSExpr(x) => {
            add_to_history_rl(x.as_str(), &mut rl);
            return Some(x);
        }
    }

    loop {
        let line = rl.readline("...");
        let line = handle_non_first_lines(line, &mut stack);
        if line.is_none() {
            ret_none = true;
            break;
        }
        match line.unwrap() {
            // doesnt make sense
            LineStatus::EXIT => unimplemented!(),
            LineStatus::GoNextLine(x) => {
                add_to_history_rl(x.as_str(), &mut rl);
                s += &x;
                s.push('\n');
            }
            LineStatus::ValidSExpr(x) | LineStatus::InvalidSExpr(x) => {
                add_to_history_rl(x.as_str(), &mut rl);
                s += &x;
                break;
            }
        }
    }
    if !ret_none {
        Some(s.trim().to_owned())
    } else {
        None
    }
}

pub fn run_from_source_code(program: String) {
    let mut def_env = default_env();
    let mut lines: Vec<String> = program.lines().map(|x| x.to_string()).rev().collect();
    let mut line_num = -1;

    while !lines.is_empty() {
        line_num += 1;
        let mut stack: Vec<char> = Vec::new();
        let mut s = String::new();

        let first_line = lines
            .last()
            .map(|x| x.clone())
            .ok_or(ReadlineError::Interrupted);
        lines.pop();
        let first_line = handle_first_line(first_line, &mut stack);
        match first_line.unwrap() {
            LineStatus::EXIT => {
                exit(0);
            }
            LineStatus::GoNextLine(x) => {
                s += &x;
                s.push('\n');
            }
            LineStatus::ValidSExpr(x) | LineStatus::InvalidSExpr(x) => match x.as_str() {
                "" => continue,
                _ => {
                    parse_and_eval(x, &mut def_env).unwrap();
                    continue;
                }
            },
        }

        loop {
            let next_line = lines
                .last()
                .map(|x| x.clone())
                .ok_or(ReadlineError::Interrupted);
            line_num += 1;
            let next_line = handle_non_first_lines(next_line, &mut stack);
            if next_line.is_none() {
                panic!("Incomplete expression at line {}", line_num);
            }
            lines.pop();
            match next_line.unwrap() {
                // doesnt make sense
                LineStatus::EXIT => unimplemented!(),
                LineStatus::GoNextLine(x) => {
                    s += &x;
                    s.push('\n');
                }
                LineStatus::ValidSExpr(x) | LineStatus::InvalidSExpr(x) => {
                    s += &x;
                    parse_and_eval(s, &mut &mut def_env).unwrap();
                    break;
                }
            }
        }
    }
}
