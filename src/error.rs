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
            CalcError::InvalidToken(token) => format!("Неверное число: {token}"),
            CalcError::UnmatchedParens => "Не совпадают скобки.".to_owned(),
            CalcError::DivideByZero => "Деление на 0.".to_owned(),
            CalcError::InvalidExpression(expr) => format!("Некорректное выражение: {expr}"),
        };

        write!(f, "Error: {message}")
    }
}

// Реализуем Error для CalcError для использования в контексте ошибок
impl Error for CalcError {}
