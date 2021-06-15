use std::io::{self, Write};

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
    loop {
        let s = input();
        match parse_and_eval(s, &mut def_env) {
            Ok(x) => println!("=> {}", x),
            Err(x) => println!("{:?}", x),
        }
    }
}
