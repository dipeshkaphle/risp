use std::ffi::{CStr, CString};
use std::io::{self, Write};
use std::os::raw::c_char;
use std::process::exit;

use linenoise::ffi::{linenoiseHistoryAdd, linenoiseHistoryLoad, linenoiseHistorySave};
use risp::default_env::*;
use risp::eval::parse_and_eval;

enum ParensStatus {
    MATCHING,
    LeftParensMore,
    RightParensMore,
}

enum FirstLineStatus {
    EXIT,
    GoNextLine(String),
    InvalidFirstLine(String),
    ValidOneLiner(String),
}

fn first_line_status(s: &str) -> ParensStatus {
    let mut valid = true;
    let mut stack = vec![];
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

fn add_to_history(x: &str, filename_cstr: &CStr) {
    unsafe {
        let c_string = CString::new(x).unwrap();
        let c_str = CStr::from_bytes_with_nul_unchecked(c_string.as_bytes_with_nul());
        linenoiseHistoryAdd(c_str.as_ptr() as *const i8);
        linenoiseHistorySave(filename_cstr.as_ptr() as *const c_char);
    }
}

fn handle_first_line() -> Option<FirstLineStatus> {
    let line = linenoise::input(">>>");
    let mut s = String::new();
    if line.is_none() {
        return None;
    } else {
        s += &line.clone().unwrap();
        if s.is_empty() {
            return Some(FirstLineStatus::ValidOneLiner(s));
        }
    }
    if s.as_str() == "exit" {
        return Some(FirstLineStatus::EXIT);
    } else {
        let status = first_line_status(s.as_str());
        match status {
            ParensStatus::MATCHING => {
                return Some(FirstLineStatus::ValidOneLiner(s));
            }
            ParensStatus::LeftParensMore => {
                return Some(FirstLineStatus::GoNextLine(s));
            }
            ParensStatus::RightParensMore => {
                return Some(FirstLineStatus::InvalidFirstLine(s));
            }
        }
    }
}

fn input() -> Option<String> {
    let filename = CString::new(".risp_history").unwrap();
    let filename_cstr =
        unsafe { CStr::from_bytes_with_nul_unchecked(filename.as_bytes_with_nul()) };
    unsafe {
        linenoiseHistoryLoad(filename_cstr.as_ptr() as *const c_char);
    }

    io::stdout().flush().unwrap();
    let mut s = String::new();
    let mut ret_none = false;

    let first_line = handle_first_line();
    if first_line.is_none() {
        return None;
    }
    match first_line.unwrap() {
        FirstLineStatus::EXIT => {
            add_to_history("exit", filename_cstr);
            exit(0);
        }
        FirstLineStatus::GoNextLine(x) => {
            add_to_history(x.as_str(), filename_cstr);
            s += &x;
        }
        FirstLineStatus::ValidOneLiner(x) | FirstLineStatus::InvalidFirstLine(x) => {
            add_to_history(x.as_str(), filename_cstr);
            return Some(x);
        }
    }

    loop {
        let line = linenoise::input("...");
        if line.is_none() {
            ret_none = true;
            break;
        } else {
            let unwrapped = line.unwrap();
            match unwrapped.as_str() {
                "" => break,
                t => {
                    add_to_history(&unwrapped, filename_cstr);
                    s += t;
                }
            }
        }
    }
    if !ret_none {
        Some(s.trim().to_owned())
    } else {
        None
    }
}

fn main() {
    let mut def_env = default_env();
    loop {
        let s = input();
        match &s {
            None => {
                continue;
            }
            Some(x) => match parse_and_eval(x.clone(), &mut def_env) {
                Ok(y) => println!("=> {}", y),
                Err(y) => eprintln!("{:?}", y),
            },
        }
    }
}
