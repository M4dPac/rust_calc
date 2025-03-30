pub enum Token {
    Number(f64),
    Plus,
    Minus,
    Divide,
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, E> {}
