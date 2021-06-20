use std::collections::HashMap;
use std::fmt;

pub type Symbol = String;
pub type Env = HashMap<String, Exp>;

#[derive(Debug, Clone)]
pub enum Exceptions {
    ValueError(String),
    SyntaxError(String),
}
#[derive(Debug, Clone, PartialEq)]
pub enum Number {
    Int(i64),
    Float(f64),
}
#[derive(Debug, Clone, PartialEq)]
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
    Procedure((Vec<String>, Box<Exp>)),
}

// credits : https://www.reddit.com/r/rust/comments/3vchld/how_to_check_if_two_borrowed_objects_are_the_same/
fn _is_same_object<T>(a: &T, b: &T) -> bool {
    a as *const T == b as *const T
}
impl PartialEq for Exp {
    fn eq(&self, other: &Self) -> bool {
        match &self {
            Exp::Func(x) => {
                if let Exp::Func(other_at) = &other {
                    return (*other_at as usize) == (*x as usize);
                } else {
                    return false;
                }
            }
            Exp::Procedure(p) => {
                if let Exp::Procedure(other_at) = &other {
                    return p == other_at;
                } else {
                    return false;
                }
            }
            Exp::Atom(at) => {
                if let Exp::Atom(other_at) = other {
                    return at == other_at;
                } else {
                    return false;
                }
            }
            Exp::List(lst) => {
                if let Exp::List(other_lst) = other {
                    return lst == other_lst;
                } else {
                    return false;
                }
            }
        }
    }
}

pub struct Environment {
    maps: Vec<Env>,
}
impl Environment {
    pub fn new() -> Self {
        Environment {
            maps: vec![HashMap::new()],
        }
    }

    pub fn insert(&mut self, key: String, val: Exp) -> Option<Exp> {
        self.maps.last_mut().unwrap().insert(key, val)
    }

    pub fn get(&self, key: &String) -> Option<&Exp> {
        let i = &self.maps.len();
        for map_index in (0..(*i)).rev() {
            if let Some(x) = self.maps[map_index].get(key) {
                return Some(x);
            } else {
                continue;
            }
        }
        return None;
    }

    pub fn push_stack_frame(&mut self, new_frame_data: Env) {
        self.maps.push(new_frame_data);
    }
    pub fn pop_stack_frame(&mut self) {
        self.maps.pop();
    }
}

pub trait ToFloat {
    fn to_f64(&self) -> Option<f64>;
}

impl ToFloat for Exp {
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
pub trait ToInt {
    fn to_i64(&self) -> Option<i64>;
}
impl ToInt for Exp {
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
pub trait ToBool {
    fn to_bool(&self) -> Option<bool>;
}

impl ToBool for Exp {
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
                "(".to_string() + &str_form.join(" ") + ")"
            }
            Exp::Func(_) => "Func".to_string(),
            Exp::Procedure(_) => "Proc".to_string(),
        };
        write!(f, "{}", s)
    }
}
