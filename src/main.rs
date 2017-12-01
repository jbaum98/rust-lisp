extern crate rustyline;

use rustyline::error::ReadlineError;
use rustyline::Editor;

mod lisp;
mod tokenizer;
use tokenizer::tokenize;

fn main() {
    let mut rl = Editor::<()>::new();
    loop {
        let readline = rl.readline(":=> ");
        match readline {
            Ok(line) => {
                println!("Line: {:?}", tokenize(&line));
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
