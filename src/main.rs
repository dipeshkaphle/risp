use risp::default_env::*;
use risp::eval::parse_and_eval;
use risp::runner::{repl, run_from_source_code};

use std::env;
use std::fs::File;
use std::io::Read;

fn main() -> std::io::Result<()> {
    if env::args().len() >= 2 {
        let args: Vec<String> = env::args().collect();
        let mut file = File::open(&args[1])?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        run_from_source_code(contents);
    } else {
        let mut def_env = default_env();
        loop {
            let s = repl();
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
    Ok(())
}
