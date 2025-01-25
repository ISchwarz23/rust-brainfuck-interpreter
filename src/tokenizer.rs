
pub enum Token {
    MovePointerRight,
    MovePointerLeft,
    IncrementCurrentRegister,
    DecrementCurrentRegister,
    LoopStart,
    LoopEnd,
    PrintCurrentRegister,
    ReadToCurrentRegister,
}

#[derive(Debug)]
pub struct TokenizeError {
    symbol: char,
    index: usize,
}

impl TokenizeError {
    pub(crate) fn message(&self) -> String {
        format!("Unrecognized symbol '{}' at index {}.", self.symbol, self.index)
    }
}

pub fn tokenize(code: &str) -> Result<Vec<Token>, TokenizeError> {
    code.chars().into_iter()
        .map(|c| tokenize_one(c))
        .enumerate()
        .try_fold(Vec::new(), |mut acc, (index, token)| {
            if token.is_some() {
                acc.push(token.unwrap());
                Ok(acc)
            } else {
                Err(TokenizeError{ index, symbol: code.chars().nth(index).unwrap() })
            }
        })
}

fn tokenize_one(c: char) -> Option<Token> {
    match c {
        '>' => { Some(Token::MovePointerRight) }
        '<' => { Some(Token::MovePointerLeft) }
        '+' => { Some(Token::IncrementCurrentRegister) }
        '-' => { Some(Token::DecrementCurrentRegister) }
        '[' => { Some(Token::LoopStart) }
        ']' => { Some(Token::LoopEnd) }
        '.' => { Some(Token::PrintCurrentRegister) }
        ',' => { Some(Token::ReadToCurrentRegister) }
        _ => { None }
    }
}
