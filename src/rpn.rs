use crate::error::CalcError;
use crate::parser::Token;

/// Алгоритм сортировочной станции (Shunting-yard)
pub fn to_rpn(tokens: Vec<Token>) -> Result<Vec<Token>, CalcError> {
    let mut output: Vec<Token> = Vec::with_capacity(tokens.len());
    let mut operators: Vec<Token> = Vec::new();

    for token in tokens {
        match token {
            Token::Number(_) => output.push(token),
            Token::LParen | Token::UnaryMinus => operators.push(token),
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

#[cfg(test)]
mod tests_to_rpn {
    use super::*;
    use crate::parser::Token;

    #[test]
    fn test_to_rpn_simple_expression() {
        // 2 + 3
        let tokens = vec![Token::Number(2.0), Token::Plus, Token::Number(3.0)];
        let expected = vec![Token::Number(2.0), Token::Number(3.0), Token::Plus];
        assert_eq!(to_rpn(tokens).unwrap(), expected);
    }

    #[test]
    fn test_to_rpn_complex_expression() {
        // 12.5 - 4.2 * (3 / 7)
        let tokens = vec![
            Token::Number(12.5),
            Token::Minus,
            Token::Number(4.2),
            Token::Multiply,
            Token::LParen,
            Token::Number(3.0),
            Token::Divide,
            Token::Number(7.0),
            Token::RParen,
        ];
        let expected = vec![
            Token::Number(12.5),
            Token::Number(4.2),
            Token::Number(3.0),
            Token::Number(7.0),
            Token::Divide,
            Token::Multiply,
            Token::Minus,
        ];
        assert_eq!(to_rpn(tokens).unwrap(), expected);
    }

    #[test]
    fn test_to_rpn_unmatched_parens() {
        // 1 + (2 * 3)
        let tokens = vec![
            Token::Number(1.0),
            Token::Plus,
            Token::LParen,
            Token::Number(2.0),
            Token::Multiply,
            Token::Number(3.0),
        ];
        assert!(matches!(to_rpn(tokens), Err(CalcError::UnmatchedParens)));

        let tokens = vec![
            Token::Number(1.0),
            Token::Plus,
            Token::Number(2.0),
            Token::RParen,
        ];
        assert!(matches!(to_rpn(tokens), Err(CalcError::UnmatchedParens)));
    }

    #[test]
    fn test_to_rpn_operator_precedence() {
        // 1 + 2 * 3
        let tokens = vec![
            Token::Number(1.0),
            Token::Plus,
            Token::Number(2.0),
            Token::Multiply,
            Token::Number(3.0),
        ];
        let expected = vec![
            Token::Number(1.0),
            Token::Number(2.0),
            Token::Number(3.0),
            Token::Multiply,
            Token::Plus,
        ];
        assert_eq!(to_rpn(tokens).unwrap(), expected);

        // 1 * 2 + 3
        let tokens = vec![
            Token::Number(1.0),
            Token::Multiply,
            Token::Number(2.0),
            Token::Plus,
            Token::Number(3.0),
        ];
        let expected = vec![
            Token::Number(1.0),
            Token::Number(2.0),
            Token::Multiply,
            Token::Number(3.0),
            Token::Plus,
        ];
        assert_eq!(to_rpn(tokens).unwrap(), expected);
    }

    #[test]
    fn test_to_rpn_associativity() {
        // 1 - 2 - 3
        let tokens = vec![
            Token::Number(1.0),
            Token::Minus,
            Token::Number(2.0),
            Token::Minus,
            Token::Number(3.0),
        ];
        let expected = vec![
            Token::Number(1.0),
            Token::Number(2.0),
            Token::Minus,
            Token::Number(3.0),
            Token::Minus,
        ];
        assert_eq!(to_rpn(tokens).unwrap(), expected);
    }

    #[test]
    fn test_to_rpn_negative_number() {
        // - 5
        let tokens = vec![Token::UnaryMinus, Token::Number(5.0)];
        let expected = vec![Token::Number(5.0), Token::UnaryMinus];
        assert_eq!(to_rpn(tokens).unwrap(), expected);
    }

    #[test]
    fn test_to_rpn_negative_number_in_expression() {
        // 2 - (-3)
        let tokens = vec![
            Token::Number(2.0),
            Token::Minus,
            Token::LParen,
            Token::UnaryMinus,
            Token::Number(3.0),
            Token::RParen,
        ];
        let expected = vec![
            Token::Number(2.0),
            Token::Number(3.0),
            Token::UnaryMinus,
            Token::Minus,
        ];
        assert_eq!(to_rpn(tokens).unwrap(), expected);
    }

    #[test]
    fn test_unary_minus_complex() {
        // -(1 + 2) * -3
        let tokens = vec![
            Token::UnaryMinus,
            Token::LParen,
            Token::Number(1.0),
            Token::Plus,
            Token::Number(2.0),
            Token::RParen,
            Token::Multiply,
            Token::UnaryMinus,
            Token::Number(3.0),
        ];
        let expected = vec![
            Token::Number(1.0),
            Token::Number(2.0),
            Token::Plus,
            Token::UnaryMinus,
            Token::Number(3.0),
            Token::UnaryMinus,
            Token::Multiply,
        ];
        assert_eq!(to_rpn(tokens).unwrap(), expected);
    }
}
