use std::collections::HashMap;
use std::error::Error;
use std::io::{self, Write};

use risp::default_env::*;
use risp::eval;
use risp::eval::parse_and_eval;
use risp::parser::*;
use risp::types::*;

fn input() -> String {
    print!(">>>");
    io::stdout().flush().unwrap();
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read line");
    s.to_owned()
}

fn main() {
    let mut def_env = default_env();
    loop {
        let s = input();
        if let Ok(x) = parse_and_eval(s, &def_env) {
            println!("=> {}", x);
        } else {
            println!("Error");
        }
    }
}
