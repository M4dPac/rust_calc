use std::collections::VecDeque;

use crate::error::CalcError;
use crate::parser::Token;

/// Алгоритм сортировочной станции (Shunting-yard)
pub fn to_rpn(tokens: Vec<Token>) -> Result<VecDeque<Token>, CalcError> {
    let mut output: VecDeque<Token> = VecDeque::with_capacity(tokens.len());
    let mut operators: Vec<Token> = Vec::new();

    for token in tokens {
        match token {
            Token::Number(_) => output.push_back(token),
            Token::LParen | Token::UnaryMinus => operators.push(token),
            Token::RParen => {
                while let Some(top) = operators.pop() {
                    match top {
                        Token::LParen => break,
                        _ => output.push_back(top),
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
                        output.push_back(operators.pop().unwrap());
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
        output.push_back(op);
    }

    Ok(output)
}

/// Вычисляет результат ОПЗ.
pub fn eval_rpn(mut rpn: VecDeque<Token>) -> Result<f64, CalcError> {
    let mut stack: Vec<f64> = Vec::new();

    while let Some(token) = rpn.pop_front() {
        match token {
            Token::Number(num) => stack.push(num),
            Token::UnaryMinus => {
                let Some(x) = stack.pop() else {
                    return Err(CalcError::InvalidExpression("".to_string()));
                };

                stack.push(-x);
            }
            _ => {
                let (Some(b), Some(a)) = (stack.pop(), stack.pop()) else {
                    return Err(CalcError::InvalidExpression("".to_string()));
                };

                stack.push(match token {
                    Token::Plus => a + b,
                    Token::Minus => a - b,
                    Token::Multiply => a * b,
                    Token::Divide => {
                        if b == 0.0 {
                            return Err(CalcError::DivideByZero);
                        }
                        a / b
                    }
                    _ => return Err(CalcError::InvalidToken("".to_string())),
                });
            }
        }
    }

    match (stack.pop(), stack.is_empty()) {
        (Some(result), true) => Ok(result),
        (_, _) => Err(CalcError::InvalidExpression("".to_string())),
    }
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

#[cfg(test)]
mod tests_eval_rpn {
    use super::*;
    use crate::parser::Token;
    use std::collections::VecDeque;

    #[test]
    fn test_eval_rpn_simple_expression() {
        // Проверка простого сложения: 2 + 3
        let tokens: VecDeque<Token> = vec![Token::Number(2.0), Token::Number(3.0), Token::Plus]
            .into_iter()
            .collect();
        assert_eq!(eval_rpn(tokens).unwrap(), 5.0);
    }

    #[test]
    fn test_eval_rpn_complex_expression() {
        // Проверка сложного выражения: 12.5 - 4.2 * (3 / 7)
        let tokens: VecDeque<Token> = vec![
            Token::Number(12.5),
            Token::Number(4.2),
            Token::Number(3.0),
            Token::Number(7.0),
            Token::Divide,
            Token::Multiply,
            Token::Minus,
        ]
        .into_iter()
        .collect();
        assert_eq!(eval_rpn(tokens).unwrap(), 10.7);
    }

    #[test]
    fn test_eval_rpn_unary_minus() {
        // Проверка отрицательного числа: -5
        let tokens: VecDeque<Token> = vec![Token::Number(5.0), Token::UnaryMinus]
            .into_iter()
            .collect();
        assert_eq!(eval_rpn(tokens).unwrap(), -5.0);
    }

    #[test]
    fn test_eval_rpn_unary_minus_in_expression() {
        // Проверка унарного минуса внутри выражения: 2 - (-3)
        let tokens: VecDeque<Token> = vec![
            Token::Number(2.0),
            Token::Number(3.0),
            Token::UnaryMinus,
            Token::Minus,
        ]
        .into_iter()
        .collect();
        assert_eq!(eval_rpn(tokens).unwrap(), 5.0);
    }

    #[test]
    fn test_eval_rpn_operator_precedence() {
        // Проверка приоритета операторов: 1 + 2 * 3
        let tokens: VecDeque<Token> = vec![
            Token::Number(1.0),
            Token::Number(2.0),
            Token::Number(3.0),
            Token::Multiply,
            Token::Plus,
        ]
        .into_iter()
        .collect();
        assert_eq!(eval_rpn(tokens).unwrap(), 7.0);

        // Проверка приоритета операторов: 1 * 2 + 3
        let tokens: VecDeque<Token> = vec![
            Token::Number(1.0),
            Token::Number(2.0),
            Token::Multiply,
            Token::Number(3.0),
            Token::Plus,
        ]
        .into_iter()
        .collect();
        assert_eq!(eval_rpn(tokens).unwrap(), 5.0);
    }

    #[test]
    fn test_eval_rpn_associativity() {
        // Проверка ассоциативности операторов: 1 - 2 - 3
        let tokens: VecDeque<Token> = vec![
            Token::Number(1.0),
            Token::Number(2.0),
            Token::Minus,
            Token::Number(3.0),
            Token::Minus,
        ]
        .into_iter()
        .collect();
        assert_eq!(eval_rpn(tokens).unwrap(), -4.0);
    }

    #[test]
    fn test_eval_rpn_divide_by_zero() {
        // Проверка деления на ноль: 1 / 0
        let tokens: VecDeque<Token> = vec![Token::Number(1.0), Token::Number(0.0), Token::Divide]
            .into_iter()
            .collect();
        assert!(matches!(eval_rpn(tokens), Err(CalcError::DivideByZero)));
    }

    #[test]
    fn test_eval_rpn_invalid_expression() {
        // Проверка некорректного выражения: недостаточно операндов
        let tokens: VecDeque<Token> = vec![Token::Number(1.0), Token::Plus].into_iter().collect();
        assert!(matches!(
            eval_rpn(tokens),
            Err(CalcError::InvalidExpression(_))
        ));

        // Проверка некорректного выражения: некорректное расположение операторов
        let tokens: VecDeque<Token> = vec![Token::Plus, Token::Number(1.0), Token::Number(2.0)]
            .into_iter()
            .collect();
        assert!(matches!(
            eval_rpn(tokens),
            Err(CalcError::InvalidExpression(_))
        ));
    }

    #[test]
    fn test_eval_rpn_invalid_token() {
        // Проверка некорректного токена: скобка в выражении
        let tokens: VecDeque<Token> = vec![Token::Number(1.0), Token::Number(2.0), Token::LParen]
            .into_iter()
            .collect();
        assert!(matches!(eval_rpn(tokens), Err(CalcError::InvalidToken(_))));
    }
}
