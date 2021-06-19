use super::types::*;
fn tokenize(chars: &str) -> Vec<String> {
    chars
        .replace("(", "( ")
        .replace(")", " )")
        .replace("'", "' ")
        .split_whitespace()
        .map(|x| String::from(x))
        .collect()
}

fn atom(token: String) -> Atom {
    match &token.parse::<i64>() {
        Ok(x) => return Atom::Number(Number::Int(*x)),
        Err(_) => match &token.parse::<f64>() {
            Ok(x) => return Atom::Number(Number::Float(*x)),
            Err(_) => match &token[..] {
                "#t" => Atom::Bool(true),
                "#f" => Atom::Bool(false),
                _ => Atom::Symbol(token),
            },
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
            if tokens.is_empty() {
                return Err(Exceptions::SyntaxError("Non matching parens".to_string()));
            }
        }
        tokens.remove(0);
        return Ok(Exp::List(l));
    } else if token == ")" {
        return Err(Exceptions::SyntaxError("Unexpected )".to_string()));
    } else if token == "'" {
        let mut lst = vec![];
        lst.push(Exp::Atom(Atom::Symbol("quote".to_string())));
        lst.push(read_from_tokens(tokens).unwrap());
        return Ok(Exp::List(lst));
    } else {
        return Ok(Exp::Atom(atom(token)));
    }
}

fn _get_matching_parens_and_remaining(program: &str) -> Option<(&str, &str)> {
    if !program.starts_with("(") {
        return None;
    } else {
        let mut stack: Vec<char> = vec![];
        let mut ends_at = 0;
        for x in program.chars() {
            if x == '(' {
                stack.push('(');
            } else if !stack.is_empty() && *stack.last().unwrap() == '(' && x == ')' {
                stack.pop();
            }
            ends_at += 1;
            if stack.is_empty() {
                break;
            }
        }
        return Some((&program[..ends_at], &program[ends_at..]));
    }
}

pub fn parse(program: String) -> Result<Exp, Exceptions> {
    let mut tokenized = tokenize(&program);
    let ans = read_from_tokens(&mut tokenized);
    if tokenized.is_empty() {
        ans
    } else {
        Err(Exceptions::SyntaxError(
            format!(
                "Non matching parens or invalid expression : '{}'",
                tokenized.join(" ")
            )
            .to_string(),
        ))
    }
}
