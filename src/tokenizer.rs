#[derive(Debug)]
pub enum Token {
    OpenParen,
    Symbol(String),
    Str(String),
    CloseParen
}

enum State {
    Default,
    InSymbol(String),
    InString(String),
}

impl State {
    pub fn handle_state (self, c: char, tokens: &mut Vec<Token>) -> Self {
        use self::Token::*;
        match self {
            State::Default => {
                match c {
                    '(' => {
                        tokens.push(OpenParen);
                        State::Default
                    },
                    ')' => {
                        tokens.push(CloseParen);
                        State::Default
                    },
                    '"' => {
                        State::InString(String::new())
                    },
                    ' ' => {
                        State::Default
                    }
                    _ => {
                        let mut s = String::new();
                        s.push(c);
                        State::InSymbol(s)
                    },
                }
            },

            State::InSymbol(mut s) => {
                match c {
                    ' ' => {
                       tokens.push(Symbol(s));
                       State::Default
                    },

                    '(' => {
                       tokens.push(Symbol(s));
                       tokens.push(OpenParen);
                       State::Default
                    }

                    ')' => {
                       tokens.push(Symbol(s));
                       tokens.push(CloseParen);
                       State::Default
                    }

                    _ => {
                        s.push(c);
                        State::InSymbol(s)
                    },
                }
            },

            State::InString(mut s) => {
                match c {
                    '"' => {
                        tokens.push(Str(s));
                        State::Default
                    },

                    _ => {
                        s.push(c);
                        State::InString(s)
                    },
                }
            },
        }
    }
}

pub fn tokenize(s: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut state = State::Default;
    for c in s.chars() {
        state = state.handle_state(c, &mut tokens);
    }
    match state {
        State::InSymbol(s) => tokens.push(Token::Symbol(s)),
        State::InString(s) => tokens.push(Token::Str(s)),
        _ => {}
    }
    tokens
}
