#[derive(Debug)]
pub enum Token {
    OpenParen,
    Symbol(String),
    CloseParen
}

pub fn tokenize(s: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut last_beginning: Option<usize> = None;
    for (i, c) in s.char_indices() {
        match c {
            '(' => {
                if let Some(start) = last_beginning {
                    tokens.push(Token::Symbol(s[start..i].to_owned()))
                }
                tokens.push(Token::OpenParen);
            },
            ')' => {
                if let Some(start) = last_beginning {
                    tokens.push(Token::Symbol(s[start..i].to_owned()))
                }
                tokens.push(Token::CloseParen);
            },
            _ => {
                if last_beginning.is_none() {;
                    last_beginning = Some(i);
                }
            }
        }
    }
    tokens
}
