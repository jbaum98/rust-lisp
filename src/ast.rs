use std::fmt::{Display, Formatter, Error};
extern crate itertools;
use self::itertools::Itertools;

#[derive(Debug)]
pub enum Expr {
    Symbol(String),
    Str(String),
    Apply(Box<Expr>, Vec<Expr>),
    NullList,
}

impl Display for Expr {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Expr::*;
        match *self {
            Symbol(ref s) => write!(fmt, "{}", s),
            Str(ref s) => write!(fmt, "\"{}\"", s),
            Apply(ref head, ref tail) =>
               write!(fmt, "({} {})", head, tail.iter().format(" ")),
            NullList => write!(fmt, "()"),
        }
    }
}

pub struct Exprs(pub Vec<Expr>);

impl Display for Exprs {
    fn fmt(&self, fmt: &mut Formatter) -> Result <(), Error> {
        write!(fmt, "{}", self.0.iter().format(" "))
    }
}
