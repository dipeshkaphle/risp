use super::types::*;
fn tokenize(chars: &str) -> Vec<String> {
    chars
        .replace("(", "( ")
        .replace(")", " )")
        .split_whitespace()
        .map(|x| String::from(x))
        .collect()
}

fn atom(token: String) -> Atom {
    match &token.parse::<i64>() {
        Ok(x) => return Atom::Number(Number::Int(*x)),
        Err(_) => match &token.parse::<f64>() {
            Ok(x) => return Atom::Number(Number::Float(*x)),
            Err(_) => {
                return Atom::Symbol(token);
            }
        },
    }
}

fn read_from_tokens(tokens: &mut Vec<String>) -> Result<Exp, Exceptions> {
    if tokens.is_empty() {
        return Err(Exceptions::SyntaxError("Unexpected EOF".to_string()));
    }
    let token = tokens[0].clone();
    tokens.remove(0);
    if token == "(" {
        let mut l = Vec::new();
        while tokens[0] != ")" {
            l.push(read_from_tokens(tokens).unwrap());
        }
        tokens.remove(0);
        return Ok(Exp::List(l));
    } else if token == ")" {
        return Err(Exceptions::SyntaxError("Unexpected )".to_string()));
    } else {
        return Ok(Exp::Atom(atom(token)));
    }
}

pub fn parse(program: String) -> Result<Exp, Exceptions> {
    return read_from_tokens(&mut tokenize(&program));
}
