use std::fmt::{Display, Formatter, Error};
extern crate itertools;
use self::itertools::Itertools;

#[derive(Debug)]
pub enum Ast {
    Atom(String),
    Int(i32),
    Str(String),
    List(Vec<Ast>),
}

impl Display for Ast {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Ast::*;
        match *self {
            Atom(ref s) => write!(fmt, "{}", s),
            Int(ref i) => write!(fmt, "{}", i),
            Str(ref s) => write!(fmt, "\"{}\"", s),
            List(ref xs) =>
               write!(fmt, "({})", xs.iter().format(" ")),
        }
    }
}

pub struct DispAst(pub Vec<Ast>);

impl Display for DispAst {
    fn fmt(&self, fmt: &mut Formatter) -> Result <(), Error> {
        write!(fmt, "[{}]", self.0.iter().format(", "))
    }
}
