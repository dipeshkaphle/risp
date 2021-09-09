use super::types::*;
use std::rc::Rc;
fn tokenize(chars: &str) -> Vec<String> {
    let mut toks = vec![];
    let mut i = 0;
    let characters: Vec<char> = chars.chars().collect();
    while i < characters.len() {
        match characters[i] {
            '(' => toks.push("(".to_string()),
            ')' => toks.push(")".to_string()),
            '"' => {
                toks.push("\"".to_string());
                i += 1;
                let mut s = String::new();
                while i < characters.len() && characters[i] != '"' {
                    match characters[i] {
                        '\\' => {
                            if i + 1 < characters.len() {
                                s.push(characters[i + 1]);
                                i += 1;
                            } else {
                                panic!("Invalid escape '\'");
                            }
                        }
                        c => {
                            s.push(c);
                        }
                    }
                    i += 1;
                }
                toks.push(s);
            }
            '\'' => {
                toks.push("'".to_string());
            }
            _ => {
                if characters[i].is_whitespace() {
                    while i < characters.len() && characters[i].is_whitespace() {
                        i += 1;
                    }
                    i -= 1;
                } else {
                    let mut s = String::new();
                    while i < characters.len()
                        && (!characters[i].is_whitespace()
                            && characters[i] != ')'
                            && characters[i] != '(')
                    {
                        s.push(characters[i]);
                        i += 1;
                    }
                    i -= 1;
                    toks.push(s);
                }
            }
        }
        i += 1;
    }
    toks
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
        return Ok(Exp::List(Rc::new(l)));
    } else if token == ")" {
        return Err(Exceptions::SyntaxError("Unexpected )".to_string()));
    } else if token == "'" {
        let mut lst = vec![];
        lst.push(Exp::Atom(Atom::Symbol("quote".to_string())));
        lst.push(read_from_tokens(tokens).unwrap());
        return Ok(Exp::List(Rc::new(lst)));
    } else if token == "\"" {
        let s = tokens[0].clone();
        tokens.remove(0);
        return Ok(Exp::Str(s));
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
