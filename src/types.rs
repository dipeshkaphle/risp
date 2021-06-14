use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fmt::format;
use std::io;

pub type Symbol = String;

#[derive(Debug, Clone)]
pub enum Exceptions {
    ValueError(String),
    SyntaxError(String),
}
#[derive(Debug, Clone)]
pub enum Number {
    Int(i64),
    Float(f64),
}
#[derive(Debug, Clone)]
pub enum Atom {
    Bool(bool),
    Symbol(Symbol),
    Number(Number),
}
#[derive(Clone)]
pub enum Exp {
    Atom(Atom),
    List(Vec<Exp>),
    Func(fn(&[Exp]) -> Result<Exp, Exceptions>),
}

pub type Env = HashMap<String, Exp>;

pub trait To_Float {
    fn to_f64(&self) -> Option<f64>;
}

impl To_Float for Exp {
    fn to_f64(&self) -> Option<f64> {
        if let Exp::Atom(Atom::Number(y)) = self {
            match y {
                Number::Int(z) => return Some(*z as f64),
                Number::Float(z) => return Some(*z),
            }
        }
        None
    }
}
pub trait To_Int {
    fn to_i64(&self) -> Option<i64>;
}
impl To_Int for Exp {
    fn to_i64(&self) -> Option<i64> {
        if let Exp::Atom(Atom::Number(y)) = self {
            match y {
                Number::Int(z) => return Some(*z),
                Number::Float(z) => return Some(*z as i64),
            }
        }
        None
    }
}
pub trait To_Bool {
    fn to_bool(&self) -> Option<bool>;
}

impl To_Bool for Exp {
    fn to_bool(&self) -> Option<bool> {
        if let Exp::Atom(Atom::Bool(y)) = self {
            return Some(*y);
        }
        None
    }
}
pub fn get_bool(x: &Exp) -> Result<bool, Exceptions> {
    x.to_bool()
        .ok_or(Exceptions::ValueError("Not a boolean".to_string()))
}
pub fn get_float(x: &Exp) -> Result<f64, Exceptions> {
    x.to_f64()
        .ok_or(Exceptions::ValueError("Not a number".to_string()))
}
pub fn get_int(x: &Exp) -> Result<i64, Exceptions> {
    x.to_i64()
        .ok_or(Exceptions::ValueError("Not a number".to_string()))
}

impl fmt::Display for Exp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s: String = match self {
            Exp::Atom(x) => match x {
                Atom::Symbol(y) => y.clone(),
                Atom::Number(Number::Int(y)) => y.to_string(),
                Atom::Number(Number::Float(y)) => y.to_string(),
                Atom::Bool(x) => match x {
                    true => "#t".to_string(),
                    false => "#f".to_string(),
                },
            },
            Exp::List(x) => {
                let str_form: Vec<String> =
                    x.iter().map(|a| format!("{}", a).to_string()).collect();
                str_form.join(" ")
            }
            Exp::Func(_) => "Func".to_string(),
        };
        write!(f, "{}", s)
    }
}
