use crate::error::CalcError;

#[derive(Debug, PartialEq)]
pub enum Token {
    Number(f64),
    Plus,
    Minus,
    Multiply,
    Divide,
    LParen,
    RParen,
}

impl Token {
    pub fn precedence(&self) -> u8 {
        match self {
            Token::Number(_) => 0,
            Token::LParen | Token::RParen => 1,
            Token::Plus | Token::Minus => 2,
            Token::Multiply | Token::Divide => 3,
        }
    }
}

fn get_token(c: char) -> Result<Token, CalcError> {
    let result = match c {
        '+' => Token::Plus,
        '-' => Token::Minus,
        '*' => Token::Multiply,
        '/' => Token::Divide,
        '(' => Token::LParen,
        ')' => Token::RParen,
        _ => return Err(CalcError::InvalidToken(c.to_string())),
    };

    Ok(result)
}

fn get_fnum(s: &str) -> Result<f64, CalcError> {
    match s.trim().parse::<f64>() {
        Ok(fnum) => Ok(fnum),
        Err(_) => Err(CalcError::InvalidToken(s.to_string())),
    }
}

// Разбивает строку на токены.
// Пример: "2 + 3" → [Token::Number(2.0), Token::Plus, Token::Number(3.0)]
pub fn tokenize(input: &str) -> Result<Vec<Token>, CalcError> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut num_buffer = String::new();

    for c in input.chars() {
        if c.is_ascii_digit() || c == '.' {
            num_buffer.push(c);
            continue;
        } else if !num_buffer.is_empty() {
            let num = get_fnum(&num_buffer)?;
            tokens.push(Token::Number(num));
            num_buffer.clear();
        }

        if c.is_whitespace() {
            continue;
        }

        tokens.push(get_token(c)?);
    }

    if !num_buffer.is_empty() {
        let num = get_fnum(&num_buffer)?;
        tokens.push(Token::Number(num));
    }

    Ok(tokens)
}

/// Проверяет корректность скобок.
pub fn validate_parens(tokens: &[Token]) -> Result<(), CalcError> {
    let mut balance = 0;
    for token in tokens {
        match token {
            Token::LParen => balance += 1,
            Token::RParen => balance -= 1,
            _ => {}
        }

        if balance < 0 {
            return Err(CalcError::UnmatchedParens);
        }
    }

    if balance != 0 {
        return Err(CalcError::UnmatchedParens);
    }

    Ok(())
}

// Модуль для тестов
#[cfg(test)]
mod tests_tokenize {
    use super::*;

    #[test]
    fn test_tokenize_simple_expression() {
        let input = "2 + 3";
        let expected = vec![Token::Number(2.0), Token::Plus, Token::Number(3.0)];
        assert_eq!(tokenize(input).unwrap(), expected);
    }

    #[test]
    fn test_tokenize_complex_expression() {
        let input = "12.5 - 4.2 * (3 / 7)";
        let expected = vec![
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
        assert_eq!(tokenize(input).unwrap(), expected);
    }

    #[test]
    fn test_tokenize_with_whitespace() {
        let input = "  2   +   3  ";
        let expected = vec![Token::Number(2.0), Token::Plus, Token::Number(3.0)];
        assert_eq!(tokenize(input).unwrap(), expected);
    }

    #[test]
    fn test_tokenize_invalid_token() {
        let input = "2 + a";
        assert!(matches!(tokenize(input), Err(CalcError::InvalidToken(_))));
    }

    #[test]
    fn test_tokenize_invalid_number() {
        let input = "2 + .";
        assert!(matches!(tokenize(input), Err(CalcError::InvalidToken(_))));
    }

    #[test]
    fn test_tokenize_empty_input() {
        let input = "";
        let expected: Vec<Token> = vec![];
        assert_eq!(tokenize(input).unwrap(), expected);
    }

    #[test]
    fn test_tokenize_single_number() {
        let input = "42";
        let expected = vec![Token::Number(42.0)];
        assert_eq!(tokenize(input).unwrap(), expected);
    }

    #[test]
    fn test_tokenize_single_operator() {
        let input = "+";
        let expected = vec![Token::Plus];
        assert_eq!(tokenize(input).unwrap(), expected);
    }

    #[test]
    fn test_tokenize_mixed_tokens() {
        let input = "1 + -2 * (3 / 4)";
        let expected = vec![
            Token::Number(1.0),
            Token::Plus,
            Token::Minus,
            Token::Number(2.0),
            Token::Multiply,
            Token::LParen,
            Token::Number(3.0),
            Token::Divide,
            Token::Number(4.0),
            Token::RParen,
        ];
        assert_eq!(tokenize(input).unwrap(), expected);
    }

    #[test]
    fn test_tokenize_negative_number() {
        let input = "-5";
        let expected = vec![Token::Minus, Token::Number(5.0)];
        assert_eq!(tokenize(input).unwrap(), expected);
    }

    #[test]
    fn test_tokenize_number_with_leading_dot() {
        let input = ".5";
        let expected = vec![Token::Number(0.5)];
        assert_eq!(tokenize(input).unwrap(), expected);
    }

    #[test]
    fn test_tokenize_number_with_trailing_dot() {
        let input = "5.";
        let expected = vec![Token::Number(5.0)];
        assert_eq!(tokenize(input).unwrap(), expected);
    }

    #[test]
    fn test_tokenize_number_with_multiple_dots() {
        let input = "1.2.3";
        assert!(matches!(tokenize(input), Err(CalcError::InvalidToken(_))));
    }

    #[test]
    fn test_tokenize_number_with_invalid_character() {
        let input = "1a2";
        assert!(matches!(tokenize(input), Err(CalcError::InvalidToken(_))));
    }

    #[test]
    fn test_tokenize_chained_operators() {
        let input = "1 + - * /";
        let expected = vec![
            Token::Number(1.0),
            Token::Plus,
            Token::Minus,
            Token::Multiply,
            Token::Divide,
        ];
        assert_eq!(tokenize(input).unwrap(), expected);
    }

    #[test]
    fn test_tokenize_parentheses_only() {
        let input = "()";
        let expected = vec![Token::LParen, Token::RParen];
        assert_eq!(tokenize(input).unwrap(), expected);
    }

    #[test]
    fn test_tokenize_nested_parentheses() {
        let input = "(1 + (2 * 3))";
        let expected = vec![
            Token::LParen,
            Token::Number(1.0),
            Token::Plus,
            Token::LParen,
            Token::Number(2.0),
            Token::Multiply,
            Token::Number(3.0),
            Token::RParen,
            Token::RParen,
        ];
        assert_eq!(tokenize(input).unwrap(), expected);
    }

    #[test]
    fn test_tokenize_large_numbers() {
        let input = "1234567890.1234567890";
        let expected = vec![Token::Number(1234567890.1234567)];
        assert_eq!(tokenize(input).unwrap(), expected);
    }

    #[test]
    fn test_tokenize_scientific_notation() {
        let input = "1e10";
        assert!(matches!(tokenize(input), Err(CalcError::InvalidToken(_))));
    }
}

// Тесты для precedence
#[cfg(test)]
mod tests_precedence {
    use super::*;

    #[test]
    fn test_precedence_number() {
        let number = Token::Number(1.0);
        assert_eq!(number.precedence(), 0);
    }

    #[test]
    fn test_precedence_parentheses() {
        let lparen = Token::LParen;
        let rparen = Token::RParen;
        assert_eq!(lparen.precedence(), 1);
        assert_eq!(rparen.precedence(), 1);
    }

    #[test]
    fn test_precedence_plus_minus() {
        let plus = Token::Plus;
        let minus = Token::Minus;
        assert_eq!(plus.precedence(), 2);
        assert_eq!(minus.precedence(), 2);
    }

    #[test]
    fn test_precedence_multiply_divide() {
        let multiply = Token::Multiply;
        let divide = Token::Divide;
        assert_eq!(multiply.precedence(), 3);
        assert_eq!(divide.precedence(), 3);
    }

    #[test]
    fn test_precedence_comparison() {
        let number = Token::Number(1.0);
        let plus = Token::Plus;
        let multiply = Token::Multiply;
        let lparen = Token::LParen;
        let rparen = Token::RParen;

        assert!(number.precedence() < plus.precedence());
        assert!(number.precedence() < multiply.precedence());
        assert!(number.precedence() < lparen.precedence());
        assert!(number.precedence() < rparen.precedence());

        assert!(plus.precedence() > number.precedence());
        assert!(plus.precedence() < multiply.precedence());
        assert!(plus.precedence() > lparen.precedence());
        assert!(plus.precedence() > rparen.precedence());

        assert!(multiply.precedence() > number.precedence());
        assert!(multiply.precedence() > plus.precedence());
        assert!(multiply.precedence() > lparen.precedence());
        assert!(multiply.precedence() > rparen.precedence());

        assert!(lparen.precedence() > number.precedence());
        assert!(lparen.precedence() < multiply.precedence());
        assert!(lparen.precedence() < plus.precedence());
        assert_eq!(lparen.precedence(), rparen.precedence());

        assert!(rparen.precedence() > number.precedence());
        assert!(rparen.precedence() < multiply.precedence());
        assert!(rparen.precedence() < plus.precedence());
        assert_eq!(rparen.precedence(), lparen.precedence());
    }
}

// Тесты для validate_parens
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_parens_valid_simple() {
        let tokens = vec![Token::Number(1.0), Token::Plus, Token::Number(2.0)];
        assert_eq!(validate_parens(&tokens), Ok(()));
    }

    #[test]
    fn test_validate_parens_valid_nested() {
        let tokens = vec![
            Token::LParen,
            Token::Number(1.0),
            Token::Plus,
            Token::LParen,
            Token::Number(2.0),
            Token::Multiply,
            Token::Number(3.0),
            Token::RParen,
            Token::RParen,
        ];
        assert_eq!(validate_parens(&tokens), Ok(()));
    }

    #[test]
    fn test_validate_parens_valid_mixed() {
        let tokens = vec![
            Token::Number(1.0),
            Token::Plus,
            Token::LParen,
            Token::Number(2.0),
            Token::Minus,
            Token::Number(3.0),
            Token::RParen,
            Token::Multiply,
            Token::Number(4.0),
        ];
        assert_eq!(validate_parens(&tokens), Ok(()));
    }

    #[test]
    fn test_validate_parens_invalid_unmatched_open() {
        let tokens = vec![
            Token::LParen,
            Token::Number(1.0),
            Token::Plus,
            Token::Number(2.0),
        ];
        assert!(matches!(
            validate_parens(&tokens),
            Err(CalcError::UnmatchedParens)
        ));
    }

    #[test]
    fn test_validate_parens_invalid_unmatched_close() {
        let tokens = vec![
            Token::Number(1.0),
            Token::Plus,
            Token::Number(2.0),
            Token::RParen,
        ];
        assert!(matches!(
            validate_parens(&tokens),
            Err(CalcError::UnmatchedParens)
        ));
    }

    #[test]
    fn test_validate_parens_invalid_extra_open() {
        let tokens = vec![
            Token::LParen,
            Token::LParen,
            Token::Number(1.0),
            Token::Plus,
            Token::Number(2.0),
            Token::RParen,
        ];
        assert!(matches!(
            validate_parens(&tokens),
            Err(CalcError::UnmatchedParens)
        ));
    }

    #[test]
    fn test_validate_parens_invalid_extra_close() {
        let tokens = vec![
            Token::Number(1.0),
            Token::Plus,
            Token::Number(2.0),
            Token::RParen,
            Token::RParen,
        ];
        assert!(matches!(
            validate_parens(&tokens),
            Err(CalcError::UnmatchedParens)
        ));
    }

    #[test]
    fn test_validate_parens_invalid_mismatched_order() {
        let tokens = vec![
            Token::RParen,
            Token::Number(1.0),
            Token::Plus,
            Token::Number(2.0),
            Token::LParen,
        ];
        assert!(matches!(
            validate_parens(&tokens),
            Err(CalcError::UnmatchedParens)
        ));
    }

    #[test]
    fn test_validate_parens_invalid_empty_parens() {
        let tokens = vec![Token::LParen, Token::RParen, Token::Number(1.0)];
        assert_eq!(validate_parens(&tokens), Ok(()));
    }

    #[test]
    fn test_validate_parens_invalid_consecutive_parens() {
        let tokens = vec![Token::LParen, Token::RParen, Token::LParen, Token::RParen];
        assert_eq!(validate_parens(&tokens), Ok(()));
    }

    #[test]
    fn test_validate_parens_invalid_unmatched_nested() {
        let tokens = vec![
            Token::LParen,
            Token::Number(1.0),
            Token::Plus,
            Token::LParen,
            Token::Number(2.0),
            Token::Multiply,
            Token::Number(3.0),
            Token::RParen,
        ];
        assert!(matches!(
            validate_parens(&tokens),
            Err(CalcError::UnmatchedParens)
        ));
    }
}
