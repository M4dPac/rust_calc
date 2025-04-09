use crate::error::CalcError;
use crate::parser::Token;

/// Алгоритм сортировочной станции (Shunting-yard)
pub fn to_rpn(tokens: Vec<Token>) -> Result<Vec<Token>, CalcError> {
    let mut output: Vec<Token> = Vec::with_capacity(tokens.len());
    let mut operators: Vec<Token> = Vec::new();

    for token in tokens {
        match token {
            Token::Number(_) => output.push(token),
            Token::LParen => operators.push(token),
            Token::RParen => {
                while let Some(top) = operators.pop() {
                    match top {
                        Token::LParen => break,
                        _ => output.push(top),
                    }

                    // Проверяем на наличие непарных скобок
                    if operators.is_empty() {
                        return Err(CalcError::UnmatchedParens);
                    }
                }
            }
            Token::Plus | Token::Minus | Token::Multiply | Token::Divide => {
                while let Some(top) = operators.last() {
                    if top.precedence() >= token.precedence() {
                        output.push(operators.pop().unwrap());
                    } else {
                        break;
                    }
                }
                operators.push(token);
            } // _ => return Err(CalcError::InvalidToken(format!("{:?}", token))),
        }
    }

    // Переносим оставшиеся операторы в выходную очередь
    while let Some(op) = operators.pop() {
        if op == Token::LParen {
            return Err(CalcError::UnmatchedParens);
        }
        output.push(op);
    }

    Ok(output)
}
