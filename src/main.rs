extern crate rustyline;

use rustyline::error::ReadlineError;
use rustyline::Editor;

mod tokenizer;
use tokenizer::tokenize;

mod ast;
use ast::Exprs;
mod parser;
use parser::parse;

fn main() {
    let mut rl = Editor::<()>::new();
    loop {
        let readline = rl.readline(":=> ");
        match readline {
            Ok(line) => {
                let tokens = tokenize(&line);
                println!("Tokens: {:?}", tokens);
                let ast = parse(&tokens);
                println!("AST: {:?}", ast);
                println!("Pretty AST: {}", Exprs(ast));
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
