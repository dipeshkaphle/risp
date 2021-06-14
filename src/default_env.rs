use super::types::*;
use std::collections::HashMap;

pub fn default_env() -> Env {
    let mut env: Env = HashMap::new();
    env.insert(
        "+".to_string(),
        Exp::Func(|args| -> Result<Exp, Exceptions> {
            binary_op_arith(&args[..], 0.0, |x, y| x + y)
        }),
    );
    env.insert(
        "*".to_string(),
        Exp::Func(|args| binary_op_arith(&args[..], 1_f64, |x, y| x * y)),
    );
    env.insert(
        "-".to_string(),
        Exp::Func(|args| {
            let (head, tail) = args
                .split_first()
                .ok_or(Exceptions::ValueError(
                    "- must have one argument at least".to_string(),
                ))
                .unwrap();
            let first = get_float(head)?;
            let rem_sum = binary_op_arith(tail, 0_f64, |x, y| x + y)?;
            let ans = first - (get_float(&rem_sum)?);
            let rounded_down = ans.floor();
            if ans == rounded_down {
                if tail.is_empty() {
                    return Ok(Exp::Atom(Atom::Number(Number::Int(-rounded_down as i64))));
                } else {
                    return Ok(Exp::Atom(Atom::Number(Number::Int(rounded_down as i64))));
                }
            } else {
                if tail.is_empty() {
                    return Ok(Exp::Atom(Atom::Number(Number::Float(-ans))));
                } else {
                    return Ok(Exp::Atom(Atom::Number(Number::Float(ans))));
                }
            }
        }),
    );
    env.insert(
        "/".to_string(),
        Exp::Func(|args| {
            let (head, tail) = args
                .split_first()
                .ok_or(Exceptions::ValueError(
                    "/ must have one argument at least".to_string(),
                ))
                .unwrap();
            let first = get_float(head)?;
            let rem_sum = binary_op_arith(tail, 1_f64, |x, y| x * y)?;
            let ans = first * 1_f64 / (get_float(&rem_sum)?);
            if tail.is_empty() {
                return Ok(Exp::Atom(Atom::Number(Number::Float(1_f64 / ans))));
            } else {
                return Ok(Exp::Atom(Atom::Number(Number::Float(ans))));
            }
        }),
    );
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
    env.insert(
        "not".to_string(),
        Exp::Func(|args| {
            if args.len() != 1 {
                return Err(Exceptions::ValueError(
                    format!("expected 1 arguments for comparision got {}", args.len()).to_string(),
                ));
            } else {
                let operand = get_bool(&args[0])?;
                return Ok(Exp::Atom(Atom::Bool(!operand)));
            }
        }),
    );
    env
}

fn logical_bin_ops(args: &[Exp], f: fn(bool, bool) -> bool) -> Result<Exp, Exceptions> {
    let evaluated: Result<Vec<bool>, Exceptions> = args
        .into_iter()
        .map(|x| -> Result<bool, Exceptions> { get_bool(x) })
        .collect();
    let ans = evaluated?.iter().fold(true, |acc, x| f(acc, *x));
    return Ok(Exp::Atom(Atom::Bool(ans)));
}

fn binary_cmp(args: &[Exp], f: fn(f64, f64) -> bool) -> Result<Exp, Exceptions> {
    if args.len() != 2 {
        return Err(Exceptions::ValueError(
            format!("expected two arguments for comparision got {}", args.len()).to_string(),
        ));
    } else {
        let (a, b) = (get_float(&args[0])?, get_float(&args[1])?);
        return Ok(Exp::Atom(Atom::Bool(f(a, b))));
    }
}

fn binary_op_arith(args: &[Exp], init: f64, f: fn(f64, f64) -> f64) -> Result<Exp, Exceptions> {
    let evaluated: Result<Vec<f64>, Exceptions> = args
        .into_iter()
        .map(|x| -> Result<f64, Exceptions> { get_float(x) })
        .collect();
    let ans = evaluated?.iter().fold(init as f64, |sum, x| f(sum, *x));
    let rounded_down = ans.floor();
    if ans == rounded_down {
        return Ok(Exp::Atom(Atom::Number(Number::Int(rounded_down as i64))));
    } else {
        return Ok(Exp::Atom(Atom::Number(Number::Float(ans))));
    }
}
