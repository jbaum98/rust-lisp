pub use ast::Expr;
use tokenizer::Token;

#[derive(Debug)]
enum ExprToBe {
    Empty,
    AwaitingArgs(Expr, Vec<Expr>),
}

#[derive(Debug)]
struct State {
    finished: Vec<Expr>,
    in_progress: Vec<ExprToBe>,
}

impl State {
    fn push_expr(mut self, e: Expr) -> Self {
        match self.in_progress.last_mut() {
            None => {
                self.finished.push(e);
            },
            Some(last) => match last {
                &mut ExprToBe::Empty => {
                    *last = ExprToBe::AwaitingArgs(e, Vec::new());
                },
                &mut ExprToBe::AwaitingArgs(_, ref mut args) => {
                    args.push(e);
                },
            }
        }
        self
    }

    fn handle_token(mut self, t: &Token) -> Self {
        match *t {
            Token::OpenParen => {
                self.in_progress.push(ExprToBe::Empty);
                self
            },
            Token::CloseParen => {
                match self.in_progress.pop().unwrap() {
                    ExprToBe::Empty => {
                        self.push_expr(Expr::NullList)
                    },
                    ExprToBe::AwaitingArgs(f, args) => {
                        self.push_expr(Expr::Apply(Box::new(f), args))
                    }
                }
            },
            Token::Symbol(ref s) => {
                let s = Expr::Symbol(s.clone());
                self.push_expr(s)
            },
            Token::Str(ref s) => {
                let s = Expr::Str(s.clone());
                self.push_expr(s)
            },
        }
    }
}

pub fn parse(tokens: &[Token]) -> Vec<Expr> {
    let mut state = State { finished: Vec::new(), in_progress: Vec::new() };
    for t in tokens {
        state = state.handle_token(t);
    }
    if !state.in_progress.is_empty() {
        panic!("AST Invalid: {:?}", state)
    }
    state.finished
}
