use std::{
    cmp::Ordering,
    fmt,
    ops::{Add, Div, Mul, Neg, Sub},
};

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Num(f64),
    Str(String),
    Bool(bool),
    Nil,
    ArithmeticError,
    DivisionByZeroError,
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Num(x) => write!(f, "{x}"),
            Self::Str(s) => write!(f, "{s}"),
            Self::Bool(b) => write!(f, "{b}"),
            Self::Nil => write!(f, "nil"),
            Self::ArithmeticError => write!(f, "ArithmeticError"),
            Self::DivisionByZeroError => write!(f, "DivisionByZeroError"),
        }
    }
}

impl Neg for Object {
    type Output = Self;
    fn neg(self) -> Self::Output {
        if let Object::Num(x) = self {
            Object::Num(-x)
        } else {
            Object::ArithmeticError
        }
    }
}

impl Add for Object {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Object::Num(left), Object::Num(right)) => Object::Num(left + right),
            (Object::Str(left), Object::Str(right)) => Object::Str(format!("{left}{right}")),
            (Object::Str(left), Object::Num(right)) => Object::Str(format!("{left}{right}")),
            (Object::Num(left), Object::Str(right)) => Object::Str(format!("{left}{right}")),
            _ => Object::ArithmeticError,
        }
    }
}

impl Sub for Object {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Object::Num(left), Object::Num(right)) => Object::Num(left - right),
            _ => Object::ArithmeticError,
        }
    }
}

impl Mul for Object {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Object::Num(left), Object::Num(right)) => Object::Num(left * right),
            _ => Self::ArithmeticError,
        }
    }
}

impl Div for Object {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Object::Num(_), Object::Num(0.0)) => Object::DivisionByZeroError,
            (Object::Num(left), Object::Num(right)) => Object::Num(left / right),
            _ => Self::ArithmeticError,
        }
    }
}

impl PartialOrd for Object {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Object::Num(left), Object::Num(right)) => Some(left.partial_cmp(right)?),
            _ => None,
        }
    }
}
