use crate::error::CalcError;

pub enum Token {
    Number(f64),
    Plus,
    Minus,
    Multiply,
    Divide,
    LParen,
    RParen,
}

// Разбивает строку на токены.
// Пример: "2 + 3" → [Token::Number(2.0), Token::Plus, Token::Number(3.0)]
pub fn tokenize(input: &str) -> Result<Vec<Token>, CalcError> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut num_buffer = String::new();

    for c in input.chars() {
        if c.is_whitespace() {
            continue;
        } else if c.is_ascii_digit() || c == '.' {
            num_buffer.push(c);
        } else if !num_buffer.is_empty() {
            let num = match num_buffer.parse::<f64>() {
                Ok(n) => n,
                Err(_) => return Err(CalcError::InvalidToken(num_buffer)),
            };
            tokens.push(Token::Number(num));
        } else {
            let token = match c {
                '+' => Token::Plus,
                '-' => Token::Minus,
                '*' => Token::Multiply,
                '/' => Token::Divide,
                '(' => Token::LParen,
                ')' => Token::RParen,
                _ => return Err(CalcError::InvalidToken(c.to_string())),
            };

            tokens.push(token);
        }
    }

    Ok(tokens)
}
