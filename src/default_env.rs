use super::builtin_functions::*;
use super::types::*;
use std::f64;
use std::rc::Rc;

pub fn default_env() -> Environment {
    let mut env: Environment = Environment::new();
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
    env.insert("same_obj?".to_string(), Exp::Func(|args| same_obj(args)));
    env.insert("equal?".to_string(), Exp::Func(|args| equal(args)));
    env.insert("length".to_string(), Exp::Func(|args| length(args)));
    env.insert("list?".to_string(), Exp::Func(|args| is_list(args)));
    env.insert(
        "max".to_string(),
        Exp::Func(|args| min_max(args, "max", |x: f64, y: f64| x.max(y))),
    );
    env.insert(
        "min".to_string(),
        Exp::Func(|args| min_max(args, "min", |x: f64, y: f64| x.min(y))),
    );
    env.insert("null?".to_string(), Exp::Func(|args| is_null(args)));
    env.insert("number?".to_string(), Exp::Func(|args| is_number(args)));
    env.insert("procedure?".to_string(), Exp::Func(|args| is_proc(args)));
    env.insert("bool?".to_string(), Exp::Func(|args| is_bool(args)));
    env.insert("map".to_string(), Exp::Func(|args| map(args)));
    env.insert(
        "list".to_string(),
        Exp::Func(|args| {
            let x = Exp::List(Rc::new(args.to_vec()));
            Ok(x)
        }),
    );

    env
}
