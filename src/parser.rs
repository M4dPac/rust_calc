use crate::error::CalcError;

pub enum Token {
    Number(f64),
    Plus,
    Minus,
    Divide,
    LParen,
    RParen,
}

// Разбивает строку на токены.
// Пример: "2 + 3" → [Token::Number(2.0), Token::Plus, Token::Number(3.0)]
pub fn tokenize(input: &str) -> Result<Vec<Token>, CalcError> {}
