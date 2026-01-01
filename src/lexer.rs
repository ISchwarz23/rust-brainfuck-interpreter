
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
pub struct LexingError {
    symbol: char,
    index: usize,
}

impl LexingError {
    pub(crate) fn message(&self) -> String {
        format!("Unrecognized symbol '{}' at index {}.", self.symbol, self.index)
    }
}

pub fn lex(code: &str) -> Result<Vec<Token>, LexingError> {
    code.chars().into_iter()
        .map(|c| to_token(c))
        .enumerate()
        .try_fold(Vec::new(), |mut acc, (index, token)| {
            if token.is_some() {
                acc.push(token.unwrap());
                Ok(acc)
            } else {
                Err(LexingError { index, symbol: code.chars().nth(index).unwrap() })
            }
        })
}

fn to_token(c: char) -> Option<Token> {
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
