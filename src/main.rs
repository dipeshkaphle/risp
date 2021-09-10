use std::ffi::{CStr, CString};
use std::io::{self, Write};
use std::os::raw::c_char;

use linenoise::ffi::{
    linenoiseHistoryAdd, linenoiseHistoryLine, linenoiseHistoryLoad, linenoiseHistorySave,
};
use risp::default_env::*;
use risp::eval::parse_and_eval;

fn input() -> String {
    print!(">>>");
    io::stdout().flush().unwrap();
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read line");
    s.trim().to_owned()
}

fn main() {
    let mut def_env = default_env();
    // let filename;
    // let filename_cstr;
    let filename = CString::new(".risp_history").unwrap();
    let filename_cstr =
        unsafe { CStr::from_bytes_with_nul_unchecked(filename.as_bytes_with_nul()) };
    unsafe {
        linenoiseHistoryLoad(filename_cstr.as_ptr() as *const c_char);
    }
    loop {
        let s = linenoise::input(">>>");
        match &s {
            None => {
                break;
            }
            Some(x) => {
                match parse_and_eval(x.clone(), &mut def_env) {
                    Ok(y) => println!("=> {}", y),
                    Err(y) => eprintln!("{:?}", y),
                }
                unsafe {
                    let c_string = CString::new(x.as_str()).unwrap();
                    let c_str = CStr::from_bytes_with_nul_unchecked(c_string.as_bytes_with_nul());
                    linenoiseHistoryAdd(c_str.as_ptr() as *const i8);
                    linenoiseHistorySave(filename_cstr.as_ptr() as *const c_char);
                }
            }
        }
    }
}
