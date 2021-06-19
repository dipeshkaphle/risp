use super::parser::*;
use super::types::*;

pub fn parse_and_eval(program: String, env: &mut Env) -> Result<Exp, Exceptions> {
    let parsed_exp = parse(program)?;
    let eval_exp = eval(&parsed_exp, env)?;
    Ok(eval_exp)
}

pub fn eval(exp: &Exp, env: &mut Env) -> Result<Exp, Exceptions> {
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
                        _ => {
                            // must be a function
                            let f = eval(first, env)?;
                            if let Exp::Func(function) = f {
                                let rest_evaluated: Result<Vec<Exp>, Exceptions> =
                                    rest.iter().map(|x| eval(x, env)).collect();
                                return function(&rest_evaluated?);
                            } else {
                                return Err(Exceptions::ValueError(
                                    format!("{} is not a defined as a function", s).to_string(),
                                ));
                            }
                        }
                    },
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
        Exp::Func(_) => Err(Exceptions::ValueError("Invalid form".to_string())),
    }
}

fn if_handler(args: &[Exp], env: &mut Env) -> Result<Exp, Exceptions> {
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

fn define_handler(args: &[Exp], env: &mut Env) -> Result<Exp, Exceptions> {
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
