use crate::lexer::Token;

#[derive(Debug)]
pub enum Expression {
    MovePointer(isize),
    ModifyRegister(isize),
    Loop(Vec<Expression>),
    PrintRegister,
    ReadToRegister,
}

#[derive(Debug)]
pub enum ParseError {
    UnmatchedBrackets {
        no_opening_brackets: usize,
        no_closing_brackets: usize,
    },
}

impl ParseError {
    pub(crate) fn message(&self) -> String {
        match self {
            ParseError::UnmatchedBrackets { no_opening_brackets, no_closing_brackets } => {
                format!(
                    "Unmatched brackets. {} opening and {} closing",
                    no_opening_brackets, no_closing_brackets
                )
            }
        }
    }
}

pub fn parse(tokens: &Vec<Token>) -> Result<Vec<Expression>, ParseError> {
    parse_internal(tokens, 0).map(|(exprs, _)| exprs)
}

fn parse_internal(
    tokens: &Vec<Token>,
    start_index: usize,
) -> Result<(Vec<Expression>, usize), ParseError> {
    let loop_start_count = tokens
        .iter()
        .filter(|t| matches!(t, Token::LoopStart))
        .count();
    let loop_end_count = tokens
        .iter()
        .filter(|t| matches!(t, Token::LoopEnd))
        .count();

    if loop_start_count != loop_end_count {
        return Err(ParseError::UnmatchedBrackets {
            no_opening_brackets: loop_start_count,
            no_closing_brackets: loop_end_count,
        });
    };

    let mut expressions: Vec<Expression> = Vec::new();
    let mut index = start_index;

    while index < tokens.len() {
        let current_token = &tokens[index];
        match current_token {
            Token::MovePointerRight => {
                expressions.push(Expression::MovePointer(1));
            }
            Token::MovePointerLeft => {
                expressions.push(Expression::MovePointer(-1));
            }
            Token::IncrementCurrentRegister => {
                expressions.push(Expression::ModifyRegister(1));
            }
            Token::DecrementCurrentRegister => {
                expressions.push(Expression::ModifyRegister(-1));
            }
            Token::LoopStart => {
                let result = parse_internal(&tokens, index + 1);
                if result.is_err() {
                    return result;
                } else {
                    let (parsed_expressions, end_index) = result.ok().unwrap();
                    expressions.push(Expression::Loop(parsed_expressions));
                    index = end_index;
                }
            }
            Token::LoopEnd => return Ok((expressions, index)),
            Token::PrintCurrentRegister => {
                expressions.push(Expression::PrintRegister);
            }
            Token::ReadToCurrentRegister => {
                expressions.push(Expression::ReadToRegister);
            }
        }
        index += 1;
    }

    Ok((expressions, index))
}
