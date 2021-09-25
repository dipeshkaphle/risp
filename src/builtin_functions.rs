use super::types::*;
use std::rc::Rc;
use std::{f64, i64};
pub fn logical_bin_ops(args: &[Exp], f: fn(bool, bool) -> bool) -> Result<Exp, Exceptions> {
    let evaluated: Result<Vec<bool>, Exceptions> = args
        .into_iter()
        .map(|x| -> Result<bool, Exceptions> { get_bool(x) })
        .collect();
    let ans = evaluated?.iter().fold(true, |acc, x| f(acc, *x));
    return Ok(Exp::Atom(Atom::Bool(ans)));
}

pub fn binary_cmp(args: &[Exp], f: fn(f64, f64) -> bool) -> Result<Exp, Exceptions> {
    if args.len() != 2 {
        return Err(Exceptions::ValueError(
            format!("expected two arguments for comparision got {}", args.len()).to_string(),
        ));
    } else {
        let (a, b) = (get_float(&args[0])?, get_float(&args[1])?);
        return Ok(Exp::Atom(Atom::Bool(f(a, b))));
    }
}

pub fn binary_op_arith(args: &[Exp], init: f64, f: fn(f64, f64) -> f64) -> Result<Exp, Exceptions> {
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

pub fn minus(args: &[Exp]) -> Result<Exp, Exceptions> {
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
}

pub fn divide(args: &[Exp]) -> Result<Exp, Exceptions> {
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
}

pub fn fmod(args: &[Exp]) -> Result<Exp, Exceptions> {
    let _ = expect_x_args(2, "fmod", args)?;
    return Ok(Exp::Atom(Atom::Number(Number::Float(
        get_float(&args[0])? % get_float(&args[1])?,
    ))));
}

pub fn mod_int(args: &[Exp]) -> Result<Exp, Exceptions> {
    let _ = expect_x_args(2, "mod", args)?;
    return Ok(Exp::Atom(Atom::Number(Number::Int(
        get_int(&args[0])? % get_int(&args[1])?,
    ))));
}

pub fn logical_not(args: &[Exp]) -> Result<Exp, Exceptions> {
    let _ = expect_x_args(1, "not", args)?;
    let operand = get_bool(&args[0])?;
    return Ok(Exp::Atom(Atom::Bool(!operand)));
}

pub fn absolute_val(args: &[Exp]) -> Result<Exp, Exceptions> {
    let _ = expect_x_args(1, "abs", args)?;
    match &args[0] {
        Exp::Atom(Atom::Number(x)) => match x {
            Number::Int(y) => Ok(Exp::Atom(Atom::Number(Number::Int(y.abs())))),
            Number::Float(y) => Ok(Exp::Atom(Atom::Number(Number::Float(y.abs())))),
        },
        _ => Err(Exceptions::ValueError(
            "Number not provided to abs".to_string(),
        )),
    }
}

pub fn power(args: &[Exp]) -> Result<Exp, Exceptions> {
    let _ = expect_x_args(2, "expt", args)?;
    return Ok(Exp::Atom(Atom::Number(Number::Float(
        get_float(&args[0])?.powf(get_float(&args[1])?),
    ))));
}

pub fn begin(args: &[Exp]) -> Result<Exp, Exceptions> {
    if args.is_empty() {
        return Err(Exceptions::ValueError(
            "Expected atleast one expression after begin".to_string(),
        ));
    } else {
        return Ok(args.last().unwrap().clone());
    }
}

pub fn append(args: &[Exp]) -> Result<Exp, Exceptions> {
    let mut ret_list = vec![];
    for exps in args {
        match exps {
            Exp::List(vals) => {
                for e in vals.as_ref() {
                    ret_list.push(e.clone());
                }
            }
            _ => ret_list.push(exps.clone()),
        }
    }
    Ok(Exp::List(Rc::new(ret_list)))
}

pub fn apply(args: &[Exp]) -> Result<Exp, Exceptions> {
    expect_x_args(2, "apply", args)?;
    if let Exp::Func(f) = &args[0] {
        if let Exp::List(params) = &args[1] {
            return f(params);
        } else {
            return Err(Exceptions::ValueError(
                "Expected a list as second argument to apply. Got something else".to_string(),
            ));
        }
    } else {
        return Err(Exceptions::ValueError(
            "Expected the first argument for apply to be a function".to_string(),
        ));
    }
}

pub fn cons(args: &[Exp]) -> Result<Exp, Exceptions> {
    expect_x_args(2, "cons", args)?;
    if let Exp::List(lst) = &args[1] {
        let ret_list = [[args[0].clone()].to_vec(), lst.as_ref().clone()].concat();
        Ok(Exp::List(Rc::new(ret_list)))
    } else {
        return Err(Exceptions::ValueError(
            "Expected a list as second argument to cons. Got something else".to_string(),
        ));
    }
}

fn head_tails<'a>(args: &'a [Exp]) -> Result<(&'a Exp, &'a [Exp]), Exceptions> {
    let _ = expect_x_args(1, "car", args)?;
    if let Exp::List(lst) = &args[0] {
        let (head, tails) = lst
            .split_first()
            .ok_or(Exceptions::ValueError(
                "The list should have length atleast 1".to_string(),
            ))
            .unwrap();
        return Ok((head, tails));
    } else {
        Err(Exceptions::ValueError(
            "Expected a list after car".to_string(),
        ))
    }
}
fn same_object(a: &Exp, b: &Exp) -> bool {
    match a {
        Exp::List(lst1) => {
            if let Exp::List(lst2) = b {
                lst1.as_ref() as *const Vec<Exp> == lst2.as_ref() as *const Vec<Exp>
            } else {
                false
            }
        }
        _ => a == b,
    }
    // a as *const Exp == b as *const Exp
}
pub fn same_obj(args: &[Exp]) -> Result<Exp, Exceptions> {
    expect_x_args(2, "equal?", args)?;
    return Ok(Exp::Atom(Atom::Bool(same_object(&args[0], &args[1]))));
}
pub fn equal(args: &[Exp]) -> Result<Exp, Exceptions> {
    expect_x_args(2, "equal?", args)?;
    return Ok(Exp::Atom(Atom::Bool(&args[0] == &args[1])));
}

pub fn length(args: &[Exp]) -> Result<Exp, Exceptions> {
    expect_x_args(1, "length", args)?;
    if let Exp::List(lst) = &args[0] {
        Ok(Exp::Atom(Atom::Number(Number::Int(lst.len() as i64))))
    } else {
        Err(Exceptions::ValueError(
            "non list type passed to length function".to_string(),
        ))
    }
}

pub fn is_list(args: &[Exp]) -> Result<Exp, Exceptions> {
    expect_x_args(1, "list?", args)?;
    if let Exp::List(_) = &args[0] {
        return Ok(Exp::Atom(Atom::Bool(true)));
    } else {
        return Ok(Exp::Atom(Atom::Bool(false)));
    }
}

pub fn car(args: &[Exp]) -> Result<Exp, Exceptions> {
    return Ok(head_tails(args)?.0.clone());
}
pub fn cdr(args: &[Exp]) -> Result<Exp, Exceptions> {
    return Ok(Exp::List(Rc::new(head_tails(args)?.1.to_vec())));
}
pub fn is_null(args: &[Exp]) -> Result<Exp, Exceptions> {
    expect_x_args(1, "null?", args)?;
    if let Exp::List(lst) = &args[0] {
        Ok(Exp::Atom(Atom::Bool(lst.is_empty())))
    } else {
        Ok(Exp::Atom(Atom::Bool(false)))
    }
}

pub fn min_max(args: &[Exp], funcname: &str, f: fn(f64, f64) -> f64) -> Result<Exp, Exceptions> {
    expect_atleast_x_args(1, funcname, args)?;
    let evaluated: Result<Vec<f64>, Exceptions> = args
        .into_iter()
        .map(|x| -> Result<f64, Exceptions> { get_float(x) })
        .collect();
    let evaluated = evaluated?;
    let mut ans = evaluated[0].clone();
    for elem in evaluated {
        ans = f(ans, elem);
    }
    let rounded_down = ans.floor();
    if ans == rounded_down {
        return Ok(Exp::Atom(Atom::Number(Number::Int(rounded_down as i64))));
    } else {
        return Ok(Exp::Atom(Atom::Number(Number::Float(ans))));
    }
}

pub fn is_number(args: &[Exp]) -> Result<Exp, Exceptions> {
    expect_x_args(1, "number?", args)?;
    if let Exp::Atom(Atom::Number(_)) = &args[0] {
        Ok(Exp::Atom(Atom::Bool(true)))
    } else {
        Ok(Exp::Atom(Atom::Bool(false)))
    }
}

pub fn is_proc(args: &[Exp]) -> Result<Exp, Exceptions> {
    expect_x_args(1, "procedure?", args)?;
    match &args[0] {
        Exp::Func(_) => Ok(Exp::Atom(Atom::Bool(true))),
        Exp::Procedure(_) => Ok(Exp::Atom(Atom::Bool(true))),
        _ => Ok(Exp::Atom(Atom::Bool(false))),
    }
}
pub fn is_bool(args: &[Exp]) -> Result<Exp, Exceptions> {
    expect_x_args(1, "bool?", args)?;
    if let Exp::Atom(Atom::Bool(_)) = &args[0] {
        Ok(Exp::Atom(Atom::Bool(true)))
    } else {
        Ok(Exp::Atom(Atom::Bool(false)))
    }
}

pub fn map(args: &[Exp]) -> Result<Exp, Exceptions> {
    expect_x_args(2, "map", args)?;
    println!("hi");
    let is_callable = get_bool(&is_proc(&args[..1])?)?;
    if is_callable {
        if let Exp::List(lst) = &args[1] {
            let mut mapped_list = vec![Exp::Atom(Atom::Symbol("list".to_string()))];
            for x in lst.iter() {
                mapped_list.push(Exp::List(Rc::new(vec![args[0].clone(), x.clone()])));
            }
            Ok(Exp::List(Rc::new(mapped_list)))
        } else {
            Err(Exceptions::ValueError(
                "Expected a list as second argument to map".to_string(),
            ))
        }
    } else {
        Err(Exceptions::ValueError(
            "Expected a callable as first argument to map".to_string(),
        ))
    }
}

pub fn expect_x_args(x: usize, func_name: &str, args: &[Exp]) -> Result<usize, Exceptions> {
    if args.len() != x {
        return Err(Exceptions::ValueError(
            format!(
                "expected {} arguments for {}, got {}",
                x,
                func_name,
                args.len()
            )
            .to_string(),
        ));
    } else {
        Ok(x)
    }
}
pub fn expect_atleast_x_args(x: usize, func_name: &str, args: &[Exp]) -> Result<usize, Exceptions> {
    if args.len() < x {
        return Err(Exceptions::ValueError(
            format!(
                "expected at least {} arguments for {}, got {}",
                x,
                func_name,
                args.len()
            )
            .to_string(),
        ));
    } else {
        Ok(x)
    }
}
