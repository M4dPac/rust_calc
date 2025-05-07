use std::error::Error;
use std::fmt;

// Определяем перечисление для ошибок калькулятора
#[derive(Debug, PartialEq)]
pub enum CalcError {
    InvalidToken(String),
    UnmatchedParens,
    DivideByZero,
    InvalidExpression(String),
}

// Реализуем Display для CalcError для удобного вывода ошибок
impl fmt::Display for CalcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            CalcError::InvalidToken(token) => format!("Некорректный символ: {}", token),
            CalcError::UnmatchedParens => "Не совпадают скобки.".to_owned(),
            CalcError::DivideByZero => "Деление на 0.".to_owned(),
            CalcError::InvalidExpression(expr) => format!("Некорректное выражение: {}", expr),
        };

        write!(f, "{}", message)
    }
}

// Реализуем Error для CalcError для использования в контексте ошибок
impl Error for CalcError {}

// Модуль для тестов
#[cfg(test)]
mod tests {
    use super::CalcError;

    #[test]
    fn test_calcerror_invalid_token() {
        let error = CalcError::InvalidToken("abc".to_string());
        assert_eq!(format!("{}", error), "Некорректный символ: abc");
    }

    #[test]
    fn test_calcerror_unmatched_parens() {
        let error = CalcError::UnmatchedParens;
        assert_eq!(format!("{}", error), "Не совпадают скобки.");
    }
    #[test]
    fn test_calcerror_divide_by_zero() {
        let error = CalcError::DivideByZero;
        assert_eq!(format!("{}", error), "Деление на 0.");
    }
    #[test]
    fn test_calcerror_invalid_expression() {
        let error = CalcError::InvalidExpression("1 + 2 *".to_string());
        assert_eq!(format!("{}", error), "Некорректное выражение: 1 + 2 *");
    }
}
