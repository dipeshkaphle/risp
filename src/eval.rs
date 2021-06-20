use std::collections::HashMap;

use super::parser::*;
use super::types::*;

pub fn parse_and_eval(program: String, env: &mut Environment) -> Result<Exp, Exceptions> {
    let parsed_exp = parse(program)?;
    let eval_exp = eval(&parsed_exp, env)?;
    Ok(eval_exp)
}

pub fn eval(exp: &Exp, env: &mut Environment) -> Result<Exp, Exceptions> {
    match &exp {
        Exp::Atom(x) => match x {
            Atom::Symbol(y) => env.get(y).map(|z| z.clone()).ok_or(Exceptions::ValueError(
                format!("{} is not a valid symbol", y).to_string(),
            )),
            _ => Ok(exp.clone()),
        },
        Exp::List(x) => {
            if x.is_empty() {
                return Err(Exceptions::ValueError(
                    "Expected a non empty list".to_string(),
                ));
            } else {
                let (first, rest) = x.split_first().unwrap();
                let f = eval(first, env);
                match first {
                    Exp::Atom(Atom::Symbol(s)) => match &s[..] {
                        "if" => if_handler(x, env),
                        "define" => define_handler(x, env),
                        "quote" => {
                            if rest.is_empty() {
                                Err(Exceptions::ValueError(
                                    "Expected something after quote".to_string(),
                                ))
                            } else {
                                Ok(rest[0].clone())
                            }
                        }
                        "set!" => {
                            if rest.len() != 2 {
                                Err(Exceptions::ValueError(
                                    "Expected 2 things after set!,symbol followed by an expression"
                                        .to_string(),
                                ))
                            } else {
                                if let Exp::Atom(Atom::Symbol(x)) = &rest[0] {
                                    let evaluated_exp = eval(&rest[1], env)?;
                                    if env.exists(x) {
                                        env.insert(x.clone(), evaluated_exp.clone());
                                        Ok(evaluated_exp)
                                    } else {
                                        Err(Exceptions::ValueError(
                                            " set!: assignment disallowed\
                                                , cannot set variable before its definition "
                                                .to_string(),
                                        ))
                                    }
                                } else {
                                    Err(Exceptions::ValueError(
                                        "Expected a symbol after set!, got something else"
                                            .to_string(),
                                    ))
                                }
                            }
                        }
                        "lambda" => {
                            // two lists
                            // one for params and another is body
                            if rest.len() != 2 {
                                return Err(Exceptions::SyntaxError(
                                    "lambda expects a params list and body".to_string(),
                                ));
                            }
                            if let Exp::List(params) = &rest[0] {
                                let params_as_strings: Result<Vec<String>, Exceptions> = params
                                    .iter()
                                    .map(|x| {
                                        if let Exp::Atom(Atom::Symbol(y)) = x {
                                            return Ok(y.clone());
                                        } else {
                                            return Err(Exceptions::ValueError(
                                                "non symbol passed in a lambda parameters list"
                                                    .to_string(),
                                            ));
                                        }
                                    })
                                    .collect();
                                Ok(Exp::Procedure((
                                    params_as_strings?,
                                    Box::new(rest[1].clone()),
                                )))
                            } else {
                                return Err(Exceptions::ValueError(
                                    "Expected a params LIST after lambda keyword".to_string(),
                                ));
                            }
                        }
                        _ => {
                            // must be a function
                            match f? {
                                Exp::Func(function) => func_handler(function, rest, env),
                                Exp::Procedure(proc) => proc_handler(&proc, rest, env),
                                _ => {
                                    return Err(Exceptions::ValueError(
                                        format!("{} is not a defined as a function", s).to_string(),
                                    ));
                                }
                            }
                        }
                    },
                    Exp::Procedure(proc) => proc_handler(proc, rest, env),
                    Exp::Func(func) => func_handler(*func, rest, env),
                    _ => {
                        // invalid expression
                        return Err(Exceptions::ValueError(
                            format!(
                                "First thing in an expression should be a keyword or a function not {}",
                                first
                            )
                            .to_string(),
                        ));
                    }
                }
            }
        }
        _ => Err(Exceptions::ValueError("Invalid form".to_string())),
    }
}

fn if_handler(args: &[Exp], env: &mut Environment) -> Result<Exp, Exceptions> {
    let x = args;
    if x.len() == 4 {
        let (test, conseq, alt) = (&x[1], &x[2], &x[3]);
        if let Exp::Atom(Atom::Bool(test_evaluated)) = eval(test, env)? {
            if test_evaluated {
                return eval(conseq, env);
            } else {
                return eval(alt, env);
            }
        } else {
            return Err(Exceptions::ValueError(
                format!("{} doesnt evaluate to a boolean", test).to_string(),
            ));
        }
    } else {
        return Err(Exceptions::ValueError(
            "Not a valid if expression".to_string(),
        ));
    }
}

fn define_handler(args: &[Exp], env: &mut Environment) -> Result<Exp, Exceptions> {
    if args.len() == 3 {
        let (symbol, exp) = (&args[1], &args[2]);
        if let Exp::Atom(Atom::Symbol(x)) = symbol {
            let evaluated_exp = eval(exp, env)?;
            env.insert(x.clone(), evaluated_exp.clone());
            Ok(evaluated_exp)
        } else {
            return Err(Exceptions::ValueError(
                format!(
                    "define expression should have its first argument as a symbol not a {}",
                    symbol
                )
                .to_string(),
            ));
        }
    } else {
        return Err(Exceptions::ValueError(
            "Not a valid define expression".to_string(),
        ));
    }
}

fn proc_handler(
    proc: &(Vec<String>, Box<Exp>),
    rest: &[Exp],
    env: &mut Environment,
) -> Result<Exp, Exceptions> {
    if rest.len() == proc.0.len() {
        let rest_evaluated: Result<Vec<Exp>, Exceptions> =
            rest.iter().map(|x| eval(x, env)).collect();
        let rest_evaluated = rest_evaluated?;
        env.push_stack_frame(HashMap::new());
        for i in 0..rest_evaluated.len() {
            env.insert(proc.0[i].clone(), rest_evaluated[i].clone());
        }
        let ans = eval(proc.1.as_ref(), env);
        env.pop_stack_frame();
        ans
    } else {
        return Err(Exceptions::ValueError(
            format!("Expected {} arguments but got {}", proc.0.len(), rest.len()).to_string(),
        ));
    }
}

pub fn func_handler(
    function: fn(&[Exp]) -> Result<Exp, Exceptions>,
    args: &[Exp],
    env: &mut Environment,
) -> Result<Exp, Exceptions> {
    let rest_evaluated: Result<Vec<Exp>, Exceptions> = args.iter().map(|x| eval(x, env)).collect();
    let func_result = function(&rest_evaluated?)?;
    let eval_again = eval(&func_result, env);
    eval_again.or_else(|_| Ok(func_result))
}
