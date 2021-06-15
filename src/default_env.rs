use super::builtin_functions::*;
use super::types::*;
use std::collections::HashMap;

pub fn default_env() -> Env {
    let mut env: Env = HashMap::new();
    env.insert(
        "+".to_string(),
        Exp::Func(|args| binary_op_arith(&args[..], 0.0, |x, y| x + y)),
    );
    env.insert(
        "*".to_string(),
        Exp::Func(|args| binary_op_arith(&args[..], 1_f64, |x, y| x * y)),
    );
    env.insert("-".to_string(), Exp::Func(|args| minus(args)));
    env.insert("/".to_string(), Exp::Func(|args| divide(args)));
    env.insert("fmod".to_string(), Exp::Func(|args| fmod(args)));
    env.insert("mod".to_string(), Exp::Func(|args| mod_int(args)));
    env.insert(
        ">".to_string(),
        Exp::Func(|args| binary_cmp(args, |x, y| x > y)),
    );
    env.insert(
        "<".to_string(),
        Exp::Func(|args| binary_cmp(args, |x, y| x < y)),
    );
    env.insert(
        "=".to_string(),
        Exp::Func(|args| binary_cmp(args, |x, y| x == y)),
    );
    env.insert(
        ">=".to_string(),
        Exp::Func(|args| binary_cmp(args, |x, y| x >= y)),
    );
    env.insert(
        "<=".to_string(),
        Exp::Func(|args| binary_cmp(args, |x, y| x <= y)),
    );
    env.insert(
        "and".to_string(),
        Exp::Func(|args| logical_bin_ops(args, |x, y| x && y)),
    );
    env.insert(
        "or".to_string(),
        Exp::Func(|args| logical_bin_ops(args, |x, y| x && y)),
    );
    env.insert("not".to_string(), Exp::Func(|args| logical_not(args)));
    env
}
