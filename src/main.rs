extern crate rustyline;

use rustyline::error::ReadlineError;
use rustyline::Editor;

mod ast;
mod lisp;
use lisp::parse_Exprs;

fn main() {
    let mut rl = Editor::<()>::new();
    loop {
        let readline = rl.readline(":=> ");
        match readline {
            Ok(line) => {
                let exprs = parse_Exprs(&line).unwrap();
                println!("AST: {:?}", exprs);
                println!("Pretty: {}", ast::DispAst(exprs));
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
}
