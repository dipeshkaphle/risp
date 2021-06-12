use super::parser::*;
use super::types::*;
pub fn eval(exp: &Exp, env: &Env) -> Result<Exp, Exceptions> {
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
                let f = eval(first, env)?;
                if let Exp::Func(function) = f {
                    return function(rest);
                } else {
                    return Err(Exceptions::ValueError(
                        "First arg should be a function".to_string(),
                    ));
                }
            }
        }
        Exp::Func(_) => Err(Exceptions::ValueError("Invalid form".to_string())),
    }
}

pub fn parse_and_eval(program: String, env: &Env) -> Result<Exp, Exceptions> {
    let parsed_exp = parse(program)?;
    let eval_exp = eval(&parsed_exp, env)?;
    Ok(eval_exp)
}
