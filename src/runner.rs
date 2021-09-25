use std::ffi::{CStr, CString};
use std::io::{self, Read, Write};
use std::os::raw::c_char;
use std::process::exit;

use super::default_env::*;
use super::eval::parse_and_eval;
use linenoise::ffi::{linenoise, linenoiseHistoryAdd, linenoiseHistoryLoad, linenoiseHistorySave};

use std::env;
use std::fs::File;

fn add_to_history(x: &str, filename_cstr: &CStr) {
    unsafe {
        let c_string = CString::new(x).unwrap();
        let c_str = CStr::from_bytes_with_nul_unchecked(c_string.as_bytes_with_nul());
        linenoiseHistoryAdd(c_str.as_ptr() as *const i8);
        linenoiseHistorySave(filename_cstr.as_ptr() as *const c_char);
    }
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

fn handle_first_line(line: Option<String>, stack: &mut Vec<char>) -> Option<LineStatus> {
    // let line = linenoise::input(">>>");
    let mut s = String::new();
    if line.is_none() {
        return None;
    } else {
        s += &line.clone().unwrap();
        if s.is_empty() {
            return Some(LineStatus::ValidSExpr(s));
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

fn handle_non_first_lines(line: Option<String>, stack: &mut Vec<char>) -> Option<LineStatus> {
    // let line = linenoise::input("...");
    let mut s = String::new();
    if line.is_none() {
        return None;
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
    let filename = CString::new(".risp_history").unwrap();
    let filename_cstr =
        unsafe { CStr::from_bytes_with_nul_unchecked(filename.as_bytes_with_nul()) };
    unsafe {
        linenoiseHistoryLoad(filename_cstr.as_ptr() as *const c_char);
    }

    io::stdout().flush().unwrap();

    let mut stack: Vec<char> = Vec::new();

    let mut s = String::new();
    let mut ret_none = false;
    let line = linenoise::input(">>>");
    let first_line = handle_first_line(line, &mut stack);
    if first_line.is_none() {
        return None;
    }
    match first_line.unwrap() {
        LineStatus::EXIT => {
            add_to_history("exit", filename_cstr);
            exit(0);
        }
        LineStatus::GoNextLine(x) => {
            add_to_history(x.as_str(), filename_cstr);
            s += &x;
            s.push('\n');
        }
        LineStatus::ValidSExpr(x) | LineStatus::InvalidSExpr(x) => {
            add_to_history(x.as_str(), filename_cstr);
            return Some(x);
        }
    }

    loop {
        let line = linenoise::input("...");
        let line = handle_non_first_lines(line, &mut stack);
        if line.is_none() {
            ret_none = true;
            break;
        }
        match line.unwrap() {
            // doesnt make sense
            LineStatus::EXIT => unimplemented!(),
            LineStatus::GoNextLine(x) => {
                add_to_history(x.as_str(), filename_cstr);
                s += &x;
                s.push('\n');
            }
            LineStatus::ValidSExpr(x) | LineStatus::InvalidSExpr(x) => {
                add_to_history(x.as_str(), filename_cstr);
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

        let first_line = lines.last().map(|x| x.clone());
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
            let next_line = lines.last().map(|x| x.clone());
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
