use std::error::Error;
use std::fmt;

// Определяем перечисление для ошибок калькулятора
#[derive(Debug)]
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
            CalcError::InvalidToken(token) => format!("Неверное число: {}", token),
            CalcError::UnmatchedParens => "Не совпадают скобки.".to_owned(),
            CalcError::DivideByZero => "Деление на 0.".to_owned(),
            CalcError::InvalidExpression(expr) => format!("Некорректное выражение: {}", expr),
        };

        write!(f, "Error: {}", message)
    }
}

// Реализуем Error для CalcError для использования в контексте ошибок
impl Error for CalcError {}

// Модуль для тестов
#[cfg(test)]
mod tests {
    use super::CalcError;

    #[test]
    fn test_invalid_token() {
        let error = CalcError::InvalidToken("abc".to_string());
        assert_eq!(format!("{}", error), "Error: Неверное число: abc");
    }

    #[test]
    fn test_unmatched_parens() {
        let error = CalcError::UnmatchedParens;
        assert_eq!(format!("{}", error), "Error: Не совпадают скобки.");
    }
    #[test]
    fn test_divide_by_zero() {
        let error = CalcError::DivideByZero;
        assert_eq!(format!("{}", error), "Error: Деление на 0.");
    }
    #[test]
    fn test_invalid_expression() {
        let error = CalcError::InvalidExpression("1 + 2 *".to_string());
        assert_eq!(
            format!("{}", error),
            "Error: Некорректное выражение: 1 + 2 *"
        );
    }
}
