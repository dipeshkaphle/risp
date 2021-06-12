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
            if args.is_empty() {
                return Err(Exceptions::ValueError(
                    "- must have one argument at least".to_string(),
                ));
            } else {
                let first = get_float(&args[0])?;
                let rem_sum = binary_op_arith(&args[1..], 0_f64, |x, y| x + y)?;
                let ans = first - (get_float(&rem_sum)?);
                let rounded_down = ans.floor();
                if ans == rounded_down {
                    return Ok(Exp::Atom(Atom::Number(Number::Int(rounded_down as i64))));
                } else {
                    return Ok(Exp::Atom(Atom::Number(Number::Float(ans))));
                }
            }
        }),
    );
    env.insert(
        "/".to_string(),
        Exp::Func(|args| {
            if args.is_empty() {
                return Err(Exceptions::ValueError(
                    "- must have one argument at least".to_string(),
                ));
            } else {
                let first = get_float(&args[0])?;
                let rem_sum = binary_op_arith(&args[1..], 1_f64, |x, y| x * y)?;
                let ans = first * (1_f64 / (get_float(&rem_sum)?));
                let rounded_down = ans.floor();
                if ans == rounded_down {
                    return Ok(Exp::Atom(Atom::Number(Number::Int(rounded_down as i64))));
                } else {
                    return Ok(Exp::Atom(Atom::Number(Number::Float(ans))));
                }
            }
        }),
    );
    env
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
