use super::builtin_functions::*;
use super::types::*;
use std::collections::HashMap;
use std::f64;

pub fn default_env() -> Env {
    let mut env: Env = HashMap::new();
    env.insert(
        "pi".to_string(),
        Exp::Atom(Atom::Number(Number::Float(f64::consts::PI))),
    );
    env.insert(
        "exp".to_string(),
        Exp::Atom(Atom::Number(Number::Float(f64::consts::E))),
    );
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
    env.insert("abs".to_string(), Exp::Func(|args| absolute_val(args)));
    env.insert("expt".to_string(), Exp::Func(|args| power(args)));
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
    env.insert("begin".to_string(), Exp::Func(|args| begin(args)));
    env.insert("append".to_string(), Exp::Func(|args| append(args)));
    env.insert("car".to_string(), Exp::Func(|args| car(args)));
    env.insert("cdr".to_string(), Exp::Func(|args| cdr(args)));
    env.insert("apply".to_string(), Exp::Func(|args| apply(args)));
    env.insert("cons".to_string(), Exp::Func(|args| cons(args)));
    env.insert("equal?".to_string(), Exp::Func(|args| equal(args)));
    env.insert("length".to_string(), Exp::Func(|args| length(args)));

    env
}
