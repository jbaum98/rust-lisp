extern crate rustyline;

use rustyline::error::ReadlineError;
use rustyline::{Cmd, Config, CompletionType, Editor, EditMode, KeyPress};

mod ast;
mod lisp;
use lisp::parse_Exprs;

fn main() {
    let mut rl = Editor::<()>::new();
    rl.bind_sequence(KeyPress::Up, Cmd::PreviousHistory);
    rl.bind_sequence(KeyPress::Down, Cmd::NextHistory);
    loop {
        let readline = rl.readline(":=> ");
        match readline {
            Ok(line) => {

                let exprs = parse_Exprs(&line).unwrap();
                println!("AST: {:?}", exprs);
                println!("Pretty: {}", ast::DispAst(exprs));

                rl.add_history_entry(line);
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
